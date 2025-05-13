import Foundation
import Tauri
import WebKit

// Request models
struct PingRequest: Decodable {
    let value: String?
}

struct ConnectOptions: Decodable {
    let url: String
    let authToken: String?
    let localPath: String?
}

struct ExecuteOptions: Decodable {
    let connectionId: String
    let sql: String
    let params: [Value]?
}

struct QueryOptions: Decodable {
    let connectionId: String
    let sql: String
    let params: [Value]?
}

struct SyncOptions: Decodable {
    let connectionId: String
}

struct CloseOptions: Decodable {
    let connectionId: String
}

// Value type for parameters
enum ValueType: String, Decodable {
    case Null
    case Integer
    case Real
    case Text
    case Blob
}

struct Value: Decodable {
    let type: String
    let value: AnyCodable?
}

// AnyCodable helper for handling dynamic JSON values
struct AnyCodable: Codable {
    let value: Any
    
    init(value: Any) {
        self.value = value
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if container.decodeNil() {
            self.value = NSNull()
        } else if let bool = try? container.decode(Bool.self) {
            self.value = bool
        } else if let int = try? container.decode(Int.self) {
            self.value = int
        } else if let double = try? container.decode(Double.self) {
            self.value = double
        } else if let string = try? container.decode(String.self) {
            self.value = string
        } else if let array = try? container.decode([AnyCodable].self) {
            self.value = array.map { $0.value }
        } else if let dict = try? container.decode([String: AnyCodable].self) {
            self.value = dict.mapValues { $0.value }
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "Cannot decode value")
        }
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        
        switch self.value {
        case is NSNull:
            try container.encodeNil()
        case let bool as Bool:
            try container.encode(bool)
        case let int as Int:
            try container.encode(int)
        case let double as Double:
            try container.encode(double)
        case let string as String:
            try container.encode(string)
        case let array as [Any]:
            try container.encode(array.map { AnyCodable(value: $0) })
        case let dict as [String: Any]:
            try container.encode(dict.mapValues { AnyCodable(value: $0) })
        default:
            throw EncodingError.invalidValue(self.value, EncodingError.Context(codingPath: container.codingPath, debugDescription: "Cannot encode value"))
        }
    }
}

class LibsqlPlugin: Plugin {
    // 存储数据库连接和实例
    private var connections: [String: Any] = [:]
    private var databases: [String: Any] = [:]
    
    @objc public func ping(_ invoke: Invoke) throws {
        let request = try invoke.parseArgs(PingRequest.self)
        invoke.resolve(["value": request.value as Any])
    }
    
    @objc public func connect(_ invoke: Invoke) throws {
        let options = try invoke.parseArgs(ConnectOptions.self)
        
        // 在实际实现中，这里应该使用 libSQL Swift 客户端
        // 为简化示例，我们仅模拟实现
        
        // 创建连接 ID
        let connectionId = UUID().uuidString
        
        // 创建一个 DispatchQueue 在后台线程操作数据库
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            
            // 模拟创建数据库连接
            // 实际实现应该使用 LibSQL 客户端
            // let db = LibSQL.connect(url: options.url, authToken: options.authToken, localPath: options.localPath)
            
            // 存储连接和数据库实例
            self.connections[connectionId] = "connection"  // 模拟连接
            self.databases[connectionId] = "database"      // 模拟数据库
            
            // 返回连接 ID
            DispatchQueue.main.async {
                invoke.resolve([connectionId])
            }
        }
    }
    
    @objc public func execute(_ invoke: Invoke) throws {
        let options = try invoke.parseArgs(ExecuteOptions.self)
        
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            
            // 检查连接是否存在
            guard self.connections[options.connectionId] != nil else {
                DispatchQueue.main.async {
                    invoke.reject("Connection not found", "DATABASE_CONNECTION_NOT_FOUND")
                }
                return
            }
            
            // 在实际实现中，获取连接并执行 SQL
            // let connection = self.connections[options.connectionId] as! LibSQLConnection
            // let params = self.convertParams(options.params)
            // let result = try? connection.execute(options.sql, params: params)
            
            // 模拟执行结果
            DispatchQueue.main.async {
                invoke.resolve([
                    "rowsAffected": 1,
                    "lastInsertRowid": 1
                ])
            }
        }
    }
    
    @objc public func query(_ invoke: Invoke) throws {
        let options = try invoke.parseArgs(QueryOptions.self)
        
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            
            // 检查连接是否存在
            guard self.connections[options.connectionId] != nil else {
                DispatchQueue.main.async {
                    invoke.reject("Connection not found", "DATABASE_CONNECTION_NOT_FOUND")
                }
                return
            }
            
            // 在实际实现中，获取连接并执行查询
            // let connection = self.connections[options.connectionId] as! LibSQLConnection
            // let params = self.convertParams(options.params)
            // let rows = try? connection.query(options.sql, params: params)
            
            // 模拟查询结果
            DispatchQueue.main.async {
                invoke.resolve([
                    "columns": [],
                    "rows": []
                ])
            }
        }
    }
    
    @objc public func sync(_ invoke: Invoke) throws {
        let options = try invoke.parseArgs(SyncOptions.self)
        
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            
            // 检查数据库是否存在
            guard self.databases[options.connectionId] != nil else {
                DispatchQueue.main.async {
                    invoke.reject("Database not found", "DATABASE_NOT_FOUND")
                }
                return
            }
            
            // 在实际实现中，获取数据库并同步
            // let database = self.databases[options.connectionId] as! LibSQLDatabase
            // try? database.sync()
            
            DispatchQueue.main.async {
                invoke.resolve(nil)
            }
        }
    }
    
    @objc public func close(_ invoke: Invoke) throws {
        let options = try invoke.parseArgs(CloseOptions.self)
        
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }
            
            // 移除连接和数据库
            self.connections.removeValue(forKey: options.connectionId)
            self.databases.removeValue(forKey: options.connectionId)
            
            DispatchQueue.main.async {
                invoke.resolve(nil)
            }
        }
    }
    
    // 助手方法：转换参数
    private func convertParams(_ params: [Value]?) -> [Any] {
        guard let params = params else { return [] }
        
        return params.map { param in
            switch param.type {
            case "Null":
                return NSNull()
            case "Integer":
                if let value = param.value?.value as? Int {
                    return value
                }
                return 0
            case "Real":
                if let value = param.value?.value as? Double {
                    return value
                }
                return 0.0
            case "Text":
                if let value = param.value?.value as? String {
                    return value
                }
                return ""
            case "Blob":
                if let value = param.value?.value as? [UInt8] {
                    return Data(value)
                }
                return Data()
            default:
                return NSNull()
            }
        }
    }
} 