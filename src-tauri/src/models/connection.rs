//! 数据库连接模型
//!
//! 本模块定义了数据库连接相关的所有数据结构，包括连接配置、连接状态等。
//! 所有结构体都实现了 Serialize 和 Deserialize trait，支持 JSON 序列化。

use crate::models::security::{decrypt_password, encrypt_password, EncryptedPassword, SecurityError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

/// SSL 连接模式枚举
///
/// 定义 PostgreSQL 数据库连接时的 SSL/TLS 加密模式选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SslMode {
    /// 禁用 SSL 连接
    ///
    /// 使用纯文本连接，不进行加密
    Disable,
    
    /// 允许 SSL 连接
    ///
    /// 优先使用非 SSL 连接，如果服务器要求则使用 SSL
    Allow,
    
    /// 优先使用 SSL 连接
    ///
    /// 默认选项，优先尝试 SSL 连接，如果服务器不支持则回退到非 SSL
    Prefer,
    
    /// 要求 SSL 连接
    ///
    /// 强制使用 SSL 连接，如果服务器不支持则连接失败
    Require,
}

impl Default for SslMode {
    /// 返回默认的 SSL 模式
    ///
    /// # 返回值
    /// * `SslMode::Prefer` - 优先使用 SSL
    fn default() -> Self {
        SslMode::Prefer
    }
}

/// 密码存储类型，支持明文和加密两种格式（用于向后兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum PasswordStorage {
    Encrypted(EncryptedPassword),
    Plaintext(String),
}

/// 数据库连接配置结构体
///
/// 存储单个数据库连接的所有配置信息，包括连接参数和元数据
#[derive(Debug, Clone, Zeroize)]
pub struct ConnectionConfig {
    /// 连接唯一标识符
    ///
    /// 使用 UUID v4 生成，用于在系统中唯一标识一个连接配置
    #[zeroize(skip)]
    pub id: String,
    
    /// 连接显示名称
    ///
    /// 用户自定义的连接名称，用于在界面中标识该连接
    #[zeroize(skip)]
    pub name: String,
    
    /// 数据库服务器主机地址
    ///
    /// 可以是 IP 地址（如 127.0.0.1）或域名（如 localhost, db.example.com）
    #[zeroize(skip)]
    pub host: String,
    
    /// 数据库服务器端口号
    ///
    /// PostgreSQL 默认端口为 5432
    #[zeroize(skip)]
    pub port: u16,
    
    /// 数据库名称
    ///
    /// 要连接的具体数据库实例名称
    #[zeroize(skip)]
    pub database: String,
    
    /// 数据库用户名
    ///
    /// 用于身份验证的数据库账户名
    #[zeroize(skip)]
    pub username: String,
    
    /// 数据库密码（敏感信息，加密存储）
    ///
    /// # 安全说明
    /// 使用 AES-256-GCM 加密存储，密钥派生自应用程序名称
    encrypted_password: Option<EncryptedPassword>,
    
    /// 临时存储的明文密码（仅用于初始化，不持久化）
    plaintext_password: Option<String>,
    
    /// SSL 连接模式
    ///
    /// 指定连接时使用的 SSL/TLS 加密模式
    #[zeroize(skip)]
    pub ssl_mode: SslMode,
    
    /// 创建时间（可选）
    ///
    /// ISO 8601 格式的日期时间字符串
    #[zeroize(skip)]
    pub created_at: Option<String>,
    
    /// 最后更新时间（可选）
    ///
    /// ISO 8601 格式的日期时间字符串
    #[zeroize(skip)]
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ConnectionConfigSerialized {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordStorage>,
    pub ssl_mode: SslMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Serialize for ConnectionConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let password = if let Some(encrypted) = &self.encrypted_password {
            Some(PasswordStorage::Encrypted(encrypted.clone()))
        } else if let Some(plain) = &self.plaintext_password {
            if !plain.is_empty() {
                let encrypted = encrypt_password(plain).map_err(serde::ser::Error::custom)?;
                Some(PasswordStorage::Encrypted(encrypted))
            } else {
                None
            }
        } else {
            None
        };

        let serialized = ConnectionConfigSerialized {
            id: self.id.clone(),
            name: self.name.clone(),
            host: self.host.clone(),
            port: self.port,
            database: self.database.clone(),
            username: self.username.clone(),
            password,
            ssl_mode: self.ssl_mode.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        };

        serialized.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ConnectionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serialized = ConnectionConfigSerialized::deserialize(deserializer)?;
        
        let (encrypted_password, plaintext_password) = match serialized.password {
            Some(PasswordStorage::Encrypted(enc)) => (Some(enc), None),
            Some(PasswordStorage::Plaintext(plain)) => {
                if !plain.is_empty() {
                    let encrypted = encrypt_password(&plain).map_err(serde::de::Error::custom)?;
                    (Some(encrypted), None)
                } else {
                    (None, None)
                }
            },
            None => (None, None),
        };

        Ok(ConnectionConfig {
            id: serialized.id,
            name: serialized.name,
            host: serialized.host,
            port: serialized.port,
            database: serialized.database,
            username: serialized.username,
            encrypted_password,
            plaintext_password,
            ssl_mode: serialized.ssl_mode,
            created_at: serialized.created_at,
            updated_at: serialized.updated_at,
        })
    }
}

impl ConnectionConfig {
    /// 创建新的连接配置实例
    ///
    /// 自动生成 UUID 作为连接 ID，SSL 模式使用默认值
    ///
    /// # 参数
    /// * `name` - 连接显示名称
    /// * `host` - 数据库服务器主机地址
    /// * `port` - 数据库服务器端口号
    /// * `database` - 数据库名称
    /// * `username` - 数据库用户名
    /// * `password` - 数据库密码
    ///
    /// # 返回值
    /// * `ConnectionConfig` - 新的连接配置实例
    pub fn new(
        name: String,
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
    ) -> Self {
        let plaintext_password = if password.is_empty() { None } else { Some(password) };
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            host,
            port,
            database,
            username,
            encrypted_password: None,
            plaintext_password,
            ssl_mode: SslMode::default(),
            created_at: None,
            updated_at: None,
        }
    }

    /// 设置密码
    ///
    /// # 参数
    /// * `password` - 要设置的密码
    pub fn set_password(&mut self, password: String) {
        self.plaintext_password = if password.is_empty() { None } else { Some(password) };
        self.encrypted_password = None;
    }

    /// 获取解密后的密码
    ///
    /// # 返回值
    /// * `Result<Option<String>, SecurityError>` - 解密后的密码，如果没有密码则返回 None
    pub fn get_password(&self) -> Result<Option<String>, SecurityError> {
        if let Some(plain) = &self.plaintext_password {
            Ok(Some(plain.clone()))
        } else if let Some(encrypted) = &self.encrypted_password {
            Ok(Some(decrypt_password(encrypted)?))
        } else {
            Ok(None)
        }
    }

    /// 加密存储密码（在保存前调用）
    pub fn encrypt_and_store_password(&mut self) -> Result<(), SecurityError> {
        if let Some(plain) = &self.plaintext_password {
            if !plain.is_empty() {
                self.encrypted_password = Some(encrypt_password(plain)?);
                self.plaintext_password = None;
            }
        }
        Ok(())
    }

    /// 生成 PostgreSQL 连接字符串
    ///
    /// 根据配置参数生成标准的 PostgreSQL 连接字符串，
    /// 格式遵循 libpq 连接字符串规范
    ///
    /// # 返回值
    /// * `Result<String, SecurityError>` - PostgreSQL 连接字符串
    pub fn connection_string(&self) -> Result<String, SecurityError> {
        let password = self.get_password()?.unwrap_or_default();
        Ok(format!(
            "host={} port={} dbname={} user={} password={} sslmode={}",
            self.host,
            self.port,
            self.database,
            self.username,
            password,
            self.ssl_mode.to_string()
        ))
    }
}

impl std::fmt::Display for SslMode {
    /// 将 SSL 模式转换为字符串表示
    ///
    /// # 参数
    /// * `f` - 格式化器
    ///
    /// # 返回值
    /// * `fmt::Result` - 格式化结果
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SslMode::Disable => write!(f, "disable"),
            SslMode::Allow => write!(f, "allow"),
            SslMode::Prefer => write!(f, "prefer"),
            SslMode::Require => write!(f, "require"),
        }
    }
}

/// 连接状态结构体
///
/// 表示一个数据库连接的当前状态信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    /// 连接 ID
    ///
    /// 关联的连接配置唯一标识符
    pub id: String,
    
    /// 连接状态标志
    ///
    /// true 表示已连接，false 表示未连接或连接已断开
    pub connected: bool,
    
    /// 状态描述信息
    ///
    /// 包含连接成功或失败的详细描述
    pub message: String,
    
    /// 数据库版本信息（可选）
    ///
    /// 连接成功后获取的数据库服务器版本字符串
    pub database_version: Option<String>,
}

/// 测试连接请求结构体
///
/// 用于测试数据库连接时传递的参数，不包含 ID 等元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConnectionRequest {
    /// 数据库服务器主机地址
    pub host: String,
    
    /// 数据库服务器端口号
    pub port: u16,
    
    /// 数据库名称
    pub database: String,
    
    /// 数据库用户名
    pub username: String,
    
    /// 数据库密码
    pub password: String,
    
    /// SSL 连接模式
    pub ssl_mode: SslMode,
}
