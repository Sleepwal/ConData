<script setup lang="ts">
import { ref, computed } from 'vue'
import { useQueryStore } from '../../stores/query'
import { useConnectionStore } from '../../stores/connection'

const queryStore = useQueryStore()
const connectionStore = useConnectionStore()

const sql = ref('')
const limit = ref(1000)

const canExecute = computed(() => {
  return sql.value.trim().length > 0 && connectionStore.isConnected
})

async function executeQuery() {
  if (!connectionStore.activeConnectionId) {
    alert('请先选择一个连接')
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
  <div class="query-editor">
    <div class="editor-header">
      <h3 class="editor-title">SQL 编辑器</h3>
      <div class="editor-actions">
        <div class="limit-control">
          <label>结果限制:</label>
          <input 
            v-model.number="limit" 
            type="number" 
            class="limit-input"
            min="1"
            max="10000"
          />
        </div>
        <button 
          class="btn btn-primary execute-btn" 
          @click="executeQuery"
          :disabled="!canExecute || queryStore.loading"
        >
          <span v-if="queryStore.loading">执行中...</span>
          <span v-else>执行 (Ctrl+Enter)</span>
        </button>
      </div>
    </div>

    <div class="editor-container">
      <textarea
        v-model="sql"
        class="sql-input"
        placeholder="在此输入 SQL 查询语句...&#10;按 Ctrl+Enter 快速执行"
        @keydown="handleKeydown"
        :disabled="!connectionStore.isConnected"
      ></textarea>
    </div>

    <div v-if="!connectionStore.isConnected" class="connection-warning">
      请先连接到数据库以执行查询
    </div>
  </div>
</template>

<style scoped>
.query-editor {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #eee;
  background-color: #fafafa;
}

.editor-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.editor-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.limit-control {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #666;
}

.limit-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.execute-btn {
  padding: 8px 20px;
  font-size: 14px;
}

.editor-container {
  position: relative;
}

.sql-input {
  width: 100%;
  min-height: 200px;
  padding: 20px;
  border: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: vertical;
  outline: none;
  background-color: #f8f9fa;
  color: #333;
}

.sql-input:focus {
  background-color: #fff;
}

.sql-input:disabled {
  background-color: #f5f5f5;
  color: #999;
  cursor: not-allowed;
}

.connection-warning {
  padding: 12px 20px;
  background-color: #fff3e0;
  color: #e65100;
  font-size: 14px;
  text-align: center;
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

.btn-primary {
  background-color: #4CAF50;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #45a049;
}
</style>
