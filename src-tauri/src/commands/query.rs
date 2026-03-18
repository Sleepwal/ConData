use crate::error::Result;
use crate::models::query::{DatabaseInfo, QueryRequest, QueryResult, SchemaInfo, TableInfo, TableSchema};
use crate::services::postgres_service::PostgresService;

#[tauri::command]
pub async fn execute_query(request: QueryRequest) -> Result<QueryResult> {
    let limit = request.limit.unwrap_or(1000);
    PostgresService::execute_query(&request.connection_id, &request.sql, Some(limit)).await
}

#[tauri::command]
pub async fn get_tables(connection_id: String) -> Result<Vec<TableInfo>> {
    PostgresService::get_tables(&connection_id).await
}

#[tauri::command]
pub async fn get_table_schema(
    connection_id: String,
    schema: String,
    table_name: String,
) -> Result<TableSchema> {
    PostgresService::get_table_schema(&connection_id, &schema, &table_name).await
}

#[tauri::command]
pub async fn get_databases(connection_id: String) -> Result<Vec<DatabaseInfo>> {
    PostgresService::get_databases(&connection_id).await
}

#[tauri::command]
pub async fn get_schemas(connection_id: String) -> Result<Vec<SchemaInfo>> {
    PostgresService::get_schemas(&connection_id).await
}
