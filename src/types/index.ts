export enum SslMode {
  Disable = 'Disable',
  Allow = 'Allow',
  Prefer = 'Prefer',
  Require = 'Require'
}

export interface ConnectionConfig {
  id: string
  name: string
  host: string
  port: number
  database: string
  username: string
  password: string
  ssl_mode: SslMode
  created_at?: string
  updated_at?: string
}

export interface ConnectionStatus {
  id: string
  connected: boolean
  message: string
  database_version?: string
}

export interface TestConnectionRequest {
  host: string
  port: number
  database: string
  username: string
  password: string
  ssl_mode: SslMode
}

export interface ColumnInfo {
  name: string
  data_type: string
}

export interface QueryResult {
  columns: ColumnInfo[]
  rows: any[][]
  row_count: number
  execution_time_ms: number
  success: boolean
  message?: string
}

export interface QueryRequest {
  connection_id: string
  sql: string
  limit?: number
}

export interface TableInfo {
  schema: string
  name: string
  table_type: string
}

export interface ColumnSchema {
  name: string
  data_type: string
  is_nullable: boolean
  column_default?: string
  is_primary_key: boolean
}

export interface TableSchema {
  schema: string
  table_name: string
  columns: ColumnSchema[]
}

export interface QueryHistoryItem {
  id: string
  connection_id: string
  sql: string
  executed_at: string
  execution_time_ms: number
  row_count: number
  success: boolean
}
