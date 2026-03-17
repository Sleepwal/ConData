<script setup lang="ts">
import { computed } from 'vue'
import { useQueryStore } from '../../stores/query'

const queryStore = useQueryStore()

const hasResult = computed(() => queryStore.queryResult !== null)

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
  <div class="query-result">
    <div class="result-header">
      <h3 class="result-title">查询结果</h3>
      <div v-if="hasResult && queryStore.queryResult?.success" class="result-stats">
        <span class="stat">
          行数: <strong>{{ queryStore.queryResult?.row_count }}</strong>
        </span>
        <span class="stat">
          耗时: <strong>{{ queryStore.queryResult?.execution_time_ms }}ms</strong>
        </span>
      </div>
    </div>

    <div v-if="queryStore.loading" class="loading">
      执行中...
    </div>

    <div v-else-if="queryStore.error" class="error-message">
      {{ queryStore.error }}
    </div>

    <div v-else-if="hasResult && !queryStore.queryResult?.success" class="error-message">
      {{ queryStore.queryResult?.message || '查询执行失败' }}
    </div>

    <div v-else-if="hasResult && queryStore.queryResult?.success" class="result-container">
      <div v-if="queryStore.queryResult?.columns.length === 0" class="empty-result">
        {{ queryStore.queryResult?.message || '查询成功，无返回数据' }}
      </div>
      
      <div v-else class="table-wrapper">
        <table class="result-table">
          <thead>
            <tr>
              <th 
                v-for="column in queryStore.queryResult?.columns" 
                :key="column.name"
                :title="column.data_type"
              >
                {{ column.name }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, rowIndex) in queryStore.queryResult?.rows" :key="rowIndex">
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

    <div v-else class="no-result">
      <p>执行查询以查看结果</p>
    </div>
  </div>
</template>

<style scoped>
.query-result {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.result-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.result-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.result-stats {
  display: flex;
  gap: 16px;
  font-size: 14px;
  color: #666;
}

.stat strong {
  color: #333;
}

.loading {
  padding: 40px;
  text-align: center;
  color: #666;
}

.error-message {
  padding: 20px;
  background-color: #ffebee;
  color: #c62828;
  font-size: 14px;
}

.no-result {
  padding: 60px 20px;
  text-align: center;
  color: #999;
}

.empty-result {
  padding: 40px 20px;
  text-align: center;
  color: #666;
  font-size: 14px;
}

.result-container {
  overflow: auto;
  max-height: 500px;
}

.table-wrapper {
  min-width: 100%;
}

.result-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.result-table th,
.result-table td {
  padding: 10px 12px;
  text-align: left;
  border-bottom: 1px solid #eee;
  white-space: nowrap;
}

.result-table th {
  background-color: #f5f5f5;
  font-weight: 600;
  color: #333;
  position: sticky;
  top: 0;
  z-index: 1;
}

.result-table tbody tr:hover {
  background-color: #f8f9fa;
}

.null-value {
  color: #999;
  font-style: italic;
}
</style>
