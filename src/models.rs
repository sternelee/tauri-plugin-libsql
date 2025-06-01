use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOptions {
    pub local_path: String,
    pub url: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionId(pub String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOptions {
    pub connection_id: String,
    pub sql: String,
    pub params: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOptions {
    pub connection_id: String,
    pub sql: String,
    pub params: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncOptions {
    pub connection_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOptions {
    pub connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResult {
    pub rows_affected: u64,
    pub last_insert_rowid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl From<libsql::Value> for Value {
    fn from(value: libsql::Value) -> Self {
        match value {
            libsql::Value::Null => Value::Null,
            libsql::Value::Integer(v) => Value::Integer(v),
            libsql::Value::Real(v) => Value::Real(v),
            libsql::Value::Text(v) => Value::Text(v),
            libsql::Value::Blob(v) => Value::Blob(v),
        }
    }
}

impl TryFrom<Value> for libsql::Value {
    type Error = crate::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(libsql::Value::Null),
            Value::Integer(v) => Ok(libsql::Value::Integer(v)),
            Value::Real(v) => Ok(libsql::Value::Real(v)),
            Value::Text(v) => Ok(libsql::Value::Text(v)),
            Value::Blob(v) => Ok(libsql::Value::Blob(v)),
        }
    }
}
