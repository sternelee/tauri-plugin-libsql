# Tauri Plugin LibSQL

A [Tauri](https://tauri.app) plugin for SQLite database access using [LibSQL](https://libsql.org/), providing local and remote synchronization capabilities.

## Installation

### Rust

```toml
# Cargo.toml
[dependencies]
tauri-plugin-libsql = { git = "https://github.com/yourusername/tauri-plugin-libsql" }
```

### JavaScript

```js
import { connect } from 'tauri-plugin-libsql-api';
```

## Usage

### Rust

```rust
use tauri_plugin_libsql::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_libsql::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript

```js
import { connect, execute, query, sync, close } from 'tauri-plugin-libsql-api';

// Connect to a database
const connectionId = await connect({
  url: 'libsql://your-database.turso.io',
  authToken: 'your-auth-token',
  localPath: 'path/to/local/db.sqlite', // Optional for local-first mode
});

// Execute a SQL statement
const result = await execute({
  connectionId,
  sql: 'CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)',
});

// Insert data
await execute({
  connectionId,
  sql: 'INSERT INTO users (name) VALUES (?)',
  params: [{ type: 'Text', value: 'John Doe' }],
});

// Query data
const queryResult = await query({
  connectionId,
  sql: 'SELECT * FROM users',
});
console.log(queryResult.rows);

// Sync changes with the remote database (if in replica mode)
await sync({ connectionId });

// Close the connection when done
await close({ connectionId });
```

## Features

- Local SQLite database access
- Remote SQLite database access via LibSQL's HTTP protocol
- Local-first with remote syncing capabilities
- Full SQL query support
- Parameterized queries for safe data handling
- Cross-platform support (Windows, macOS, Linux, Android, iOS)

## Mobile Platform Implementation

The plugin also supports mobile platforms through Tauri's mobile plugin system. The implementation provides the same API as the desktop version, allowing you to use the same code across all platforms.

### Android

For Android, the plugin uses the Kotlin API. In your Android project, the plugin automatically integrates with your Tauri setup.

To fully implement the plugin in an actual application, you would need to add the LibSQL Kotlin SDK dependency to your app's `build.gradle` file:

```gradle
dependencies {
    // LibSQL dependencies would go here
    // For example: implementation 'org.libsql:libsql-android:1.0.0'
}
```

### iOS

For iOS, the plugin uses the Swift API. In your iOS project, the plugin automatically integrates with your Tauri setup.

To fully implement the plugin in an actual application, you would need to include the LibSQL Swift SDK in your project, for example via Swift Package Manager.

## License

MIT
