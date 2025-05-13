# Tauri Plugin LibSQL - Mobile Integration Guide

This guide explains how to integrate the LibSQL plugin into your Tauri mobile application.

## Android Setup

### Prerequisites

1. Make sure you have a working Tauri Android application
2. Familiarity with Gradle and Android development

### Integration Steps

1. Add the plugin to your Tauri configuration:

```json
{
  "plugins": {
    "libsql": {
      "android": {
        "package": "app.tauri",
        "class": "LibsqlPlugin"
      }
    }
  }
}
```

2. Setup LibSQL dependencies in your `app/build.gradle` file:

```gradle
dependencies {
    // Add any required libSQL dependencies here 
    // These would be provided by LibSQL's official SDK
}
```

3. In your Android project, make sure the `LibsqlPlugin.kt` file is correctly placed in the `app.tauri` package.

4. If you need to add permissions for SQLite database access, add them to your `AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
```

## iOS Setup

### Prerequisites

1. Make sure you have a working Tauri iOS application
2. Familiarity with Xcode and iOS development

### Integration Steps

1. Add the plugin to your Tauri configuration:

```json
{
  "plugins": {
    "libsql": {
      "ios": {
        "class": "LibsqlPlugin"
      }
    }
  }
}
```

2. Include the LibsqlPlugin.swift file in your Xcode project.

3. If you plan to use a third-party LibSQL SDK for iOS, add it to your project using Swift Package Manager or CocoaPods.

## Common Usage Examples

### Connect to a Database

```typescript
import { connect } from 'tauri-plugin-libsql-api';

// Connect to a remote database
const connectionId = await connect({
  url: 'libsql://your-database.turso.io',
  authToken: 'your-auth-token'
});

// Or connect to a local database
const localConnectionId = await connect({
  localPath: 'path/to/local.db'
});

// Or connect to a local database with remote sync
const replicaConnectionId = await connect({
  url: 'libsql://your-database.turso.io',
  authToken: 'your-auth-token',
  localPath: 'path/to/local.db'
});
```

### Execute SQL Statements

```typescript
import { execute } from 'tauri-plugin-libsql-api';

// Create a table
await execute({
  connectionId,
  sql: 'CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)'
});

// Insert data with parameters
await execute({
  connectionId,
  sql: 'INSERT INTO users (name, email) VALUES (?, ?)',
  params: [
    { type: 'Text', value: 'John Doe' },
    { type: 'Text', value: 'john@example.com' }
  ]
});
```

### Query Data

```typescript
import { query } from 'tauri-plugin-libsql-api';

// Query all users
const result = await query({
  connectionId,
  sql: 'SELECT * FROM users'
});

console.log(result.columns); // ['id', 'name', 'email']
console.log(result.rows);    // [[1, 'John Doe', 'john@example.com'], ...]
```

### Sync Remote Changes

```typescript
import { sync } from 'tauri-plugin-libsql-api';

// Sync local database with remote changes
await sync({
  connectionId: replicaConnectionId
});
```

### Close Connection

```typescript
import { close } from 'tauri-plugin-libsql-api';

// Close the connection
await close({
  connectionId
});
```

## Troubleshooting

### Common Android Issues

- **SQLite lock errors**: Make sure you're not trying to access the database from multiple threads simultaneously.
- **Permission issues**: Ensure your app has the necessary permissions for file access.
- **Crashes on startup**: Verify the plugin class is correctly registered in your Tauri configuration.

### Common iOS Issues

- **Threading issues**: Always use the provided DispatchQueue for background operations.
- **Memory management**: Be cautious with large result sets to avoid memory pressure.
- **Simulator vs Device**: Test on both as file system behaviors can differ. 