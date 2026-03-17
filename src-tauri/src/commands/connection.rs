use crate::db::pool::ConnectionPool;
use crate::error::Result;
use crate::models::connection::{ConnectionConfig, ConnectionStatus, TestConnectionRequest};
use crate::services::connection_manager::ConnectionManager;
use crate::services::postgres_service::PostgresService;

#[tauri::command]
pub async fn test_connection(request: TestConnectionRequest) -> Result<ConnectionStatus> {
    let config = ConnectionConfig {
        id: "test".to_string(),
        name: "test".to_string(),
        host: request.host,
        port: request.port,
        database: request.database,
        username: request.username,
        password: request.password,
        ssl_mode: request.ssl_mode,
        created_at: None,
        updated_at: None,
    };

    match ConnectionPool::test_connection(&config).await {
        Ok(_) => Ok(ConnectionStatus {
            id: "test".to_string(),
            connected: true,
            message: "Connection successful".to_string(),
            database_version: None,
        }),
        Err(e) => Ok(ConnectionStatus {
            id: "test".to_string(),
            connected: false,
            message: format!("Connection failed: {}", e),
            database_version: None,
        }),
    }
}

#[tauri::command]
pub async fn save_connection(config: ConnectionConfig) -> Result<ConnectionConfig> {
    ConnectionManager::save_connection(config)
}

#[tauri::command]
pub fn get_connections() -> Result<Vec<ConnectionConfig>> {
    ConnectionManager::get_all_connections()
}

#[tauri::command]
pub fn delete_connection(id: String) -> Result<()> {
    ConnectionManager::delete_connection(&id)
}

#[tauri::command]
pub async fn connect(connection_id: String) -> Result<ConnectionStatus> {
    let config = ConnectionManager::get_connection(&connection_id)?;
    
    match ConnectionPool::connect(connection_id.clone(), &config).await {
        Ok(_) => {
            let version = PostgresService::get_database_version(&connection_id).await.ok();
            Ok(ConnectionStatus {
                id: connection_id,
                connected: true,
                message: "Connected successfully".to_string(),
                database_version: version,
            })
        }
        Err(e) => Ok(ConnectionStatus {
            id: connection_id,
            connected: false,
            message: format!("Connection failed: {}", e),
            database_version: None,
        }),
    }
}

#[tauri::command]
pub fn disconnect(connection_id: String) -> Result<ConnectionStatus> {
    ConnectionPool::disconnect(&connection_id)?;
    Ok(ConnectionStatus {
        id: connection_id,
        connected: false,
        message: "Disconnected".to_string(),
        database_version: None,
    })
}

#[tauri::command]
pub fn get_connection_status(connection_id: String) -> Result<ConnectionStatus> {
    let connected = ConnectionPool::is_connected(&connection_id);
    Ok(ConnectionStatus {
        id: connection_id,
        connected,
        message: if connected {
            "Connected".to_string()
        } else {
            "Not connected".to_string()
        },
        database_version: None,
    })
}

#[tauri::command]
pub fn get_active_connections() -> Result<Vec<String>> {
    Ok(ConnectionPool::get_active_connections())
}
