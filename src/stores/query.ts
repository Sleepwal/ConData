import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { QueryResult, QueryHistoryItem, QueryRequest, TableInfo, TableSchema, DatabaseInfo, SchemaInfo } from '../types'
import { queryApi } from '../api'

export const useQueryStore = defineStore('query', () => {
  const queryResult = ref<QueryResult | null>(null)
  const queryHistory = ref<QueryHistoryItem[]>([])
  const tables = ref<TableInfo[]>([])
  const databases = ref<DatabaseInfo[]>([])
  const schemas = ref<SchemaInfo[]>([])
  const currentSql = ref('')
  const loading = ref(false)
  const loadingDatabases = ref(false)
  const loadingSchemas = ref(false)
  const loadingTables = ref(false)
  const error = ref<string | null>(null)
  const databasesError = ref<string | null>(null)
  const schemasError = ref<string | null>(null)
  const tablesError = ref<string | null>(null)

  // New states for database switching and table details
  const currentDatabase = ref<string | null>(null)
  const currentConnectionId = ref<string | null>(null)
  const schemaTables = ref<Map<string, TableInfo[]>>(new Map())
  const selectedTable = ref<TableSchema | null>(null)
  const showTableDetail = ref(false)

  const hasResult = computed(() => queryResult.value !== null)
  const hasError = computed(() => error.value !== null)

  async function executeQuery(connectionId: string, sql: string, limit: number = 1000) {
    // Use the effective connection ID (switched database if any)
    const effectiveConnectionId = getEffectiveConnectionId(connectionId)

    if (!sql.trim()) {
      error.value = 'SQL语句不能为空'
      return
    }

    loading.value = true
    error.value = null
    queryResult.value = null

    try {
      const request: QueryRequest = {
        connection_id: effectiveConnectionId,
        sql: sql.trim(),
        limit
      }

      const startTime = Date.now()
      const result = await queryApi.executeQuery(request)
      const executionTime = Date.now() - startTime

      queryResult.value = result

      const historyItem: QueryHistoryItem = {
        id: Date.now().toString(),
        connection_id: effectiveConnectionId,
        sql: sql.trim(),
        executed_at: new Date().toISOString(),
        execution_time_ms: result.execution_time_ms || executionTime,
        row_count: result.row_count,
        success: result.success
      }

      queryHistory.value.unshift(historyItem)

      if (queryHistory.value.length > 100) {
        queryHistory.value = queryHistory.value.slice(0, 100)
      }

      return result
    } catch (err) {
      error.value = err instanceof Error ? err.message : '查询执行失败'
      console.error('Failed to execute query:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  async function loadDatabases(connectionId: string) {
    loadingDatabases.value = true
    databasesError.value = null
    try {
      databases.value = await queryApi.getDatabases(connectionId)
      return databases.value
    } catch (err) {
      databasesError.value = err instanceof Error ? err.message : '加载数据库列表失败'
      console.error('Failed to load databases:', err)
      throw err
    } finally {
      loadingDatabases.value = false
    }
  }

  async function loadSchemas(connectionId: string) {
    loadingSchemas.value = true
    schemasError.value = null
    try {
      const effectiveId = getEffectiveConnectionId(connectionId)
      schemas.value = await queryApi.getSchemas(effectiveId)
      return schemas.value
    } catch (err) {
      schemasError.value = err instanceof Error ? err.message : '加载模式列表失败'
      console.error('Failed to load schemas:', err)
      throw err
    } finally {
      loadingSchemas.value = false
    }
  }

  async function loadTables(connectionId: string) {
    loadingTables.value = true
    tablesError.value = null
    try {
      const effectiveId = getEffectiveConnectionId(connectionId)
      tables.value = await queryApi.getTables(effectiveId)
      return tables.value
    } catch (err) {
      tablesError.value = err instanceof Error ? err.message : '加载表列表失败'
      console.error('Failed to load tables:', err)
      throw err
    } finally {
      loadingTables.value = false
    }
  }

  async function getTableSchema(connectionId: string, schema: string, tableName: string): Promise<TableSchema | null> {
    try {
      const effectiveId = getEffectiveConnectionId(connectionId)
      return await queryApi.getTableSchema(effectiveId, schema, tableName)
    } catch (err) {
      console.error('Failed to get table schema:', err)
      return null
    }
  }

  // Switch to a different database
  async function switchDatabase(baseConnectionId: string, databaseName: string) {
    loadingDatabases.value = true
    try {
      const status = await queryApi.switchDatabase(baseConnectionId, databaseName)
      currentDatabase.value = databaseName
      currentConnectionId.value = status.id

      // Clear previous data
      schemas.value = []
      tables.value = []
      schemaTables.value.clear()
      selectedTable.value = null
      showTableDetail.value = false

      // Load schemas for the new database
      await loadSchemas(status.id)

      return status
    } catch (err) {
      databasesError.value = err instanceof Error ? err.message : '切换数据库失败'
      console.error('Failed to switch database:', err)
      throw err
    } finally {
      loadingDatabases.value = false
    }
  }

  // Load tables for a specific schema (lazy loading)
  async function loadTablesBySchema(connectionId: string, schema: string) {
    if (schemaTables.value.has(schema)) return

    loadingTables.value = true
    try {
      const effectiveId = getEffectiveConnectionId(connectionId)
      const tables = await queryApi.getTablesBySchema(effectiveId, schema)
      schemaTables.value.set(schema, tables)
    } catch (err) {
      tablesError.value = err instanceof Error ? err.message : '加载表列表失败'
      console.error('Failed to load tables by schema:', err)
      throw err
    } finally {
      loadingTables.value = false
    }
  }

  // Load table detail and show the detail panel
  async function loadTableDetail(connectionId: string, schema: string, tableName: string) {
    try {
      const effectiveId = getEffectiveConnectionId(connectionId)
      const schemaDetail = await queryApi.getTableSchema(effectiveId, schema, tableName)
      selectedTable.value = schemaDetail
      showTableDetail.value = true
    } catch (err) {
      console.error('Failed to load table detail:', err)
    }
  }

  // Get the effective connection ID (handles switched database)
  function getEffectiveConnectionId(baseId: string): string {
    return currentConnectionId.value || baseId
  }

  // Close table detail panel
  function closeTableDetail() {
    showTableDetail.value = false
    selectedTable.value = null
  }

  function setCurrentSql(sql: string) {
    currentSql.value = sql
  }

  function clearResult() {
    queryResult.value = null
    error.value = null
  }

  function clearHistory() {
    queryHistory.value = []
  }

  function clearError() {
    error.value = null
  }

  // Reset all state when disconnecting
  function resetState() {
    currentDatabase.value = null
    currentConnectionId.value = null
    schemaTables.value.clear()
    selectedTable.value = null
    showTableDetail.value = false
    databases.value = []
    schemas.value = []
    tables.value = []
    queryResult.value = null
  }

  return {
    queryResult,
    queryHistory,
    tables,
    databases,
    schemas,
    currentSql,
    loading,
    loadingDatabases,
    loadingSchemas,
    loadingTables,
    error,
    databasesError,
    schemasError,
    tablesError,
    hasResult,
    hasError,
    // New states
    currentDatabase,
    currentConnectionId,
    schemaTables,
    selectedTable,
    showTableDetail,
    // Original methods
    executeQuery,
    loadDatabases,
    loadSchemas,
    loadTables,
    getTableSchema,
    setCurrentSql,
    clearResult,
    clearHistory,
    clearError,
    // New methods
    switchDatabase,
    loadTablesBySchema,
    loadTableDetail,
    getEffectiveConnectionId,
    closeTableDetail,
    resetState
  }
})
