<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useConnectionStore } from '../stores/connection'
import { useQueryStore } from '../stores/query'
import QueryEditor from '../components/query/QueryEditor.vue'
import QueryResult from '../components/query/QueryResult.vue'
import QueryHistory from '../components/query/QueryHistory.vue'

const connectionStore = useConnectionStore()
const queryStore = useQueryStore()

const showSidebar = ref(true)
const expandedDatabase = ref<string | null>(null)
const expandedSchema = ref<string | null>(null)

onMounted(() => {
  if (connectionStore.activeConnectionId) {
    loadDatabaseData(connectionStore.activeConnectionId)
  }
})

watch(() => connectionStore.activeConnectionId, (newId) => {
  if (newId) {
    loadDatabaseData(newId)
  } else {
    queryStore.databases = []
    queryStore.schemas = []
    queryStore.tables = []
  }
})

async function loadDatabaseData(connectionId: string) {
  await Promise.all([
    queryStore.loadDatabases(connectionId),
    queryStore.loadSchemas(connectionId),
    queryStore.loadTables(connectionId)
  ])
}

function toggleDatabase(databaseName: string) {
  if (expandedDatabase.value === databaseName) {
    expandedDatabase.value = null
  } else {
    expandedDatabase.value = databaseName
  }
}

function toggleSchema(schemaName: string) {
  if (expandedSchema.value === schemaName) {
    expandedSchema.value = null
  } else {
    expandedSchema.value = schemaName
  }
}

function selectTable(schema: string, tableName: string) {
  const sql = `SELECT * FROM "${schema}"."${tableName}" LIMIT 100;`
  queryStore.setCurrentSql(sql)
}

async function refreshAll() {
  if (connectionStore.activeConnectionId) {
    await loadDatabaseData(connectionStore.activeConnectionId)
  }
}
</script>

<template>
  <div class="query-view">
    <div class="view-header">
      <h2 class="view-title">查询执行</h2>
      <div class="header-actions">
        <button class="btn btn-secondary" @click="showSidebar = !showSidebar">
          {{ showSidebar ? '隐藏' : '显示' }}侧边栏
        </button>
      </div>
    </div>

    <div class="view-content">
      <div v-if="showSidebar" class="sidebar">
        <div class="sidebar-section">
          <div class="section-header">
            <h3 class="section-title">数据库对象</h3>
            <button class="btn btn-sm" @click="refreshAll" :disabled="!connectionStore.isConnected">
              刷新
            </button>
          </div>
          
          <div v-if="!connectionStore.isConnected" class="sidebar-message">
            请先连接到数据库
          </div>
          
          <div v-else-if="queryStore.loading" class="sidebar-message">
            加载中...
          </div>
          
          <div v-else class="tree-view">
            <div v-if="queryStore.databases.length > 0" class="tree-level">
              <div class="tree-label tree-label-database" @click="expandedDatabase = expandedDatabase === null ? '_all' : null">
                <span class="tree-icon">🗄️</span>
                <span class="tree-text">数据库 ({{ queryStore.databases.length }})</span>
                <span class="tree-arrow">{{ expandedDatabase === null ? '▶' : '▼' }}</span>
              </div>
              
              <div v-if="expandedDatabase !== null" class="tree-children">
                <div
                  v-for="db in queryStore.databases"
                  :key="db.name"
                  class="tree-item"
                >
                  <div class="tree-label" @click="toggleDatabase(db.name)">
                    <span class="tree-icon">📊</span>
                    <span class="tree-text">{{ db.name }}</span>
                    <span v-if="db.owner" class="tree-owner"> ({{ db.owner }})</span>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="queryStore.schemas.length > 0" class="tree-level">
              <div class="tree-label tree-label-schema" @click="expandedSchema = expandedSchema === null ? '_all' : null">
                <span class="tree-icon">📁</span>
                <span class="tree-text">模式 ({{ queryStore.schemas.length }})</span>
                <span class="tree-arrow">{{ expandedSchema === null ? '▶' : '▼' }}</span>
              </div>
              
              <div v-if="expandedSchema !== null" class="tree-children">
                <div
                  v-for="schema in queryStore.schemas"
                  :key="schema.name"
                  class="tree-item"
                >
                  <div class="tree-label" @click="toggleSchema(schema.name)">
                    <span class="tree-icon">📂</span>
                    <span class="tree-text">{{ schema.name }}</span>
                    <span v-if="schema.owner" class="tree-owner"> ({{ schema.owner }})</span>
                    <span class="tree-arrow">{{ expandedSchema === schema.name ? '▼' : '▶' }}</span>
                  </div>
                  
                  <div v-if="expandedSchema === schema.name" class="tree-children">
                    <div
                      v-for="table in queryStore.tables.filter(t => t.schema === schema.name)"
                      :key="`${table.schema}.${table.name}`"
                      class="table-item"
                      @click="selectTable(table.schema, table.name)"
                    >
                      <span class="table-icon">📋</span>
                      <span class="table-name">{{ table.name }}</span>
                      <span v-if="table.table_type !== 'BASE TABLE'" class="table-type"> ({{ table.table_type }})</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="queryStore.tables.length === 0 && queryStore.databases.length === 0 && queryStore.schemas.length === 0" class="sidebar-message">
              暂无数据
            </div>
          </div>
        </div>

        <div class="sidebar-section">
          <QueryHistory />
        </div>
      </div>

      <div class="main-panel">
        <div class="editor-section">
          <QueryEditor />
        </div>
        <div class="result-section">
          <QueryResult />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.query-view {
  padding: 20px 0;
}

.view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.view-title {
  font-size: 24px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.view-content {
  display: grid;
  grid-template-columns: 320px 1fr;
  gap: 24px;
}

.sidebar {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.sidebar-section {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.sidebar-message {
  padding: 20px;
  text-align: center;
  color: #999;
  font-size: 14px;
}

.tree-view {
  max-height: 500px;
  overflow-y: auto;
}

.tree-level {
  padding: 8px 0;
}

.tree-label {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  user-select: none;
}

.tree-label:hover {
  background-color: #f5f5f5;
}

.tree-label-database {
  font-weight: 600;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.tree-label-schema {
  font-weight: 600;
  border-top: 1px solid #eee;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.tree-icon {
  font-size: 14px;
}

.tree-text {
  flex: 1;
  font-size: 13px;
  color: #333;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tree-owner {
  font-size: 11px;
  color: #999;
}

.tree-arrow {
  font-size: 10px;
  color: #999;
  width: 12px;
  text-align: center;
}

.tree-children {
  padding-left: 16px;
}

.tree-item {
  border-left: 2px solid #eee;
}

.table-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.table-item:hover {
  background-color: #f5f5f5;
}

.table-icon {
  font-size: 13px;
}

.table-name {
  font-size: 13px;
  color: #333;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.table-type {
  font-size: 11px;
  color: #999;
}

.main-panel {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.editor-section {
  flex: 0 0 auto;
}

.result-section {
  flex: 1;
  min-height: 300px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
  background-color: #f5f5f5;
  color: #333;
}

.btn:hover:not(:disabled) {
  background-color: #e0e0e0;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 13px;
}

.btn-secondary {
  background-color: #2196F3;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #1976D2;
}

@media (max-width: 1024px) {
  .view-content {
    grid-template-columns: 1fr;
  }

  .sidebar {
    order: 2;
  }

  .main-panel {
    order: 1;
  }
}
</style>
