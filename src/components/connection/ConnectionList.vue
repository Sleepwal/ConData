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
  <n-card title="已保存的连接" class="connection-list">
    <n-empty v-if="connections.length === 0" description="暂无保存的连接">
      <template #extra>
        <span class="text-secondary">点击上方"新建连接"按钮添加</span>
      </template>
    </n-empty>

    <n-space v-else vertical :size="12">
      <n-card
        v-for="connection in connections"
        :key="connection.id"
        size="small"
        :class="['connection-item', { active: isActive(connection.id) }]"
        hoverable
      >
        <n-space justify="space-between" align="center" style="width: 100%">
          <n-space vertical :size="4">
            <n-space align="center" :size="12">
              <!-- 状态圆点 -->
              <span
                class="status-dot"
                :class="{ connected: getStatus(connection.id)?.connected }"
              />
              <strong class="connection-name">{{ connection.name }}</strong>
              <n-tag
                v-if="getStatus(connection.id)?.connected"
                size="small"
                type="success"
              >
                已连接
              </n-tag>
              <n-tag v-else size="small" type="default">
                未连接
              </n-tag>
            </n-space>
            <span class="connection-details text-secondary">
              {{ connection.host }}:{{ connection.port }} | {{ connection.database }} | {{ connection.username }}
            </span>
          </n-space>

          <n-space>
            <n-button
              v-if="!getStatus(connection.id)?.connected"
              size="small"
              type="primary"
              :loading="connectionStore.loading"
              @click="emit('connect', connection.id)"
            >
              连接
            </n-button>
            <n-button
              v-else
              size="small"
              type="warning"
              :loading="connectionStore.loading"
              @click="connectionStore.disconnect(connection.id)"
            >
              断开
            </n-button>
            <n-button
              size="small"
              :loading="connectionStore.loading"
              @click="emit('edit', connection)"
            >
              编辑
            </n-button>
            <n-button
              size="small"
              type="error"
              ghost
              :loading="connectionStore.loading"
              @click="emit('delete', connection.id)"
            >
              删除
            </n-button>
          </n-space>
        </n-space>
      </n-card>
    </n-space>
  </n-card>
</template>

<style scoped>
.connection-list {
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.connection-item {
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.connection-item:hover {
  border-left-color: #667eea;
}

.connection-item.active {
  border-left-color: #4CAF50;
  background-color: rgba(76, 175, 80, 0.05);
}

.connection-name {
  font-size: 16px;
}

.connection-details {
  font-size: 13px;
}

.status-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #ccc;
}

.status-dot.connected {
  background-color: #18a058;
}

.text-secondary {
  color: var(--n-text-color-disabled, #999);
}
</style>
