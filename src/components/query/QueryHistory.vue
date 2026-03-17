<script setup lang="ts">
import { computed } from 'vue'
import { useQueryStore } from '../../stores/query'

const queryStore = useQueryStore()

const hasHistory = computed(() => queryStore.queryHistory.length > 0)

function formatTime(isoString: string): string {
  const date = new Date(isoString)
  return date.toLocaleTimeString('zh-CN', { 
    hour: '2-digit', 
    minute: '2-digit', 
    second: '2-digit' 
  })
}

function truncateSql(sql: string, maxLength: number = 50): string {
  if (sql.length <= maxLength) return sql
  return sql.substring(0, maxLength) + '...'
}

function useQuery(sql: string) {
  queryStore.setCurrentSql(sql)
}

function clearHistory() {
  if (confirm('确定要清空查询历史吗？')) {
    queryStore.clearHistory()
  }
}
</script>

<template>
  <div class="query-history">
    <div class="history-header">
      <h3 class="history-title">查询历史</h3>
      <button 
        v-if="hasHistory"
        class="btn btn-sm btn-danger"
        @click="clearHistory"
      >
        清空
      </button>
    </div>

    <div v-if="!hasHistory" class="empty-history">
      暂无查询历史
    </div>

    <div v-else class="history-list">
      <div 
        v-for="item in queryStore.queryHistory" 
        :key="item.id"
        class="history-item"
        :class="{ failed: !item.success }"
        @click="useQuery(item.sql)"
      >
        <div class="history-sql" :title="item.sql">
          {{ truncateSql(item.sql) }}
        </div>
        <div class="history-meta">
          <span class="history-time">{{ formatTime(item.executed_at) }}</span>
          <span class="history-stats">
            {{ item.row_count }} 行 | {{ item.execution_time_ms }}ms
          </span>
          <span v-if="!item.success" class="history-status failed">失败</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.query-history {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.history-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.empty-history {
  padding: 40px 20px;
  text-align: center;
  color: #999;
  font-size: 14px;
}

.history-list {
  max-height: 300px;
  overflow-y: auto;
}

.history-item {
  padding: 12px 20px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.2s;
}

.history-item:hover {
  background-color: #f5f5f5;
}

.history-item.failed {
  border-left: 3px solid #f44336;
}

.history-sql {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #333;
  margin-bottom: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.history-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: #999;
}

.history-time {
  color: #666;
}

.history-stats {
  color: #2196F3;
}

.history-status.failed {
  color: #f44336;
  font-weight: 500;
}

.btn {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
  background-color: #f5f5f5;
  color: #333;
}

.btn:hover {
  background-color: #e0e0e0;
}

.btn-danger {
  background-color: #f44336;
  color: white;
}

.btn-danger:hover {
  background-color: #d32f2f;
}
</style>
