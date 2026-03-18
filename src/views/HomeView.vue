<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useConnectionStore } from '../stores/connection'
import { useQueryStore } from '../stores/query'

const router = useRouter()
const connectionStore = useConnectionStore()
const queryStore = useQueryStore()

const connectionCount = computed(() => connectionStore.connections.length)
const activeConnection = computed(() => connectionStore.activeConnection)
const recentQueries = computed(() => queryStore.queryHistory.slice(0, 5))

onMounted(() => {
  connectionStore.loadConnections()
})

function goToConnections() {
  router.push('/connections')
}

function goToQuery() {
  router.push('/query')
}
</script>

<template>
  <div class="home-view">
    <div class="welcome-section">
      <h2 class="welcome-title">欢迎使用 ConData</h2>
      <p class="welcome-subtitle">PostgreSQL 数据库连接器</p>
    </div>

    <div class="dashboard-grid">
      <n-card hoverable class="dashboard-card" @click="goToConnections">
        <n-space align="center" size="large">
          <span class="card-icon">🔌</span>
          <div class="card-content">
            <n-h3 class="card-title">连接管理</n-h3>
            <span class="card-value">{{ connectionCount }} 个连接</span>
            <span class="card-description text-secondary">管理您的数据库连接配置</span>
          </div>
        </n-space>
      </n-card>

      <n-card hoverable class="dashboard-card" @click="goToQuery">
        <n-space align="center" size="large">
          <span class="card-icon">🔍</span>
          <div class="card-content">
            <n-h3 class="card-title">查询执行</n-h3>
            <span class="card-value">SQL 编辑器</span>
            <span class="card-description text-secondary">执行 SQL 查询并查看结果</span>
          </div>
        </n-space>
      </n-card>
    </div>

    <n-grid :cols="2" :x-gap="24" responsive="screen">
      <n-grid-item>
        <n-card title="当前连接">
          <div v-if="activeConnection" class="connection-details">
            <n-space vertical size="small">
              <n-space justify="space-between">
                <span class="text-secondary">名称:</span>
                <strong>{{ activeConnection.name }}</strong>
              </n-space>
              <n-space justify="space-between">
                <span class="text-secondary">主机:</span>
                <strong>{{ activeConnection.host }}:{{ activeConnection.port }}</strong>
              </n-space>
              <n-space justify="space-between">
                <span class="text-secondary">数据库:</span>
                <strong>{{ activeConnection.database }}</strong>
              </n-space>
              <n-space justify="space-between">
                <span class="text-secondary">状态:</span>
                <n-tag :type="connectionStore.isConnected ? 'success' : 'default'" size="small">
                  {{ connectionStore.isConnected ? '已连接' : '未连接' }}
                </n-tag>
              </n-space>
            </n-space>
          </div>
          <n-empty v-else description="暂无活动连接">
            <template #extra>
              <n-button type="primary" @click="goToConnections">去连接</n-button>
            </template>
          </n-empty>
        </n-card>
      </n-grid-item>

      <n-grid-item>
        <n-card title="最近查询">
          <div v-if="recentQueries.length > 0" class="queries-list">
            <n-card
              v-for="query in recentQueries"
              :key="query.id"
              size="small"
              :class="['query-item', { failed: !query.success }]"
              hoverable
            >
              <n-tooltip placement="top" :style="{ maxWidth: '400px' }">
                <template #trigger>
                  <code class="query-sql sql-ellipsis">{{ query.sql }}</code>
                </template>
                {{ query.sql }}
              </n-tooltip>
              <n-space justify="space-between" size="small" style="margin-top: 8px">
                <span class="query-time text-secondary">{{ new Date(query.executed_at).toLocaleTimeString() }}</span>
                <n-space size="small">
                  <n-tag size="small" :type="query.success ? 'success' : 'error'">{{ query.row_count }} 行</n-tag>
                  <n-tag size="small">{{ query.execution_time_ms }}ms</n-tag>
                </n-space>
              </n-space>
            </n-card>
          </div>
          <n-empty v-else description="暂无查询记录" />
        </n-card>
      </n-grid-item>
    </n-grid>
  </div>
</template>

<style scoped>
.home-view {
  padding: 20px 0;
}

.welcome-section {
  text-align: center;
  margin-bottom: 40px;
}

.welcome-title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 8px 0;
}

.welcome-subtitle {
  font-size: 18px;
  color: #666;
  margin: 0;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
  margin-bottom: 40px;
}

.dashboard-card {
  cursor: pointer;
  transition: all 0.3s;
}

.dashboard-card:hover {
  transform: translateY(-4px);
}

.card-icon {
  font-size: 48px;
}

.card-content {
  display: flex;
  flex-direction: column;
}

.card-title {
  margin: 0 0 8px 0;
  font-size: 20px;
}

.card-value {
  font-size: 24px;
  color: #667eea;
  margin-bottom: 4px;
}

.card-description {
  font-size: 14px;
}

.connection-details {
  padding: 8px 0;
}

.queries-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-height: 400px;
  overflow-y: auto;
}

.query-item {
  border-left: 3px solid #667eea;
}

.query-item.failed {
  border-left-color: #f44336;
}

.query-sql {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
}

.sql-ellipsis {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.query-time {
  font-size: 12px;
}

.text-secondary {
  color: var(--n-text-color-disabled, #999);
}
</style>
