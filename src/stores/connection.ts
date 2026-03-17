import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ConnectionConfig, ConnectionStatus, TestConnectionRequest } from '../types'
import { connectionApi } from '../api'

export const useConnectionStore = defineStore('connection', () => {
  const connections = ref<ConnectionConfig[]>([])
  const activeConnectionId = ref<string | null>(null)
  const connectionStatuses = ref<Map<string, ConnectionStatus>>(new Map())
  const loading = ref(false)
  const error = ref<string | null>(null)

  const activeConnection = computed(() => {
    if (!activeConnectionId.value) return null
    return connections.value.find(c => c.id === activeConnectionId.value) || null
  })

  const isConnected = computed(() => {
    if (!activeConnectionId.value) return false
    const status = connectionStatuses.value.get(activeConnectionId.value)
    return status?.connected || false
  })

  async function loadConnections() {
    loading.value = true
    error.value = null
    try {
      connections.value = await connectionApi.getConnections()
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载连接失败'
      console.error('Failed to load connections:', err)
    } finally {
      loading.value = false
    }
  }

  async function saveConnection(config: ConnectionConfig) {
    loading.value = true
    error.value = null
    try {
      const saved = await connectionApi.saveConnection(config)
      const index = connections.value.findIndex(c => c.id === saved.id)
      if (index >= 0) {
        connections.value[index] = saved
      } else {
        connections.value.push(saved)
      }
      return saved
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存连接失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function deleteConnection(id: string) {
    loading.value = true
    error.value = null
    try {
      await connectionApi.deleteConnection(id)
      connections.value = connections.value.filter(c => c.id !== id)
      if (activeConnectionId.value === id) {
        activeConnectionId.value = null
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除连接失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function testConnection(request: TestConnectionRequest) {
    loading.value = true
    error.value = null
    try {
      const status = await connectionApi.testConnection(request)
      return status
    } catch (err) {
      error.value = err instanceof Error ? err.message : '测试连接失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function connect(connectionId: string) {
    loading.value = true
    error.value = null
    try {
      const status = await connectionApi.connect(connectionId)
      connectionStatuses.value.set(connectionId, status)
      if (status.connected) {
        activeConnectionId.value = connectionId
      }
      return status
    } catch (err) {
      error.value = err instanceof Error ? err.message : '连接失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function disconnect(connectionId: string) {
    loading.value = true
    error.value = null
    try {
      const status = await connectionApi.disconnect(connectionId)
      connectionStatuses.value.set(connectionId, status)
      if (activeConnectionId.value === connectionId) {
        activeConnectionId.value = null
      }
      return status
    } catch (err) {
      error.value = err instanceof Error ? err.message : '断开连接失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function checkConnectionStatus(connectionId: string) {
    try {
      const status = await connectionApi.getConnectionStatus(connectionId)
      connectionStatuses.value.set(connectionId, status)
      return status
    } catch (err) {
      console.error('Failed to check connection status:', err)
      return null
    }
  }

  function setActiveConnection(connectionId: string | null) {
    activeConnectionId.value = connectionId
  }

  function clearError() {
    error.value = null
  }

  return {
    connections,
    activeConnectionId,
    connectionStatuses,
    loading,
    error,
    activeConnection,
    isConnected,
    loadConnections,
    saveConnection,
    deleteConnection,
    testConnection,
    connect,
    disconnect,
    checkConnectionStatus,
    setActiveConnection,
    clearError
  }
})
