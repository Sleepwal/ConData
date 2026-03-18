<script setup lang="ts">
import { ref, watch } from 'vue'
import { NTabs, NTabPane, NButton, NSpin, NEmpty } from 'naive-ui'
import { useQueryStore } from '../../stores/query'
import { useConnectionStore } from '../../stores/connection'
import type { QueryResult } from '../../types'

const queryStore = useQueryStore()
const connectionStore = useConnectionStore()

const activeTab = ref<'structure' | 'data'>('structure')
const tableData = ref<QueryResult | null>(null)
const loadingData = ref(false)
const dataError = ref<string | null>(null)

// Reset data when switching to another table
watch(() => queryStore.tableKey, () => {
  tableData.value = null
  dataError.value = null
  activeTab.value = 'structure'
})

async function queryTableData() {
  if (!queryStore.selectedTable) return
  if (!connectionStore.activeConnectionId) return

  const { schema, table_name } = queryStore.selectedTable
  const sql = `SELECT * FROM "${schema}"."${table_name}" LIMIT 1000`

  loadingData.value = true
  dataError.value = null
  activeTab.value = 'data'

  try {
    const result = await queryStore.executeQuery(
      connectionStore.activeConnectionId,
      sql,
      1000
    )
    if (result) {
      tableData.value = result
    }
  } catch (err) {
    dataError.value = err instanceof Error ? err.message : '查询失败'
  } finally {
    loadingData.value = false
  }
}

function closeDetail() {
  queryStore.closeTableDetail()
}

function formatValue(value: any): string {
  if (value === null || value === undefined) {
    return 'NULL'
  }
  if (typeof value === 'object') {
    return JSON.stringify(value)
  }
  return String(value)
}
</script>

<template>
  <div class="table-detail-card">
    <div class="detail-header">
      <div class="header-title">
        <h3 class="table-name">
          <span class="schema-name">{{ queryStore.selectedTable?.schema }}</span>
          <span class="separator">.</span>
          <span class="name">{{ queryStore.selectedTable?.table_name }}</span>
        </h3>
      </div>
      <div class="header-actions">
        <n-button type="primary" @click="queryTableData">
          查询表数据
        </n-button>
        <button class="btn btn-close" @click="closeDetail" title="关闭">
          ✕
        </button>
      </div>
    </div>

    <n-tabs v-model:value="activeTab" type="line" animated>
      <n-tab-pane name="structure" tab="表结构">
        <div class="tab-content">
          <div class="section">
            <h4 class="section-title">列信息</h4>
            <div class="table-container">
              <table class="columns-table">
                <thead>
                  <tr>
                    <th class="col-name">列名</th>
                    <th class="col-type">数据类型</th>
                    <th class="col-nullable">可空</th>
                    <th class="col-default">默认值</th>
                    <th class="col-pk">主键</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="column in queryStore.selectedTable?.columns"
                    :key="column.name"
                    :class="{ 'is-pk': column.is_primary_key }"
                  >
                    <td class="col-name">
                      <span class="column-name">{{ column.name }}</span>
                    </td>
                    <td class="col-type">
                      <span class="data-type">{{ column.data_type }}</span>
                    </td>
                    <td class="col-nullable">
                      <span class="nullable-badge" :class="{ 'is-nullable': column.is_nullable }">
                        {{ column.is_nullable ? '是' : '否' }}
                      </span>
                    </td>
                    <td class="col-default">
                      <span class="default-value" :title="column.column_default">
                        {{ column.column_default || '-' }}
                      </span>
                    </td>
                    <td class="col-pk">
                      <span v-if="column.is_primary_key" class="pk-badge" title="主键">
                        🔑
                      </span>
                      <span v-else>-</span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <div class="section">
            <h4 class="section-title">统计信息</h4>
            <div class="stats-grid">
              <div class="stat-item">
                <span class="stat-label">总列数</span>
                <span class="stat-value">{{ queryStore.selectedTable?.columns.length || 0 }}</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">主键列</span>
                <span class="stat-value">
                  {{ queryStore.selectedTable?.columns.filter(c => c.is_primary_key).length || 0 }}
                </span>
              </div>
              <div class="stat-item">
                <span class="stat-label">可空列</span>
                <span class="stat-value">
                  {{ queryStore.selectedTable?.columns.filter(c => c.is_nullable).length || 0 }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </n-tab-pane>

      <n-tab-pane name="data" tab="表数据">
        <div class="tab-content">
          <n-spin v-if="loadingData" description="加载中..." />

          <div v-else-if="dataError" class="error-message">
            {{ dataError }}
          </div>

          <div v-else-if="tableData?.success" class="data-result">
            <div class="result-stats">
              <span class="stat">
                行数: <strong>{{ tableData.row_count }}</strong>
              </span>
              <span class="stat">
                耗时: <strong>{{ tableData.execution_time_ms }}ms</strong>
              </span>
            </div>
            <div class="table-wrapper">
              <table class="data-table">
                <thead>
                  <tr>
                    <th
                      v-for="column in tableData.columns"
                      :key="column.name"
                      :title="column.data_type"
                    >
                      {{ column.name }}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(row, rowIndex) in tableData.rows" :key="rowIndex">
                    <td
                      v-for="(cell, cellIndex) in row"
                      :key="cellIndex"
                      :class="{ 'null-value': cell === null }"
                    >
                      {{ formatValue(cell) }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <n-empty v-else description="点击上方「查询表数据」按钮加载数据" />
        </div>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>
.table-detail-card {
  background: var(--card-bg, white);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color, #eee);
  background-color: var(--bg-color, #fafafa);
}

.header-title {
  flex: 1;
  min-width: 0;
}

.table-name {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-color, #333);
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.schema-name {
  color: var(--text-color-secondary, #666);
  font-weight: 500;
}

.separator {
  color: var(--text-color-disabled, #999);
}

.name {
  color: var(--text-color, #333);
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-close {
  width: 32px;
  height: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: transparent;
  color: var(--text-color-disabled, #999);
  font-size: 18px;
}

.btn-close:hover {
  background-color: var(--hover-bg, #f5f5f5);
  color: var(--text-color, #333);
}

.tab-content {
  padding: 20px 24px;
}

.section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color-secondary, #666);
  margin: 0 0 12px 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.table-container {
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  overflow: hidden;
}

.columns-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.columns-table th {
  background-color: var(--table-header-bg, #f5f5f5);
  padding: 12px 16px;
  text-align: left;
  font-weight: 600;
  color: var(--text-color, #555);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.columns-table td {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color-light, #f0f0f0);
}

.columns-table tr:last-child td {
  border-bottom: none;
}

.columns-table tr:hover {
  background-color: var(--hover-bg, #fafafa);
}

.columns-table tr.is-pk {
  background-color: var(--pk-bg, #fff8e1);
}

.columns-table tr.is-pk:hover {
  background-color: var(--pk-bg-hover, #ffecb3);
}

.col-name {
  width: 25%;
}

.col-type {
  width: 20%;
}

.col-nullable {
  width: 10%;
  text-align: center;
}

.col-default {
  width: 30%;
}

.col-pk {
  width: 10%;
  text-align: center;
}

.column-name {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-weight: 500;
  color: var(--text-color, #333);
}

.data-type {
  color: var(--text-color-secondary, #666);
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.nullable-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  background-color: var(--badge-bg, #e0e0e0);
  color: var(--text-color-secondary, #666);
}

.nullable-badge.is-nullable {
  background-color: var(--badge-primary-bg, #e3f2fd);
  color: var(--primary-color, #1976D2);
}

.default-value {
  color: var(--text-color-disabled, #888);
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
}

.pk-badge {
  font-size: 14px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.stat-item {
  background-color: var(--stat-bg, #f5f5f5);
  padding: 16px;
  border-radius: 6px;
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 12px;
  color: var(--text-color-secondary, #666);
  margin-bottom: 4px;
}

.stat-value {
  display: block;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-color, #333);
}

/* 表数据 Tab 样式 */
.data-result {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.result-stats {
  display: flex;
  gap: 16px;
  font-size: 14px;
  color: var(--text-color-secondary, #666);
}

.result-stats .stat strong {
  color: var(--text-color, #333);
}

.table-wrapper {
  overflow: auto;
  max-height: 400px;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.data-table th,
.data-table td {
  padding: 10px 12px;
  text-align: left;
  border-bottom: 1px solid var(--border-color-light, #eee);
  white-space: nowrap;
}

.data-table th {
  background-color: var(--table-header-bg, #f5f5f5);
  font-weight: 600;
  color: var(--text-color, #333);
  position: sticky;
  top: 0;
  z-index: 1;
}

.data-table tbody tr:hover {
  background-color: var(--hover-bg, #f8f9fa);
}

.null-value {
  color: #999;
  font-style: italic;
}

.error-message {
  padding: 20px;
  background-color: var(--error-bg, #ffebee);
  color: var(--error-color, #c62828);
  border-radius: 6px;
  font-size: 14px;
}

@media (max-width: 640px) {
  .detail-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .header-actions {
    width: 100%;
    justify-content: space-between;
  }

  .columns-table {
    font-size: 12px;
  }

  .columns-table th,
  .columns-table td {
    padding: 8px 10px;
  }

  .col-default {
    max-width: 100px;
  }
}
</style>
