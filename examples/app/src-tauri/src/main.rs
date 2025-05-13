// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_libsql::*;

// 基本命令 - 简单的字符串返回
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to the LibSQL example app!", name)
}

// 使用插件 API 的实用指令
#[tauri::command]
async fn create_demo_db(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 连接到 LibSQL 数据库
    let connection_id = app_handle
        .libsql()
        .connect(ConnectOptions {
            url: "libsql://local".to_string(),
            auth_token: None,
            local_path: Some("./demo.db".to_string()),
        })
        .await
        .map_err(|e| e.to_string())?;

    // 创建测试表
    app_handle
        .libsql()
        .execute(ExecuteOptions {
            connection_id: connection_id.clone(),
            sql: "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL, completed BOOLEAN NOT NULL DEFAULT 0)".to_string(),
            params: None,
        })
        .await
        .map_err(|e| e.to_string())?;

    // 插入示例数据
    app_handle
        .libsql()
        .execute(ExecuteOptions {
            connection_id: connection_id.clone(),
            sql: "INSERT INTO todos (title, completed) VALUES (?, ?)".to_string(),
            params: Some(vec![
                Value::Text("Learn Tauri".to_string()),
                Value::Integer(0),
            ]),
        })
        .await
        .map_err(|e| e.to_string())?;

    app_handle
        .libsql()
        .execute(ExecuteOptions {
            connection_id: connection_id.clone(),
            sql: "INSERT INTO todos (title, completed) VALUES (?, ?)".to_string(),
            params: Some(vec![
                Value::Text("Build LibSQL plugin".to_string()),
                Value::Integer(1),
            ]),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(connection_id)
}

#[tauri::command]
async fn get_todos(app_handle: tauri::AppHandle, connection_id: String) -> Result<QueryResult, String> {
    app_handle
        .libsql()
        .query(QueryOptions {
            connection_id,
            sql: "SELECT * FROM todos".to_string(),
            params: None,
        })
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_todo(
    app_handle: tauri::AppHandle,
    connection_id: String,
    title: String,
) -> Result<ExecuteResult, String> {
    app_handle
        .libsql()
        .execute(ExecuteOptions {
            connection_id,
            sql: "INSERT INTO todos (title, completed) VALUES (?, 0)".to_string(),
            params: Some(vec![Value::Text(title)]),
        })
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_todo(
    app_handle: tauri::AppHandle,
    connection_id: String,
    id: i64,
    completed: bool,
) -> Result<ExecuteResult, String> {
    app_handle
        .libsql()
        .execute(ExecuteOptions {
            connection_id,
            sql: "UPDATE todos SET completed = ? WHERE id = ?".to_string(),
            params: Some(vec![
                Value::Integer(if completed { 1 } else { 0 }),
                Value::Integer(id),
            ]),
        })
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn close_db(app_handle: tauri::AppHandle, connection_id: String) -> Result<(), String> {
    app_handle
        .libsql()
        .close(CloseOptions { connection_id })
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_libsql::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_demo_db,
            get_todos,
            add_todo,
            toggle_todo,
            close_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 