<script setup lang="ts">
import { useConnectionStore } from '../../stores/connection'

const connectionStore = useConnectionStore()
</script>

<template>
  <header class="app-header">
    <div class="header-left">
      <h1 class="app-title">ConData</h1>
      <span class="app-subtitle">数据库连接器</span>
    </div>
    
    <nav class="header-nav">
      <router-link to="/" class="nav-link" active-class="active">
        首页
      </router-link>
      <router-link to="/connections" class="nav-link" active-class="active">
        连接管理
      </router-link>
      <router-link to="/query" class="nav-link" active-class="active">
        查询执行
      </router-link>
    </nav>

    <div class="header-right">
      <div v-if="connectionStore.activeConnection" class="connection-info">
        <span class="connection-name">{{ connectionStore.activeConnection.name }}</span>
        <span 
          class="status-dot"
          :class="{ connected: connectionStore.isConnected }"
        ></span>
      </div>
      <div v-else class="no-connection">
        未选择连接
      </div>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 0 24px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.app-title {
  font-size: 24px;
  font-weight: 700;
  margin: 0;
}

.app-subtitle {
  font-size: 14px;
  opacity: 0.9;
}

.header-nav {
  display: flex;
  gap: 8px;
}

.nav-link {
  color: white;
  text-decoration: none;
  padding: 8px 16px;
  border-radius: 4px;
  transition: background-color 0.2s;
  font-size: 14px;
}

.nav-link:hover {
  background-color: rgba(255,255,255,0.1);
}

.nav-link.active {
  background-color: rgba(255,255,255,0.2);
  font-weight: 500;
}

.header-right {
  display: flex;
  align-items: center;
}

.connection-info {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: rgba(255,255,255,0.1);
  padding: 6px 12px;
  border-radius: 16px;
}

.connection-name {
  font-size: 14px;
  font-weight: 500;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #f44336;
}

.status-dot.connected {
  background-color: #4CAF50;
}

.no-connection {
  font-size: 14px;
  opacity: 0.7;
}
</style>
