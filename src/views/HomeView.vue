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
      <div class="dashboard-card connections-card" @click="goToConnections">
        <div class="card-icon">🔌</div>
        <div class="card-content">
          <h3 class="card-title">连接管理</h3>
          <p class="card-value">{{ connectionCount }} 个连接</p>
          <p class="card-description">管理您的数据库连接配置</p>
        </div>
      </div>

      <div class="dashboard-card query-card" @click="goToQuery">
        <div class="card-icon">🔍</div>
        <div class="card-content">
          <h3 class="card-title">查询执行</h3>
          <p class="card-value">SQL 编辑器</p>
          <p class="card-description">执行 SQL 查询并查看结果</p>
        </div>
      </div>
    </div>

    <div class="info-section">
      <div class="info-card current-connection">
        <h3 class="info-title">当前连接</h3>
        <div v-if="activeConnection" class="connection-details">
          <div class="detail-item">
            <span class="detail-label">名称:</span>
            <span class="detail-value">{{ activeConnection.name }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">主机:</span>
            <span class="detail-value">{{ activeConnection.host }}:{{ activeConnection.port }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">数据库:</span>
            <span class="detail-value">{{ activeConnection.database }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">状态:</span>
            <span class="detail-value" :class="{ connected: connectionStore.isConnected }">
              {{ connectionStore.isConnected ? '已连接' : '未连接' }}
            </span>
          </div>
        </div>
        <div v-else class="no-connection">
          <p>暂无活动连接</p>
          <button class="btn btn-primary" @click="goToConnections">去连接</button>
        </div>
      </div>

      <div class="info-card recent-queries">
        <h3 class="info-title">最近查询</h3>
        <div v-if="recentQueries.length > 0" class="queries-list">
          <div 
            v-for="query in recentQueries" 
            :key="query.id"
            class="query-item"
            :class="{ failed: !query.success }"
          >
            <div class="query-sql">{{ query.sql.substring(0, 50) }}{{ query.sql.length > 50 ? '...' : '' }}</div>
            <div class="query-meta">
              <span class="query-time">{{ new Date(query.executed_at).toLocaleTimeString() }}</span>
              <span class="query-stats">{{ query.row_count }} 行 | {{ query.execution_time_ms }}ms</span>
            </div>
          </div>
        </div>
        <div v-else class="no-queries">
          <p>暂无查询记录</p>
        </div>
      </div>
    </div>
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
  color: #333;
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
  background: white;
  border-radius: 12px;
  padding: 32px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 24px;
}

.dashboard-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.card-icon {
  font-size: 48px;
}

.card-content {
  flex: 1;
}

.card-title {
  font-size: 20px;
  font-weight: 600;
  color: #333;
  margin: 0 0 8px 0;
}

.card-value {
  font-size: 24px;
  font-weight: 700;
  color: #667eea;
  margin: 0 0 4px 0;
}

.card-description {
  font-size: 14px;
  color: #999;
  margin: 0;
}

.info-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 24px;
}

.info-card {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.info-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0 0 20px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid #eee;
}

.connection-details {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.detail-label {
  font-size: 14px;
  color: #666;
}

.detail-value {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.detail-value.connected {
  color: #4CAF50;
}

.no-connection {
  text-align: center;
  padding: 20px;
  color: #999;
}

.no-connection p {
  margin: 0 0 16px 0;
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
  background-color: #667eea;
  color: white;
}

.btn-primary:hover {
  background-color: #5a6fd6;
}

.queries-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.query-item {
  padding: 12px;
  background-color: #f8f9fa;
  border-radius: 8px;
  border-left: 3px solid #667eea;
}

.query-item.failed {
  border-left-color: #f44336;
}

.query-sql {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  color: #333;
  margin-bottom: 8px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.query-meta {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #999;
}

.query-stats {
  color: #667eea;
}

.no-queries {
  text-align: center;
  padding: 20px;
  color: #999;
}
</style>
