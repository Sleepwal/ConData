//! 数据库连接池管理模块
//!
//! 本模块实现了 PostgreSQL 数据库连接池的管理功能。
//! 使用 deadpool-postgres 库提供高性能的连接池实现，支持多连接并发管理。
//!
//! # 主要功能
//! - 连接池的创建和配置
//! - 连接测试
//! - 多连接管理（支持同时维护多个数据库连接）
//! - 连接健康检查
//!
//! # 性能优化
//! - 使用 DashMap 实现高效的并发连接存储
//! - 连接池复用，避免频繁创建/销毁连接
//! - 可配置的连接池大小和超时设置

use crate::error::{AppError, Result};
use crate::models::connection::ConnectionConfig;
use dashmap::DashMap;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio_postgres::NoTls;

/// 全局连接池存储
///
/// 使用 DashMap 存储所有活动的连接池实例。
/// DashMap 提供了高效的并发读写性能，适合在多线程环境中使用。
///
/// # 键值对说明
/// - 键: 连接 ID (String) - 对应 ConnectionConfig 的 id 字段
/// - 值: 连接池实例 (Pool) - deadpool_postgres 的连接池
///
/// # 线程安全
/// DashMap 内部使用读写锁实现，支持安全的并发访问
static CONNECTION_POOLS: Lazy<DashMap<String, Pool>> = Lazy::new(DashMap::new);

/// 连接池管理器
///
/// 提供静态方法管理数据库连接池的生命周期。
/// 所有方法都是异步的，支持非阻塞的数据库操作。
pub struct ConnectionPool;

impl ConnectionPool {
    /// 创建新的数据库连接池
    ///
    /// 根据连接配置创建一个新的连接池实例。
    /// 连接池配置包括最大连接数、超时时间等参数。
    ///
    /// # 参数
    /// * `config` - 数据库连接配置，包含主机、端口、数据库名、用户名、密码等信息
    ///
    /// # 返回值
    /// * `Result<Pool>` - 成功返回连接池实例，失败返回错误
    ///
    /// # 连接池配置
    /// - 最大连接数: 10
    /// - 等待超时: 30秒
    /// - 创建超时: 30秒
    /// - 回收超时: 30秒
    /// - 回收策略: Fast（快速回收）
    ///
    /// # 错误处理
    /// 可能返回的错误类型：
    /// - `AppError::ConnectionError` - 连接池创建失败
    ///
    /// # 示例
    /// ```rust
    /// let config = ConnectionConfig::new(
    ///     "test".to_string(),
    ///     "localhost".to_string(),
    ///     5432,
    ///     "postgres".to_string(),
    ///     "postgres".to_string(),
    ///     "password".to_string()
    /// );
    /// let pool = ConnectionPool::create_pool(&config).await?;
    /// ```
    pub async fn create_pool(config: &ConnectionConfig) -> Result<Pool> {
        let password = config.get_password()?;
        let mut cfg = Config::new();
        cfg.host = Some(config.host.clone());
        cfg.port = Some(config.port);
        cfg.dbname = Some(config.database.clone());
        cfg.user = Some(config.username.clone());
        cfg.password = password;
        
        // 配置连接池管理器
        // RecyclingMethod::Fast 表示快速回收连接，不进行额外的验证
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        
        // 配置连接池参数
        cfg.pool = Some(deadpool_postgres::PoolConfig {
            max_size: 10,  // 最大连接数，可根据实际需求调整
            timeouts: deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(30)),    // 等待可用连接的超时时间
                create: Some(Duration::from_secs(30)),  // 创建新连接的超时时间
                recycle: Some(Duration::from_secs(30)), // 回收连接的超时时间
            },
            ..Default::default()
        });

        // 创建连接池，使用 Tokio 运行时和 NoTls（无 TLS 加密）
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::ConnectionError(format!("Failed to create pool: {}", e)))?;

        Ok(pool)
    }

    /// 测试数据库连接
    ///
    /// 尝试使用给定的配置连接到数据库，并执行一个简单的查询验证连接是否可用。
    /// 这是一个临时连接测试，不会在全局连接池存储中保留连接。
    ///
    /// # 参数
    /// * `config` - 数据库连接配置
    ///
    /// # 返回值
    /// * `Result<()>` - 成功返回 Ok(()，失败返回错误
    ///
    /// # 测试逻辑
    /// 1. 创建临时连接池
    /// 2. 从池中获取一个连接
    /// 3. 执行 "SELECT version()" 查询
    /// 4. 如果查询成功，说明连接可用
    ///
    /// # 错误处理
    /// 可能返回的错误类型：
    /// - `AppError::ConnectionError` - 连接失败或查询执行失败
    pub async fn test_connection(config: &ConnectionConfig) -> Result<()> {
        let pool = Self::create_pool(config).await?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to get connection: {}", e)))?;
        
        // 执行版本查询验证连接
        client
            .query_one("SELECT version()", &[])
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to query: {}", e)))?;
        
        Ok(())
    }

    /// 建立持久化数据库连接
    ///
    /// 创建连接池并将其存储在全局连接池存储中，供后续查询使用。
    /// 这是实际业务使用的连接方法。
    ///
    /// # 参数
    /// * `connection_id` - 连接唯一标识符，用于后续获取连接池
    /// * `config` - 数据库连接配置
    ///
    /// # 返回值
    /// * `Result<()>` - 成功返回 Ok(())，失败返回错误
    ///
    /// # 存储机制
    /// 使用 DashMap 存储连接池，键为 connection_id，值为 Pool 实例
    ///
    /// # 错误处理
    /// 可能返回的错误类型：
    /// - `AppError::ConnectionError` - 连接池创建失败或连接测试失败
    pub async fn connect(connection_id: String, config: &ConnectionConfig) -> Result<()> {
        let pool = Self::create_pool(config).await?;
        
        // 验证连接可用性
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to get connection: {}", e)))?;
        
        client
            .query_one("SELECT version()", &[])
            .await
            .map_err(|e| AppError::ConnectionError(format!("Failed to query: {}", e)))?;
        
        // 存储连接池到全局存储
        CONNECTION_POOLS.insert(connection_id, pool);
        Ok(())
    }

    /// 断开数据库连接
    ///
    /// 从全局连接池存储中移除指定连接，释放相关资源。
    /// 连接池会在移除后自动清理。
    ///
    /// # 参数
    /// * `connection_id` - 要断开的连接 ID
    ///
    /// # 返回值
    /// * `Result<()>` - 总是返回 Ok(())
    ///
    /// # 注意事项
    /// - 如果 connection_id 不存在，将返回错误
    /// - 连接池的清理是异步的，不会立即释放所有资源
    pub fn disconnect(connection_id: &str) -> Result<()> {
        if CONNECTION_POOLS.remove(connection_id).is_some() {
            Ok(())
        } else {
            Err(AppError::ConnectionError("Connection not found".to_string()))
        }
    }

    /// 获取指定连接的连接池
    ///
    /// 从全局存储中获取已存在的连接池实例。
    ///
    /// # 参数
    /// * `connection_id` - 连接 ID
    ///
    /// # 返回值
    /// * `Result<Pool>` - 成功返回连接池实例，失败返回错误
    ///
    /// # 错误处理
    /// 如果 connection_id 不存在，返回 `AppError::ConnectionError`
    pub fn get_pool(connection_id: &str) -> Result<Pool> {
        CONNECTION_POOLS
            .get(connection_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| AppError::ConnectionError("Connection not found".to_string()))
    }

    /// 检查连接是否处于活动状态
    ///
    /// 检查指定 connection_id 是否在全局连接池存储中存在。
    ///
    /// # 参数
    /// * `connection_id` - 连接 ID
    ///
    /// # 返回值
    /// * `bool` - true 表示连接存在，false 表示连接不存在
    ///
    /// # 注意事项
    /// 此方法只检查连接池是否存在于存储中，不验证连接的实际可用性
    pub fn is_connected(connection_id: &str) -> bool {
        CONNECTION_POOLS.contains_key(connection_id)
    }

    /// 获取所有活动的连接 ID 列表
    ///
    /// # 返回值
    /// * `Vec<String>` - 所有活动连接的 ID 列表
    ///
    /// # 用途
    /// 可用于管理界面显示当前所有活动连接，或批量操作
    pub fn get_active_connections() -> Vec<String> {
        CONNECTION_POOLS
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
}
