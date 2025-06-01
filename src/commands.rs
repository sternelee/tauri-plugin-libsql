use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::LibsqlExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.libsql().ping(payload)
}

#[command]
pub(crate) async fn connect<R: Runtime>(
    app: AppHandle<R>,
    options: ConnectOptions,
) -> Result<ConnectionId> {
    let id = app.libsql().connect(options).await?;
    Ok(ConnectionId(id))
}

#[command]
pub(crate) async fn execute<R: Runtime>(
    app: AppHandle<R>,
    options: ExecuteOptions,
) -> Result<u64> {
    app.libsql().execute(options).await
}

#[command]
pub(crate) async fn query<R: Runtime>(
    app: AppHandle<R>,
    options: QueryOptions,
) -> Result<QueryResult> {
    app.libsql().query(options).await
}

#[command]
pub(crate) async fn sync<R: Runtime>(app: AppHandle<R>, options: SyncOptions) -> Result<()> {
    app.libsql().sync(options).await
}

#[command]
pub(crate) async fn close<R: Runtime>(app: AppHandle<R>, options: CloseOptions) -> Result<()> {
    app.libsql().close(options).await
}
