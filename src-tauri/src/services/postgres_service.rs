use crate::db::pool::ConnectionPool;
use crate::error::{AppError, Result};
use crate::models::query::{ColumnInfo, ColumnSchema, DatabaseInfo, QueryResult, SchemaInfo, TableInfo, TableSchema};
use serde_json::Value;
use std::time::Instant;

pub struct PostgresService;

impl PostgresService {
    pub async fn execute_query(connection_id: &str, sql: &str, limit: Option<usize>) -> Result<QueryResult> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let start = Instant::now();

        let rows = client
            .query(sql, &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Query execution failed: {}", e)))?;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        if rows.is_empty() {
            return Ok(QueryResult {
                columns: vec![],
                rows: vec![],
                row_count: 0,
                execution_time_ms,
                success: true,
                message: Some("Query executed successfully, no rows returned".to_string()),
            });
        }

        let columns: Vec<ColumnInfo> = rows[0]
            .columns()
            .iter()
            .map(|col| ColumnInfo {
                name: col.name().to_string(),
                data_type: format!("{:?}", col.type_()),
            })
            .collect();

        let mut result_rows: Vec<Vec<Value>> = Vec::new();

        for row in &rows {
            let mut row_values: Vec<Value> = Vec::new();
            for (i, _col) in row.columns().iter().enumerate() {
                let value: Value = Self::convert_to_json_value(row, i)?;
                row_values.push(value);
            }
            result_rows.push(row_values);
        }

        if let Some(limit_val) = limit {
            if result_rows.len() > limit_val {
                result_rows.truncate(limit_val);
            }
        }

        Ok(QueryResult {
            columns,
            rows: result_rows,
            row_count: rows.len(),
            execution_time_ms,
            success: true,
            message: None,
        })
    }

    pub async fn get_tables(connection_id: &str) -> Result<Vec<TableInfo>> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let sql = r#"
            SELECT table_schema, table_name, table_type
            FROM information_schema.tables
            WHERE table_schema NOT IN ('pg_catalog', 'information_schema')
            ORDER BY table_schema, table_name
        "#;

        let rows = client
            .query(sql, &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get tables: {}", e)))?;

        let tables: Vec<TableInfo> = rows
            .iter()
            .map(|row| TableInfo {
                schema: row.try_get(0).ok().flatten().unwrap_or_default(),
                name: row.try_get(1).ok().flatten().unwrap_or_default(),
                table_type: row.try_get(2).ok().flatten().unwrap_or_default(),
            })
            .collect();

        Ok(tables)
    }

    pub async fn get_table_schema(connection_id: &str, schema: &str, table_name: &str) -> Result<TableSchema> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let sql = r#"
            SELECT 
                c.column_name,
                c.data_type,
                c.is_nullable,
                c.column_default,
                CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key
            FROM information_schema.columns c
            LEFT JOIN (
                SELECT ku.column_name, ku.table_schema, ku.table_name
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage ku
                    ON tc.constraint_name = ku.constraint_name
                    AND tc.table_schema = ku.table_schema
                WHERE tc.constraint_type = 'PRIMARY KEY'
            ) pk ON c.column_name = pk.column_name 
                AND c.table_schema = pk.table_schema 
                AND c.table_name = pk.table_name
            WHERE c.table_schema = $1 AND c.table_name = $2
            ORDER BY c.ordinal_position
        "#;

        let rows = client
            .query(sql, &[&schema, &table_name])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get table schema: {}", e)))?;

        let columns: Vec<ColumnSchema> = rows
            .iter()
            .map(|row| ColumnSchema {
                name: row.get(0),
                data_type: row.get(1),
                is_nullable: row.get::<_, String>(2) == "YES",
                column_default: row.get(3),
                is_primary_key: row.get(4),
            })
            .collect();

        Ok(TableSchema {
            schema: schema.to_string(),
            table_name: table_name.to_string(),
            columns,
        })
    }

    pub async fn get_database_version(connection_id: &str) -> Result<String> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let row = client
            .query_one("SELECT version()", &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get version: {}", e)))?;

        let version: String = row.get(0);
        Ok(version)
    }

    fn convert_to_json_value(row: &tokio_postgres::Row, index: usize) -> Result<Value> {
        let column = row.columns().get(index).ok_or_else(|| {
            AppError::DatabaseError(format!("Column index {} out of bounds", index))
        })?;

        let type_name = column.type_().name();

        let value: Value = match type_name {
            "bool" => {
                let val: Option<bool> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "int2" | "int4" => {
                let val: Option<i32> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "int8" => {
                let val: Option<i64> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "float4" => {
                let val: Option<f32> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "float8" => {
                let val: Option<f64> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "text" | "varchar" | "bpchar" | "name" => {
                let val: Option<String> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "timestamp" | "timestamptz" => {
                let val: Option<String> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "date" => {
                let val: Option<String> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
            "json" | "jsonb" => {
                let val: Option<Value> = row.try_get(index).ok();
                val.unwrap_or(Value::Null)
            }
            "uuid" => {
                let val: Option<uuid::Uuid> = row.try_get(index).ok();
                serde_json::to_value(val.map(|v| v.to_string())).unwrap_or(Value::Null)
            }
            _ => {
                let val: Option<String> = row.try_get(index).ok();
                serde_json::to_value(val).unwrap_or(Value::Null)
            }
        };

        Ok(value)
    }

    pub async fn get_databases(connection_id: &str) -> Result<Vec<DatabaseInfo>> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let sql = r#"
            SELECT 
                datname as name,
                pg_get_userbyid(datdba) as owner,
                pg_encoding_to_char(encoding) as encoding,
                datcollate as collate
            FROM pg_database
            WHERE datname NOT IN ('template0', 'template1', 'postgres')
              AND datistemplate = false
            ORDER BY datname
        "#;

        let rows = client
            .query(sql, &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get databases: {}", e)))?;

        let databases: Vec<DatabaseInfo> = rows
            .iter()
            .map(|row| DatabaseInfo {
                name: row.get(0),
                owner: row.try_get(1).ok().flatten(),
                encoding: row.try_get(2).ok().flatten(),
                collate: row.try_get(3).ok().flatten(),
            })
            .collect();

        Ok(databases)
    }

    pub async fn get_schemas(connection_id: &str) -> Result<Vec<SchemaInfo>> {
        let pool = ConnectionPool::get_pool(connection_id)?;
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let sql = r#"
            SELECT 
                schema_name as name,
                schema_owner as owner
            FROM information_schema.schemata
            WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast', 'pg_temp_1', 'pg_toast_temp_1')
              AND schema_name NOT LIKE 'pg_temp_%'
              AND schema_name NOT LIKE 'pg_toast_temp_%'
            ORDER BY schema_name
        "#;

        let rows = client
            .query(sql, &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get schemas: {}", e)))?;

        let schemas: Vec<SchemaInfo> = rows
            .iter()
            .map(|row| SchemaInfo {
                name: row.get(0),
                owner: row.try_get(1).ok().flatten(),
            })
            .collect();

        Ok(schemas)
    }
}
