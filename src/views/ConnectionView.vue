<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import type { ConnectionConfig } from '../types'
import { useConnectionStore } from '../stores/connection'
import ConnectionForm from '../components/connection/ConnectionForm.vue'
import ConnectionList from '../components/connection/ConnectionList.vue'

const router = useRouter()
const connectionStore = useConnectionStore()

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

function handleSaveConnection(connection: ConnectionConfig) {
  showForm.value = false
  editingConnection.value = null
}

function handleCancel() {
  showForm.value = false
  editingConnection.value = null
}

async function handleDeleteConnection(id: string) {
  if (confirm('确定要删除这个连接吗？')) {
    await connectionStore.deleteConnection(id)
  }
}

async function handleConnect(id: string) {
  const status = await connectionStore.connect(id)
  if (status.connected) {
    alert('连接成功！')
  } else {
    alert('连接失败: ' + status.message)
  }
}
</script>

<template>
  <div class="connection-view">
    <div class="view-header">
      <h2 class="view-title">连接管理</h2>
      <button v-if="!showForm" class="btn btn-primary" @click="handleNewConnection">
        + 新建连接
      </button>
    </div>

    <div v-if="connectionStore.error" class="error-message">
      {{ connectionStore.error }}
      <button class="btn-close" @click="connectionStore.clearError()">×</button>
    </div>

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
  color: #333;
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

.error-message {
  background-color: #ffebee;
  color: #c62828;
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.btn-close {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: #c62828;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-primary {
  background-color: #4CAF50;
  color: white;
}

.btn-primary:hover {
  background-color: #45a049;
}
</style>
