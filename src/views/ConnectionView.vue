<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { ConnectionConfig } from '../types'
import { useConnectionStore } from '../stores/connection'
import { useFeedback } from '../composables/useFeedback'
import ConnectionForm from '../components/connection/ConnectionForm.vue'
import ConnectionList from '../components/connection/ConnectionList.vue'

const connectionStore = useConnectionStore()
const { msg, confirm } = useFeedback()

const showForm = ref(false)
const editingConnection = ref<ConnectionConfig | null>(null)

onMounted(() => {
  connectionStore.loadConnections()
})

function handleNewConnection() {
  editingConnection.value = null
  showForm.value = true
}

function handleEditConnection(connection: ConnectionConfig) {
  editingConnection.value = connection
  showForm.value = true
}

function handleSaveConnection(_connection: ConnectionConfig) {
  showForm.value = false
  editingConnection.value = null
}

function handleCancel() {
  showForm.value = false
  editingConnection.value = null
}

function handleDeleteConnection(id: string) {
  confirm.warning({
    title: '删除连接',
    content: '确定要删除这个连接吗？此操作不可撤销。',
    positiveText: '删除',
    negativeText: '取消',
    onPositive: async () => {
      try {
        await connectionStore.deleteConnection(id)
        msg.success('连接已删除')
      } catch (err) {
        msg.error('删除失败: ' + (err instanceof Error ? err.message : '未知错误'))
      }
    },
  })
}

async function handleConnect(id: string) {
  try {
    const status = await connectionStore.connect(id)
    if (status.connected) {
      msg.success('连接成功！')
    } else {
      msg.error('连接失败: ' + status.message)
    }
  } catch (err) {
    msg.error('连接失败: ' + (err instanceof Error ? err.message : '未知错误'))
  }
}
</script>

<template>
  <div class="connection-view">
    <div class="view-header">
      <h2 class="view-title">连接管理</h2>
      <n-button v-if="!showForm" type="primary" @click="handleNewConnection">
        <template #icon>
          <n-icon><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg></n-icon>
        </template>
        新建连接
      </n-button>
    </div>

    <n-alert v-if="connectionStore.error" type="error" closable @close="connectionStore.clearError()">
      {{ connectionStore.error }}
    </n-alert>

    <div class="view-content">
      <div v-if="showForm" class="form-section">
        <ConnectionForm
          :edit-connection="editingConnection"
          @saved="handleSaveConnection"
          @cancelled="handleCancel"
        />
      </div>

      <div class="list-section">
        <ConnectionList
          :connections="connectionStore.connections"
          @edit="handleEditConnection"
          @delete="handleDeleteConnection"
          @connect="handleConnect"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.connection-view {
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
  margin: 0;
}

.view-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-section {
  animation: slideDown 0.3s ease;
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
</style>
