use std::collections::HashMap;
use std::sync::Arc;

use libsql::{Builder, Connection, Database};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Libsql<R>> {
    Ok(Libsql {
        app: app.clone(),
        connections: Arc::new(Mutex::new(HashMap::new())),
        databases: Arc::new(Mutex::new(HashMap::new())),
    })
}

type ConnectionsMap = HashMap<String, Connection>;
type DatabasesMap = HashMap<String, Database>;

/// Access to the libsql APIs.
pub struct Libsql<R: Runtime> {
    app: AppHandle<R>,
    connections: Arc<Mutex<ConnectionsMap>>,
    databases: Arc<Mutex<DatabasesMap>>,
}

impl<R: Runtime> Libsql<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub async fn connect(&self, options: ConnectOptions) -> Result<String> {
        let db = if let Some(url) = options.url {
            if let Some(auth_token) = options.auth_token {
                // Create a remote replica
                Builder::new_remote_replica(options.local_path, url, auth_token)
                    .build()
                    .await?
            } else {
                // Create a local database
                Builder::new_local(options.local_path).build().await?
            }
        } else {
            Builder::new_local(options.local_path).build().await?
        };

        let conn = db.connect()?;
        let id = Uuid::new_v4().to_string();

        {
            let mut databases = self.databases.lock().await;
            databases.insert(id.clone(), db);
        }

        {
            let mut connections = self.connections.lock().await;
            connections.insert(id.clone(), conn);
        }

        Ok(id)
    }

    pub async fn execute(&self, options: ExecuteOptions) -> Result<u64> {
        let connection = {
            let connections = self.connections.lock().await;
            connections
                .get(&options.connection_id)
                .cloned()
                .ok_or_else(|| Error::ConnectionNotFound(options.connection_id.clone()))?
        };

        let params = options.params.unwrap_or_default();
        let params_vec: Result<Vec<libsql::Value>> = params
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                p.try_into()
                    .map_err(|_| Error::InvalidParameter(i, format!("Failed to convert parameter")))
            })
            .collect();
        let params_vec = params_vec?;

        let result = connection.execute(&options.sql, params_vec).await.unwrap();

        Ok(result)
    }

    pub async fn query(&self, options: QueryOptions) -> Result<QueryResult> {
        let connection = {
            let connections = self.connections.lock().await;
            connections
                .get(&options.connection_id)
                .cloned()
                .ok_or_else(|| Error::ConnectionNotFound(options.connection_id.clone()))?
        };

        let params = options.params.unwrap_or_default();
        let params_vec: Result<Vec<libsql::Value>> = params
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                p.try_into()
                    .map_err(|_| Error::InvalidParameter(i, format!("Failed to convert parameter")))
            })
            .collect();
        let params_vec = params_vec?;

        let mut rows = connection.query(&options.sql, params_vec).await?;
        let mut result_rows = Vec::new();
        let mut columns = Vec::new();

        if let Some(row) = rows.next().await? {
            // Get column names from the first row
            columns = (0..row.column_count())
                .filter_map(|i| row.column_name(i))
                .map(|s| s.to_string())
                .collect();

            // Process first row
            let values = (0..row.column_count())
                .map(|i| row.get_value(i).map(Value::from).unwrap_or(Value::Null))
                .collect();
            result_rows.push(values);

            // Process remaining rows
            while let Some(row) = rows.next().await? {
                let values = (0..row.column_count())
                    .map(|i| row.get_value(i).map(Value::from).unwrap_or(Value::Null))
                    .collect();
                result_rows.push(values);
            }
        }

        Ok(QueryResult {
            columns,
            rows: result_rows,
        })
    }

    pub async fn sync(&self, options: SyncOptions) -> Result<()> {
        // Get the database and sync it while holding the lock
        let databases = self.databases.lock().await;
        let database = databases
            .get(&options.connection_id)
            .ok_or_else(|| Error::ConnectionNotFound(options.connection_id.clone()))?;

        // Execute the libsql_sync() function via SQL
        let _ = database.sync().await?;

        Ok(())
    }

    pub async fn close(&self, options: CloseOptions) -> Result<()> {
        {
            let mut connections = self.connections.lock().await;
            connections.remove(&options.connection_id);
        }

        {
            let mut databases = self.databases.lock().await;
            databases.remove(&options.connection_id);
        }

        Ok(())
    }
}
