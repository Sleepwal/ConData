import { invoke } from '@tauri-apps/api/core'
import type {
  ConnectionConfig,
  ConnectionStatus,
  TestConnectionRequest,
  QueryRequest,
  QueryResult,
  TableInfo,
  TableSchema,
  DatabaseInfo,
  SchemaInfo
} from '../types'

export const connectionApi = {
  testConnection: (request: TestConnectionRequest): Promise<ConnectionStatus> =>
    invoke('test_connection', { request }),

  saveConnection: (config: ConnectionConfig): Promise<ConnectionConfig> =>
    invoke('save_connection', { config }),

  getConnections: (): Promise<ConnectionConfig[]> =>
    invoke('get_connections'),

  deleteConnection: (id: string): Promise<void> =>
    invoke('delete_connection', { id }),

  connect: (connectionId: string): Promise<ConnectionStatus> =>
    invoke('connect', { connectionId }),

  disconnect: (connectionId: string): Promise<ConnectionStatus> =>
    invoke('disconnect', { connectionId }),

  getConnectionStatus: (connectionId: string): Promise<ConnectionStatus> =>
    invoke('get_connection_status', { connectionId }),

  getActiveConnections: (): Promise<string[]> =>
    invoke('get_active_connections')
}

export const queryApi = {
  executeQuery: (request: QueryRequest): Promise<QueryResult> =>
    invoke('execute_query', { request }),

  getTables: (connectionId: string): Promise<TableInfo[]> =>
    invoke('get_tables', { connectionId }),

  getTablesBySchema: (connectionId: string, schema: string): Promise<TableInfo[]> =>
    invoke('get_tables_by_schema', { connectionId, schema }),

  getTableSchema: (connectionId: string, schema: string, tableName: string): Promise<TableSchema> =>
    invoke('get_table_schema', { connectionId, schema, tableName }),

  getDatabases: (connectionId: string): Promise<DatabaseInfo[]> =>
    invoke('get_databases', { connectionId }),

  getSchemas: (connectionId: string): Promise<SchemaInfo[]> =>
    invoke('get_schemas', { connectionId }),

  switchDatabase: (connectionId: string, databaseName: string): Promise<ConnectionStatus> =>
    invoke('switch_database', { connectionId, databaseName })
}
