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

onMounted(() => {
  if (connectionStore.activeConnectionId) {
    queryStore.loadTables(connectionStore.activeConnectionId)
  }
})

watch(() => connectionStore.activeConnectionId, (newId) => {
  if (newId) {
    queryStore.loadTables(newId)
  } else {
    queryStore.tables = []
  }
})

function selectTable(schema: string, tableName: string) {
  const sql = `SELECT * FROM "${schema}"."${tableName}" LIMIT 100;`
  queryStore.setCurrentSql(sql)
}

function refreshTables() {
  if (connectionStore.activeConnectionId) {
    queryStore.loadTables(connectionStore.activeConnectionId)
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
            <h3 class="section-title">数据库表</h3>
            <button class="btn btn-sm" @click="refreshTables" :disabled="!connectionStore.isConnected">
              刷新
            </button>
          </div>
          <div v-if="!connectionStore.isConnected" class="sidebar-message">
            请先连接到数据库
          </div>
          <div v-else-if="queryStore.loading" class="sidebar-message">
            加载中...
          </div>
          <div v-else-if="queryStore.tables.length === 0" class="sidebar-message">
            暂无表
          </div>
          <div v-else class="tables-list">
            <div
              v-for="table in queryStore.tables"
              :key="`${table.schema}.${table.name}`"
              class="table-item"
              @click="selectTable(table.schema, table.name)"
            >
              <span class="table-icon">📋</span>
              <span class="table-name">{{ table.schema }}.{{ table.name }}</span>
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
  grid-template-columns: 300px 1fr;
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

.tables-list {
  max-height: 300px;
  overflow-y: auto;
}

.table-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  border-bottom: 1px solid #f0f0f0;
}

.table-item:hover {
  background-color: #f5f5f5;
}

.table-icon {
  font-size: 14px;
}

.table-name {
  font-size: 13px;
  color: #333;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
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
