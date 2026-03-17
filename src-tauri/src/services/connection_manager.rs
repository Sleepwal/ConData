use crate::error::{AppError, Result};
use crate::models::connection::ConnectionConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "connections.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionsData {
    connections: Vec<ConnectionConfig>,
}

impl Default for ConnectionsData {
    fn default() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
}

pub struct ConnectionManager;

impl ConnectionManager {
    fn get_config_path() -> Result<PathBuf> {
        let app_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("Failed to get config directory".to_string()))?
            .join("condata");
        
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| AppError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }
        
        Ok(app_dir.join(CONFIG_FILE_NAME))
    }

    fn load_connections_data() -> Result<ConnectionsData> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            return Ok(ConnectionsData::default());
        }
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let data: ConnectionsData = serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))?;
        
        Ok(data)
    }

    fn save_connections_data(data: &ConnectionsData) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    pub fn get_all_connections() -> Result<Vec<ConnectionConfig>> {
        let data = Self::load_connections_data()?;
        Ok(data.connections)
    }

    pub fn get_connection(id: &str) -> Result<ConnectionConfig> {
        let data = Self::load_connections_data()?;
        data.connections
            .into_iter()
            .find(|c| c.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Connection with id {} not found", id)))
    }

    pub fn save_connection(config: ConnectionConfig) -> Result<ConnectionConfig> {
        let mut data = Self::load_connections_data()?;
        let config_id = config.id.clone();
        
        if let Some(index) = data.connections.iter().position(|c| c.id == config_id) {
            data.connections[index] = config;
        } else {
            data.connections.push(config);
        }
        
        Self::save_connections_data(&data)?;
        
        let saved = data.connections
            .into_iter()
            .find(|c| c.id == config_id)
            .ok_or_else(|| AppError::InternalError("Failed to retrieve saved connection".to_string()))?;
        
        Ok(saved)
    }

    pub fn delete_connection(id: &str) -> Result<()> {
        let mut data = Self::load_connections_data()?;
        data.connections.retain(|c| c.id != id);
        Self::save_connections_data(&data)?;
        Ok(())
    }

    pub fn update_connection(config: ConnectionConfig) -> Result<ConnectionConfig> {
        let mut data = Self::load_connections_data()?;
        
        let index = data.connections
            .iter()
            .position(|c| c.id == config.id)
            .ok_or_else(|| AppError::NotFound(format!("Connection with id {} not found", config.id)))?;
        
        data.connections[index] = config.clone();
        Self::save_connections_data(&data)?;
        
        Ok(config)
    }

    pub fn connection_exists(id: &str) -> Result<bool> {
        let data = Self::load_connections_data()?;
        Ok(data.connections.iter().any(|c| c.id == id))
    }

    pub fn get_connection_count() -> Result<usize> {
        let data = Self::load_connections_data()?;
        Ok(data.connections.len())
    }
}
