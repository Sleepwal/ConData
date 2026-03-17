//! 查询相关数据模型
//!
//! 本模块定义了 SQL 查询相关的所有数据结构，包括查询请求、查询结果、表结构等。
//! 这些模型用于前后端之间传递查询参数和结果数据。

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询请求结构体
///
/// 用于从前端传递 SQL 查询请求到后端执行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    /// 连接 ID
    ///
    /// 指定要在哪个数据库连接上执行查询
    pub connection_id: String,
    
    /// SQL 查询语句
    ///
    /// 要执行的 SQL 语句，可以是 SELECT、INSERT、UPDATE、DELETE 等
    pub sql: String,
    
    /// 结果行数限制（可选）
    ///
    /// 限制返回的最大行数，防止大数据量查询导致内存溢出
    /// 默认值为 1000，设置为 None 表示不限制
    pub limit: Option<usize>,
}

/// 列信息结构体
///
/// 描述查询结果集中的一列的元数据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    /// 列名称
    ///
    /// 数据库中的列名或查询中的别名
    pub name: String,
    
    /// 数据类型
    ///
    /// PostgreSQL 数据类型的字符串表示
    pub data_type: String,
}

/// 查询结果结构体
///
/// 封装 SQL 查询执行后的完整结果信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// 列信息列表
    ///
    /// 结果集中所有列的元数据信息
    pub columns: Vec<ColumnInfo>,
    
    /// 数据行列表
    ///
    /// 二维数组，每行是一个 JSON 值数组，与 columns 一一对应
    pub rows: Vec<Vec<Value>>,
    
    /// 总行数
    ///
    /// 结果集中的数据行数量
    pub row_count: usize,
    
    /// 执行耗时（毫秒）
    ///
    /// SQL 语句从发送到返回结果的总耗时
    pub execution_time_ms: u64,
    
    /// 执行成功标志
    ///
    /// true 表示查询成功执行，false 表示执行过程中发生错误
    pub success: bool,
    
    /// 消息（可选）
    ///
    /// 成功时的提示信息或失败时的错误信息
    pub message: Option<String>,
}

/// 表信息结构体
///
/// 描述数据库中的一个表的基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    /// 表所属的模式（schema）
    ///
    /// PostgreSQL 中的 schema 名称，如 "public"
    pub schema: String,
    
    /// 表名称
    pub name: String,
    
    /// 表类型
    ///
    /// 如 "BASE TABLE"（普通表）、"VIEW"（视图）等
    pub table_type: String,
}

/// 列结构信息
///
/// 描述表中一列的详细结构信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    /// 列名称
    pub name: String,
    
    /// 数据类型
    ///
    /// 如 "integer", "varchar", "timestamp" 等
    pub data_type: String,
    
    /// 是否可为空
    ///
    /// true 表示该列允许 NULL 值
    pub is_nullable: bool,
    
    /// 默认值（可选）
    ///
    /// 列的默认值表达式，如 "nextval('seq_name')" 或 "'default_value'"
    pub column_default: Option<String>,
    
    /// 是否为主键
    ///
    /// true 表示该列是主键的一部分
    pub is_primary_key: bool,
}

/// 表结构信息
///
/// 包含一个表的完整结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    /// 表所属的模式
    pub schema: String,
    
    /// 表名称
    pub table_name: String,
    
    /// 列结构列表
    ///
    /// 表中所有列的详细结构信息
    pub columns: Vec<ColumnSchema>,
}

/// 查询历史项
///
/// 记录一次查询执行的元数据，用于查询历史功能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryItem {
    /// 历史记录 ID
    ///
    /// 唯一标识一条历史记录
    pub id: String,
    
    /// 连接 ID
    ///
    /// 执行该查询时使用的数据库连接
    pub connection_id: String,
    
    /// SQL 语句
    ///
    /// 实际执行的 SQL 语句文本
    pub sql: String,
    
    /// 执行时间
    ///
    /// ISO 8601 格式的执行时间戳
    pub executed_at: String,
    
    /// 执行耗时（毫秒）
    pub execution_time_ms: u64,
    
    /// 返回行数
    pub row_count: usize,
    
    /// 执行成功标志
    pub success: bool,
}

impl QueryResult {
    /// 创建空结果
    ///
    /// 用于初始化或返回空结果集
    ///
    /// # 返回值
    /// * `QueryResult` - 空的查询结果，success 为 true
    pub fn empty() -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            row_count: 0,
            execution_time_ms: 0,
            success: true,
            message: None,
        }
    }

    /// 创建错误结果
    ///
    /// 用于封装查询执行失败的错误信息
    ///
    /// # 参数
    /// * `message` - 错误描述信息
    ///
    /// # 返回值
    /// * `QueryResult` - 失败的查询结果，success 为 false
    pub fn error(message: String) -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            row_count: 0,
            execution_time_ms: 0,
            success: false,
            message: Some(message),
        }
    }
}
