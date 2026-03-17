use crate::error::{AppError, Result};
use crate::models::connection::ConnectionConfig;
use dashmap::DashMap;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio_postgres::NoTls;

static CONNECTION_POOLS: Lazy<DashMap<String, Pool>> = Lazy::new(DashMap::new);

pub struct ConnectionPool;

impl ConnectionPool {
    pub async fn create_pool(config: &ConnectionConfig) -> Result<Pool> {
        let mut cfg = Config::new();
        cfg.host = Some(config.host.clone());
        cfg.port = Some(config.port);
        cfg.dbname = Some(config.database.clone());
        cfg.user = Some(config.username.clone());
        cfg.password = Some(config.password.clone());
        
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        
        cfg.pool = Some(deadpool_postgres::PoolConfig {
            max_size: 10,
            timeouts: deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(30)),
                create: Some(Duration::from_secs(30)),
                recycle: Some(Duration::from_secs(30)),
            },
            ..Default::default()
        });

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::ConnectionError(format!("Failed to create pool: {}", e)))?;

        Ok(pool)
    }

    pub async fn test_connection(config: &ConnectionConfig) -> Result<()> {
        let pool = Self::create_pool(config).await?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to get connection: {}", e)))?;
        
        client
            .query_one("SELECT version()", &[])
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to query: {}", e)))?;
        
        Ok(())
    }

    pub async fn connect(connection_id: String, config: &ConnectionConfig) -> Result<()> {
        let pool = Self::create_pool(config).await?;
        
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to get connection: {}", e)))?;
        
        client
            .query_one("SELECT version()", &[])
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to query: {}", e)))?;
        
        CONNECTION_POOLS.insert(connection_id, pool);
        Ok(())
    }

    pub fn disconnect(connection_id: &str) -> Result<()> {
        CONNECTION_POOLS.remove(connection_id);
        Ok(())
    }

    pub fn get_pool(connection_id: &str) -> Result<Pool> {
        CONNECTION_POOLS
            .get(connection_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| AppError::ConnectionError("Connection not found".to_string()))
    }

    pub fn is_connected(connection_id: &str) -> bool {
        CONNECTION_POOLS.contains_key(connection_id)
    }

    pub fn get_active_connections() -> Vec<String> {
        CONNECTION_POOLS
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
}
