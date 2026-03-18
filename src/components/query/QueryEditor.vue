<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useQueryStore } from '../../stores/query'
import { useConnectionStore } from '../../stores/connection'
import { useFeedback } from '../../composables/useFeedback'

const queryStore = useQueryStore()
const connectionStore = useConnectionStore()
const { msg } = useFeedback()

const sql = ref('')
const limit = ref(1000)

// Watch for changes from store (e.g., from TableDetail component)
watch(() => queryStore.currentSql, (newSql) => {
  if (newSql && newSql !== sql.value) {
    sql.value = newSql
  }
})

const canExecute = computed(() => {
  return sql.value.trim().length > 0 && connectionStore.isConnected
})

// Sync local sql changes to store
watch(sql, (newValue) => {
  if (newValue !== queryStore.currentSql) {
    queryStore.currentSql = newValue
  }
})

async function executeQuery() {
  if (!connectionStore.activeConnectionId) {
    msg.warning('请先选择一个连接')
    return
  }

  await queryStore.executeQuery(connectionStore.activeConnectionId, sql.value, limit.value)
}

function handleKeydown(e: KeyboardEvent) {
  if (e.ctrlKey && e.key === 'Enter') {
    e.preventDefault()
    executeQuery()
  }
}
</script>

<template>
  <n-card class="query-editor" title="SQL 编辑器">
    <template #header-extra>
      <n-space>
        <n-input-number v-model:value="limit" :min="1" :max="10000" style="width: 120px">
          <template #prefix>限制</template>
        </n-input-number>
        <n-button type="primary" :disabled="!canExecute || queryStore.loading" @click="executeQuery">
          <template #icon>
            <n-icon><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polygon points="5 3 19 12 5 21 5 3"/>
            </svg></n-icon>
          </template>
          {{ queryStore.loading ? '执行中...' : '执行 (Ctrl+Enter)' }}
        </n-button>
      </n-space>
    </template>

    <n-input
      v-model:value="sql"
      type="textarea"
      placeholder="在此输入 SQL 查询语句...&#10;按 Ctrl+Enter 快速执行"
      :disabled="!connectionStore.isConnected"
      :autosize="{ minRows: 8, maxRows: 15 }"
      class="sql-input"
      @keydown="handleKeydown"
    />

    <n-alert v-if="!connectionStore.isConnected" type="warning" :show-icon="false" style="margin-top: 12px">
      请先连接到数据库以执行查询
    </n-alert>
  </n-card>
</template>

<style scoped>
.sql-input {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
}

.sql-input :deep(.n-input__textarea-el) {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}
</style>
