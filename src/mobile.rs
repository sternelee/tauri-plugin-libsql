use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::{models::*, Result};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_libsql);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Libsql<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("app.tauri", "LibsqlPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_libsql)?;
    Ok(Libsql(handle))
}

/// Access to the libsql APIs.
pub struct Libsql<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Libsql<R> {
    pub fn ping(&self, payload: PingRequest) -> Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }

    pub async fn connect(&self, options: ConnectOptions) -> Result<String> {
        self.0
            .run_mobile_plugin("connect", options)
            .map_err(Into::into)
    }

    pub async fn execute(&self, options: ExecuteOptions) -> Result<ExecuteResult> {
        self.0
            .run_mobile_plugin("execute", options)
            .map_err(Into::into)
    }

    pub async fn query(&self, options: QueryOptions) -> Result<QueryResult> {
        self.0
            .run_mobile_plugin("query", options)
            .map_err(Into::into)
    }

    pub async fn sync(&self, options: SyncOptions) -> Result<()> {
        self.0
            .run_mobile_plugin("sync", options)
            .map_err(Into::into)
    }

    pub async fn close(&self, options: CloseOptions) -> Result<()> {
        self.0
            .run_mobile_plugin("close", options)
            .map_err(Into::into)
    }
}
