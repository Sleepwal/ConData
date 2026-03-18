<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useConnectionStore } from '../stores/connection'
import { useQueryStore } from '../stores/query'
import QueryEditor from '../components/query/QueryEditor.vue'
import QueryResult from '../components/query/QueryResult.vue'
import QueryHistory from '../components/query/QueryHistory.vue'
import TableDetail from '../components/query/TableDetail.vue'

const connectionStore = useConnectionStore()
const queryStore = useQueryStore()

const showSidebar = ref(true)
const showSqlPanel = ref(false)
const expandedSchema = ref<string | null>(null)

onMounted(() => {
  if (connectionStore.activeConnectionId) {
    loadDatabaseData(connectionStore.activeConnectionId)
  }
})

watch(() => connectionStore.activeConnectionId, (newId, oldId) => {
  if (newId !== oldId) {
    // Reset state when connection changes
    queryStore.resetState()
    expandedSchema.value = null

    if (newId) {
      loadDatabaseData(newId)
    } else {
      queryStore.databases = []
      queryStore.schemas = []
      queryStore.tables = []
    }
  }
})

async function loadDatabaseData(connectionId: string) {
  // Only load databases list initially
  try {
    await queryStore.loadDatabases(connectionId)
  } catch (err) {
    console.error('Failed to load databases:', err)
  }
}

// Click database to switch to it
async function onDatabaseClick(dbName: string) {
  if (queryStore.currentDatabase === dbName) return

  const baseId = connectionStore.activeConnectionId
  if (!baseId) return

  try {
    await queryStore.switchDatabase(baseId, dbName)
    expandedSchema.value = null
  } catch (err) {
    console.error('Failed to switch database:', err)
  }
}

// Toggle schema to expand/collapse and load tables
async function toggleSchema(schemaName: string) {
  if (expandedSchema.value === schemaName) {
    expandedSchema.value = null
    return
  }

  expandedSchema.value = schemaName

  const connId = queryStore.getEffectiveConnectionId(connectionStore.activeConnectionId!)
  await queryStore.loadTablesBySchema(connId, schemaName)
}

// Click table to show detail
function onTableClick(schema: string, tableName: string) {
  const connId = queryStore.getEffectiveConnectionId(connectionStore.activeConnectionId!)
  queryStore.loadTableDetail(connId, schema, tableName)
}


async function refreshAll() {
  if (connectionStore.activeConnectionId) {
    queryStore.resetState()
    expandedSchema.value = null
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

          <div v-else-if="queryStore.loadingDatabases" class="sidebar-message">
            加载中...
          </div>

          <div v-else class="tree-view">
            <!-- Database List -->
            <div v-if="queryStore.databases.length > 0" class="tree-level">
              <div class="tree-label tree-label-database" @click="queryStore.currentDatabase = null">
                <span class="tree-icon">🗄️</span>
                <span class="tree-text">数据库 ({{ queryStore.databases.length }})</span>
              </div>

              <div class="tree-children">
                <div
                  v-for="db in queryStore.databases"
                  :key="db.name"
                  class="tree-item"
                >
                  <div
                    class="tree-label"
                    :class="{ active: queryStore.currentDatabase === db.name }"
                    @click="onDatabaseClick(db.name)"
                  >
                    <span class="tree-icon">📊</span>
                    <span class="tree-text">{{ db.name }}</span>
                    <span v-if="db.owner" class="tree-owner"> ({{ db.owner }})</span>
                    <span v-if="queryStore.currentDatabase === db.name" class="current-indicator" title="当前数据库">✓</span>
                  </div>

                  <!-- Schema list for current database -->
                  <div v-if="queryStore.currentDatabase === db.name" class="tree-children">
                    <div v-if="queryStore.loadingSchemas" class="tree-loading">
                      加载模式中...
                    </div>
                    <div
                      v-for="schema in queryStore.schemas"
                      :key="schema.name"
                      class="tree-item"
                    >
                      <div class="tree-label" @click="toggleSchema(schema.name)">
                        <span class="tree-icon">📁</span>
                        <span class="tree-text">{{ schema.name }}</span>
                        <span v-if="schema.owner" class="tree-owner"> ({{ schema.owner }})</span>
                        <span class="tree-arrow">{{ expandedSchema === schema.name ? '▼' : '▶' }}</span>
                      </div>

                      <!-- Tables for this schema -->
                      <div v-if="expandedSchema === schema.name" class="tree-children">
                        <div v-if="queryStore.loadingTables" class="tree-loading">
                          加载表格中...
                        </div>
                        <div
                          v-for="table in queryStore.schemaTables.get(schema.name) || []"
                          :key="`${table.schema}.${table.name}`"
                          class="table-item"
                          @click="onTableClick(schema.name, table.name)"
                        >
                          <span class="table-icon">📋</span>
                          <span class="table-name">{{ table.name }}</span>
                          <span v-if="table.table_type !== 'BASE TABLE'" class="table-type"> ({{ table.table_type }})</span>
                        </div>
                        <div v-if="(queryStore.schemaTables.get(schema.name) || []).length === 0 && !queryStore.loadingTables" class="tree-empty">
                          无表格
                        </div>
                      </div>
                    </div>
                    <div v-if="queryStore.schemas.length === 0 && !queryStore.loadingSchemas" class="tree-empty">
                      无模式
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="queryStore.databases.length === 0" class="sidebar-message">
              暂无数据
            </div>
          </div>
        </div>

        <div class="sidebar-section">
          <QueryHistory />
        </div>
      </div>

      <div class="main-panel">
        <!-- 表详情面板（内嵌） -->
        <div v-if="queryStore.selectedTable" class="table-detail-section">
          <TableDetail />
        </div>

        <!-- 空状态提示 -->
        <div v-else class="empty-state">
          <div class="empty-content">
            <div class="empty-icon">📋</div>
            <p>点击左侧表名查看详情</p>
          </div>
        </div>

        <!-- SQL切换按钮 -->
        <div class="sql-toggle-section">
          <button
            class="btn"
            :class="showSqlPanel ? 'btn-secondary' : 'btn-primary'"
            @click="showSqlPanel = !showSqlPanel"
          >
            {{ showSqlPanel ? '隐藏SQL编辑器' : 'SQL语句' }}
          </button>
        </div>

        <!-- SQL编辑器和结果（可折叠） -->
        <div v-if="showSqlPanel" class="sql-panel">
          <div class="editor-section">
            <QueryEditor />
          </div>
          <div class="result-section">
            <QueryResult />
          </div>
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
  padding: 8px 0;
}

.tree-level {
  padding: 0;
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

.tree-label.active {
  background-color: #e3f2fd;
  color: #1976D2;
}

.tree-label-database {
  font-weight: 600;
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

.tree-label.active .tree-text {
  color: #1976D2;
  font-weight: 500;
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

.current-indicator {
  font-size: 12px;
  color: #4CAF50;
  margin-left: 4px;
}

.tree-children {
  padding-left: 16px;
}

.tree-item {
  border-left: 2px solid #eee;
}

.tree-loading {
  padding: 8px 16px;
  font-size: 12px;
  color: #999;
  font-style: italic;
}

.tree-empty {
  padding: 8px 16px;
  font-size: 12px;
  color: #bbb;
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
  min-height: 0;
}

/* 表详情区域 */
.table-detail-section {
  flex: 1;
  min-height: 0;
  overflow: auto;
}

/* 空状态 */
.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 300px;
}

.empty-content {
  text-align: center;
  color: #999;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-content p {
  font-size: 14px;
  margin: 0;
}

/* SQL切换按钮区域 */
.sql-toggle-section {
  display: flex;
  justify-content: center;
  padding: 8px 0;
  border-top: 1px solid #eee;
  border-bottom: 1px solid #eee;
}

/* SQL面板 */
.sql-panel {
  display: flex;
  flex-direction: column;
  gap: 20px;
  animation: slideDown 0.3s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
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

.btn-primary {
  background-color: #4CAF50;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #45a049;
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
