import { invoke } from '@tauri-apps/api/core'
import type {
  ConnectionConfig,
  ConnectionStatus,
  TestConnectionRequest,
  QueryRequest,
  QueryResult,
  TableInfo,
  TableSchema
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

  getTableSchema: (connectionId: string, schema: string, tableName: string): Promise<TableSchema> =>
    invoke('get_table_schema', { connectionId, schema, tableName })
}
