import { invoke } from '@tauri-apps/api/core'

export interface PingRequest {
  value?: string
}

export interface PingResponse {
  value?: string
}

export interface ConnectOptions {
  url: string
  authToken?: string
  localPath?: string
}

export interface ConnectionId {
  0: string
}

export interface ExecuteOptions {
  connectionId: string
  sql: string
  params?: Value[]
}

export interface QueryOptions {
  connectionId: string
  sql: string
  params?: Value[]
}

export interface SyncOptions {
  connectionId: string
}

export interface CloseOptions {
  connectionId: string
}

export interface QueryResult {
  columns: string[]
  rows: Value[][]
}

export interface ExecuteResult {
  rowsAffected: number
  lastInsertRowid?: number
}

export type Value =
  | { type: 'Null' }
  | { type: 'Integer', value: number }
  | { type: 'Real', value: number }
  | { type: 'Text', value: string }
  | { type: 'Blob', value: number[] }

/**
 * Simple ping to test the plugin
 */
export async function ping(value?: string): Promise<string | null> {
  return await invoke<PingResponse>('plugin:libsql|ping', {
    payload: {
      value,
    },
  }).then((r: PingResponse) => (r.value ? r.value : null));
}

/**
 * Connect to a LibSQL database
 */
export async function connect(options: ConnectOptions): Promise<string> {
  const result = await invoke<ConnectionId>('plugin:libsql|connect', {
    options,
  });
  return result[0];
}

/**
 * Execute a SQL statement that modifies the database
 */
export async function execute(options: ExecuteOptions): Promise<ExecuteResult> {
  return await invoke<ExecuteResult>('plugin:libsql|execute', {
    options,
  });
}

/**
 * Run a SQL query and return the results
 */
export async function query(options: QueryOptions): Promise<QueryResult> {
  return await invoke<QueryResult>('plugin:libsql|query', {
    options,
  });
}

/**
 * Synchronize a local database with its remote counterpart
 */
export async function sync(options: SyncOptions): Promise<void> {
  await invoke<void>('plugin:libsql|sync', {
    options,
  });
}

/**
 * Close a database connection
 */
export async function close(options: CloseOptions): Promise<void> {
  await invoke<void>('plugin:libsql|close', {
    options,
  });
}
