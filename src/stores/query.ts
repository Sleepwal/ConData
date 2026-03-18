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

  const hasResult = computed(() => queryResult.value !== null)
  const hasError = computed(() => error.value !== null)

  async function executeQuery(connectionId: string, sql: string, limit: number = 1000) {
    if (!sql.trim()) {
      error.value = 'SQL语句不能为空'
      return
    }

    loading.value = true
    error.value = null
    queryResult.value = null

    try {
      const request: QueryRequest = {
        connection_id: connectionId,
        sql: sql.trim(),
        limit
      }

      const startTime = Date.now()
      const result = await queryApi.executeQuery(request)
      const executionTime = Date.now() - startTime

      queryResult.value = result

      const historyItem: QueryHistoryItem = {
        id: Date.now().toString(),
        connection_id: connectionId,
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
      schemas.value = await queryApi.getSchemas(connectionId)
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
      tables.value = await queryApi.getTables(connectionId)
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
      return await queryApi.getTableSchema(connectionId, schema, tableName)
    } catch (err) {
      console.error('Failed to get table schema:', err)
      return null
    }
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
    executeQuery,
    loadDatabases,
    loadSchemas,
    loadTables,
    getTableSchema,
    setCurrentSql,
    clearResult,
    clearHistory,
    clearError
  }
})
