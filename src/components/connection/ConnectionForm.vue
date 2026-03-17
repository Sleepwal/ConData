<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import type { ConnectionConfig, TestConnectionRequest } from '../../types'
import { SslMode } from '../../types'
import { useConnectionStore } from '../../stores/connection'

const connectionStore = useConnectionStore()

const props = defineProps<{
  editConnection?: ConnectionConfig | null
}>()

const emit = defineEmits<{
  saved: [connection: ConnectionConfig]
  cancelled: []
}>()

const form = reactive({
  id: '',
  name: '',
  host: 'localhost',
  port: 5432,
  database: '',
  username: '',
  password: '',
  ssl_mode: SslMode.Prefer
})

const testing = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)

const sslModes = [
  { value: SslMode.Disable, label: '禁用' },
  { value: SslMode.Allow, label: '允许' },
  { value: SslMode.Prefer, label: '优先' },
  { value: SslMode.Require, label: '要求' }
]

// 监听 editConnection 变化，重置表单
watch(() => props.editConnection, (newVal) => {
  if (newVal) {
    form.id = newVal.id
    form.name = newVal.name
    form.host = newVal.host
    form.port = newVal.port
    form.database = newVal.database
    form.username = newVal.username
    form.password = newVal.password
    form.ssl_mode = newVal.ssl_mode
  } else {
    // 重置为默认值
    form.id = ''
    form.name = ''
    form.host = 'localhost'
    form.port = 5432
    form.database = ''
    form.username = ''
    form.password = ''
    form.ssl_mode = SslMode.Prefer
  }
  // 清空测试结果
  testResult.value = null
}, { immediate: true })

async function testConnection() {
  if (!validateForm()) return

  testing.value = true
  testResult.value = null

  try {
    const request: TestConnectionRequest = {
      host: form.host,
      port: form.port,
      database: form.database,
      username: form.username,
      password: form.password,
      ssl_mode: form.ssl_mode
    }

    const result = await connectionStore.testConnection(request)
    testResult.value = {
      success: result.connected,
      message: result.message
    }
  } catch (err) {
    testResult.value = {
      success: false,
      message: err instanceof Error ? err.message : '测试连接失败'
    }
  } finally {
    testing.value = false
  }
}

async function saveConnection() {
  if (!validateForm()) return

  const config: ConnectionConfig = {
    id: form.id || Date.now().toString(),
    name: form.name,
    host: form.host,
    port: form.port,
    database: form.database,
    username: form.username,
    password: form.password,
    ssl_mode: form.ssl_mode
  }

  try {
    const saved = await connectionStore.saveConnection(config)
    emit('saved', saved)
  } catch (err) {
    console.error('Failed to save connection:', err)
  }
}

function validateForm(): boolean {
  if (!form.name.trim()) {
    alert('请输入连接名称')
    return false
  }
  if (!form.host.trim()) {
    alert('请输入主机地址')
    return false
  }
  if (!form.database.trim()) {
    alert('请输入数据库名称')
    return false
  }
  if (!form.username.trim()) {
    alert('请输入用户名')
    return false
  }
  return true
}

function cancel() {
  emit('cancelled')
}
</script>

<template>
  <div class="connection-form">
    <h3 class="form-title">{{ editConnection ? '编辑连接' : '新建连接' }}</h3>

    <div v-if="testResult" class="test-result" :class="{ success: testResult.success, error: !testResult.success }">
      {{ testResult.message }}
    </div>

    <form @submit.prevent="saveConnection">
      <div class="form-row">
        <div class="form-group">
          <label>连接名称 *</label>
          <input v-model="form.name" type="text" class="form-control" placeholder="例如：生产数据库" required />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label>主机地址 *</label>
          <input v-model="form.host" type="text" class="form-control" placeholder="localhost" required />
        </div>
        <div class="form-group">
          <label>端口 *</label>
          <input v-model.number="form.port" type="number" class="form-control" placeholder="5432" required />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label>数据库名称 *</label>
          <input v-model="form.database" type="text" class="form-control" placeholder="postgres" required />
        </div>
        <div class="form-group">
          <label>SSL 模式</label>
          <select v-model="form.ssl_mode" class="form-control">
            <option v-for="mode in sslModes" :key="mode.value" :value="mode.value">
              {{ mode.label }}
            </option>
          </select>
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label>用户名 *</label>
          <input v-model="form.username" type="text" class="form-control" placeholder="postgres" required />
        </div>
        <div class="form-group">
          <label>密码</label>
          <input v-model="form.password" type="password" class="form-control" placeholder="输入密码" />
        </div>
      </div>

      <div class="form-actions">
        <button type="button" class="btn btn-secondary" @click="testConnection" :disabled="testing">
          {{ testing ? '测试中...' : '测试连接' }}
        </button>
        <div class="spacer"></div>
        <button type="button" class="btn" @click="cancel">取消</button>
        <button type="submit" class="btn btn-primary" :disabled="connectionStore.loading">
          {{ connectionStore.loading ? '保存中...' : '保存连接' }}
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.connection-form {
  background: white;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.form-title {
  margin: 0 0 20px 0;
  font-size: 18px;
  color: #333;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-group label {
  margin-bottom: 6px;
  font-size: 14px;
  font-weight: 500;
  color: #555;
}

.form-control {
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
}

.form-control:focus {
  outline: none;
  border-color: #4CAF50;
}

.test-result {
  padding: 12px;
  border-radius: 4px;
  margin-bottom: 16px;
  font-size: 14px;
}

.test-result.success {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.test-result.error {
  background-color: #ffebee;
  color: #c62828;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid #eee;
}

.spacer {
  flex: 1;
}

.btn {
  padding: 10px 20px;
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

.btn-secondary {
  background-color: #2196F3;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #1976D2;
}
</style>
