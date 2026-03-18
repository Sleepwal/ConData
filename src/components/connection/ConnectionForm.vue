<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import type { ConnectionConfig, TestConnectionRequest } from '../../types'
import { SslMode } from '../../types'
import { useConnectionStore } from '../../stores/connection'
import { useFeedback } from '../../composables/useFeedback'

const connectionStore = useConnectionStore()
const { msg } = useFeedback()

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
    msg.success('连接保存成功')
    emit('saved', saved)
  } catch (err) {
    msg.error('保存失败: ' + (err instanceof Error ? err.message : '未知错误'))
  }
}

function validateForm(): boolean {
  if (!form.name.trim()) {
    msg.warning('请输入连接名称')
    return false
  }
  if (!form.host.trim()) {
    msg.warning('请输入主机地址')
    return false
  }
  if (!form.database.trim()) {
    msg.warning('请输入数据库名称')
    return false
  }
  if (!form.username.trim()) {
    msg.warning('请输入用户名')
    return false
  }
  return true
}

function cancel() {
  emit('cancelled')
}
</script>

<template>
  <n-card class="connection-form" :title="editConnection ? '编辑连接' : '新建连接'">
    <n-alert
      v-if="testResult"
      :type="testResult.success ? 'success' : 'error'"
      closable
      @close="testResult = null"
    >
      {{ testResult.message }}
    </n-alert>

    <n-form label-placement="left" label-width="100" require-mark-placement="right-hanging">
      <n-grid :cols="2" :x-gap="16">
        <n-grid-item :span="2">
          <n-form-item label="连接名称" required>
            <n-input v-model:value="form.name" placeholder="例如：生产数据库" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="主机地址" required>
            <n-input v-model:value="form.host" placeholder="localhost" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="端口" required>
            <n-input-number v-model:value="form.port" :min="1" :max="65535" style="width: 100%" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="数据库名称" required>
            <n-input v-model:value="form.database" placeholder="postgres" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="SSL 模式">
            <n-select v-model:value="form.ssl_mode" :options="sslModes" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="用户名" required>
            <n-input v-model:value="form.username" placeholder="postgres" />
          </n-form-item>
        </n-grid-item>

        <n-grid-item>
          <n-form-item label="密码">
            <n-input v-model:value="form.password" type="password" placeholder="输入密码" show-password-on="click" />
          </n-form-item>
        </n-grid-item>
      </n-grid>

      <n-divider />

      <n-space justify="space-between">
        <n-button @click="testConnection" :loading="testing">
          <template #icon>
            <n-icon><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14M12 5l7 7-7 7"/></svg></n-icon>
          </template>
          测试连接
        </n-button>
        <n-space>
          <n-button @click="cancel">取消</n-button>
          <n-button type="primary" @click="saveConnection" :loading="connectionStore.loading">
            保存连接
          </n-button>
        </n-space>
      </n-space>
    </n-form>
  </n-card>
</template>

<style scoped>
.connection-form {
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
