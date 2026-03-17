//! 错误处理模块
//! 
//! 本模块定义了应用程序中使用的统一错误类型，提供错误转换和格式化功能。
//! 所有错误类型都实现了 serde 的序列化/反序列化，以便通过 Tauri 命令传递给前端。

use serde::{Deserialize, Serialize};
use std::fmt;

/// 应用程序错误类型枚举
/// 
/// 定义了应用程序中可能出现的所有错误类型，每种错误都包含详细的错误信息。
/// 实现了 Display trait 用于格式化错误信息，Error trait 用于标准错误处理。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    /// 数据库操作错误
    /// 
    /// 参数：错误描述字符串
    DatabaseError(String),
    
    /// 数据库连接错误
    /// 
    /// 参数：错误描述字符串
    ConnectionError(String),
    
    /// 配置相关错误
    /// 
    /// 参数：错误描述字符串
    ConfigError(String),
    
    /// 数据验证错误
    /// 
    /// 参数：错误描述字符串
    ValidationError(String),
    
    /// 资源未找到错误
    /// 
    /// 参数：错误描述字符串
    NotFound(String),
    
    /// 内部系统错误
    /// 
    /// 参数：错误描述字符串
    InternalError(String),
}

impl fmt::Display for AppError {
    /// 格式化错误信息
    /// 
    /// # 参数
    /// * `f` - 格式化器
    /// 
    /// # 返回值
    /// * `fmt::Result` - 格式化结果
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// ==================== 错误类型转换实现 ====================

impl From<tokio_postgres::Error> for AppError {
    /// 从 tokio_postgres 错误转换为 AppError
    /// 
    /// # 参数
    /// * `err` - PostgreSQL 客户端错误
    /// 
    /// # 返回值
    /// * `AppError::DatabaseError` - 包装后的数据库错误
    fn from(err: tokio_postgres::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    /// 从 JSON 序列化错误转换为 AppError
    /// 
    /// # 参数
    /// * `err` - JSON 处理错误
    /// 
    /// # 返回值
    /// * `AppError::InternalError` - 包装后的内部错误
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalError(format!("JSON serialization error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    /// 从 IO 错误转换为 AppError
    /// 
    /// # 参数
    /// * `err` - IO 操作错误
    /// 
    /// # 返回值
    /// * `AppError::InternalError` - 包装后的内部错误
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(format!("IO error: {}", err))
    }
}

impl From<anyhow::Error> for AppError {
    /// 从 anyhow 错误转换为 AppError
    /// 
    /// # 参数
    /// * `err` - anyhow 通用错误
    /// 
    /// # 返回值
    /// * `AppError::InternalError` - 包装后的内部错误
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

/// 应用程序结果类型别名
/// 
/// 统一使用 Result<T, AppError> 作为函数的返回类型，简化错误处理
/// 
/// # 类型参数
/// * `T` - 成功时的返回类型
/// 
/// # 示例
/// ```rust
/// pub fn example() -> Result<String> {
///     Ok("success".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, AppError>;
