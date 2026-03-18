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
  <n-card class="query-result" title="查询结果">
    <template #header-extra>
      <n-space v-if="hasResult && queryStore.queryResult?.success">
        <n-tag type="info">{{ queryStore.queryResult?.row_count }} 行</n-tag>
        <n-tag>{{ queryStore.queryResult?.execution_time_ms }}ms</n-tag>
      </n-space>
    </template>

    <n-spin v-if="queryStore.loading" description="执行中..." />

    <n-alert v-else-if="queryStore.error" type="error" :show-icon="false">
      {{ queryStore.error }}
    </n-alert>

    <n-alert
      v-else-if="hasResult && !queryStore.queryResult?.success"
      type="error"
      :show-icon="false"
    >
      {{ queryStore.queryResult?.message || '查询执行失败' }}
    </n-alert>

    <template v-else-if="hasResult && queryStore.queryResult?.success">
      <n-empty
        v-if="queryStore.queryResult?.columns.length === 0"
        :description="queryStore.queryResult?.message || '查询成功，无返回数据'"
      />

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
    </template>

    <n-empty v-else description="执行查询以查看结果" />
  </n-card>
</template>

<style scoped>
.query-result {
  overflow: hidden;
}

.table-wrapper {
  overflow: auto;
  max-height: 500px;
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
  border-bottom: 1px solid var(--n-border-color);
  white-space: nowrap;
}

.result-table th {
  background-color: var(--n-th-color);
  font-weight: 600;
  color: var(--n-text-color);
  position: sticky;
  top: 0;
  z-index: 1;
}

.result-table tbody tr:hover {
  background-color: var(--n-td-color-hover);
}

.null-value {
  color: #999;
  font-style: italic;
}
</style>
