<script setup lang="ts">
import type { ConnectionConfig } from '../../types'
import { useConnectionStore } from '../../stores/connection'

const connectionStore = useConnectionStore()

defineProps<{
  connections: ConnectionConfig[]
}>()

const emit = defineEmits<{
  edit: [connection: ConnectionConfig]
  delete: [id: string]
  connect: [id: string]
}>()

function getStatus(connectionId: string) {
  return connectionStore.connectionStatuses.get(connectionId)
}

function isActive(connectionId: string) {
  return connectionStore.activeConnectionId === connectionId
}
</script>

<template>
  <div class="connection-list">
    <h3 class="list-title">已保存的连接</h3>

    <div v-if="connections.length === 0" class="empty-state">
      <p>暂无保存的连接</p>
      <p class="hint">点击上方"新建连接"按钮添加</p>
    </div>

    <div v-else class="connections">
      <div
        v-for="connection in connections"
        :key="connection.id"
        class="connection-item"
        :class="{ active: isActive(connection.id) }"
      >
        <div class="connection-info">
          <div class="connection-header">
            <h4 class="connection-name">{{ connection.name }}</h4>
            <span
              v-if="getStatus(connection.id)?.connected"
              class="status-badge connected"
            >
              已连接
            </span>
            <span v-else class="status-badge disconnected">
              未连接
            </span>
          </div>
          <div class="connection-details">
            <span>{{ connection.host }}:{{ connection.port }}</span>
            <span class="separator">|</span>
            <span>{{ connection.database }}</span>
            <span class="separator">|</span>
            <span>{{ connection.username }}</span>
          </div>
        </div>

        <div class="connection-actions">
          <button
            v-if="!getStatus(connection.id)?.connected"
            class="btn btn-sm btn-primary"
            @click="emit('connect', connection.id)"
            :disabled="connectionStore.loading"
          >
            连接
          </button>
          <button
            v-else
            class="btn btn-sm btn-secondary"
            @click="connectionStore.disconnect(connection.id)"
            :disabled="connectionStore.loading"
          >
            断开
          </button>
          <button
            class="btn btn-sm"
            @click="emit('edit', connection)"
            :disabled="connectionStore.loading"
          >
            编辑
          </button>
          <button
            class="btn btn-sm btn-danger"
            @click="emit('delete', connection.id)"
            :disabled="connectionStore.loading"
          >
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.connection-list {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.list-title {
  margin: 0 0 20px 0;
  font-size: 18px;
  color: #333;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #999;
}

.empty-state .hint {
  font-size: 14px;
  margin-top: 8px;
}

.connections {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.connection-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border: 1px solid #eee;
  border-radius: 8px;
  transition: all 0.2s;
}

.connection-item:hover {
  border-color: #ddd;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.connection-item.active {
  border-color: #4CAF50;
  background-color: #f8fff8;
}

.connection-info {
  flex: 1;
}

.connection-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.connection-name {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.connection-details {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #666;
}

.separator {
  color: #ccc;
}

.connection-actions {
  display: flex;
  gap: 8px;
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

.btn-secondary {
  background-color: #2196F3;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #1976D2;
}

.btn-danger {
  background-color: #f44336;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background-color: #d32f2f;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.connected {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.status-badge.disconnected {
  background-color: #ffebee;
  color: #c62828;
}
</style>
