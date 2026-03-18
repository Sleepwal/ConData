<script setup lang="ts">
import { computed } from 'vue'
import { useQueryStore } from '../../stores/query'
import { useFeedback } from '../../composables/useFeedback'

const queryStore = useQueryStore()
const { msg, confirm } = useFeedback()

const hasHistory = computed(() => queryStore.queryHistory.length > 0)

function formatTime(isoString: string): string {
  const date = new Date(isoString)
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

function useQuery(sql: string) {
  queryStore.setCurrentSql(sql)
}

function clearHistory() {
  confirm.warning({
    title: '清空历史',
    content: '确定要清空查询历史吗？此操作不可撤销。',
    positiveText: '清空',
    negativeText: '取消',
    onPositive: () => {
      queryStore.clearHistory()
      msg.success('查询历史已清空')
    },
  })
}
</script>

<template>
  <n-card class="query-history" title="查询历史" size="small">
    <template #header-extra>
      <n-button v-if="hasHistory" size="small" type="error" text @click="clearHistory">
        <template #icon>
          <n-icon><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg></n-icon>
        </template>
        清空
      </n-button>
    </template>

    <n-empty v-if="!hasHistory" description="暂无查询历史" />

    <div v-else class="history-list">
      <n-space vertical :size="8" style="width: 100%">
        <n-card
          v-for="item in queryStore.queryHistory"
          :key="item.id"
          size="small"
          :class="['history-item', { failed: !item.success }]"
          hoverable
          @click="useQuery(item.sql)"
        >
          <template #header>
            <n-tooltip placement="top" :style="{ maxWidth: '300px' }">
              <template #trigger>
                <code class="history-sql sql-ellipsis">{{ item.sql }}</code>
              </template>
              {{ item.sql }}
            </n-tooltip>
          </template>

          <n-space justify="space-between" size="small">
            <span class="history-time">{{ formatTime(item.executed_at) }}</span>
            <n-space size="small">
              <n-tag size="small" :type="item.success ? 'success' : 'error'">{{ item.row_count }} 行</n-tag>
              <n-tag size="small" type="info">{{ item.execution_time_ms }}ms</n-tag>
            </n-space>
          </n-space>
        </n-card>
      </n-space>
    </div>
  </n-card>
</template>

<style scoped>
.query-history {
  max-height: 400px;
  display: flex;
  flex-direction: column;
}

.history-list {
  overflow-y: auto;
  max-height: 320px;
}

.history-item {
  cursor: pointer;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.history-item:hover {
  border-color: #667eea;
}

.history-item.failed {
  border-left-color: #f44336;
}

.history-item.failed:hover {
  border-left-color: #f44336;
}

.history-sql {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
}

.history-time {
  font-size: 12px;
}

.sql-ellipsis {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}
</style>
