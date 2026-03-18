<script setup lang="ts">
import { h, computed } from 'vue'
import { useConnectionStore } from '../../stores/connection'
import { useThemeStore } from '../../stores/theme'
import { useFeedback } from '../../composables/useFeedback'

const connectionStore = useConnectionStore()
const themeStore = useThemeStore()
const { msg } = useFeedback()

// 生成下拉菜单选项
const connectionOptions = computed(() => {
  return connectionStore.connections.map(conn => {
    const status = connectionStore.connectionStatuses.get(conn.id)
    const isConnected = status?.connected || false

    return {
      key: conn.id,
      label: conn.name,
      icon: () => h('span', {
        class: ['dropdown-status-dot', { connected: isConnected }]
      }),
      description: `${conn.host}:${conn.port}`,
      disabled: connectionStore.activeConnectionId === conn.id && isConnected
    }
  })
})

// 处理连接选择
async function handleConnectionSelect(connectionId: string) {
  const conn = connectionStore.connections.find(c => c.id === connectionId)
  if (!conn) return

  const status = connectionStore.connectionStatuses.get(connectionId)
  const isConnected = status?.connected || false

  if (!isConnected) {
    // 未连接，先连接
    try {
      await connectionStore.connect(connectionId)
      msg.success(`已连接到 ${conn.name}`)
    } catch (err) {
      msg.error(`连接失败: ${err instanceof Error ? err.message : '未知错误'}`)
    }
  } else {
    // 已连接，切换活动连接
    connectionStore.setActiveConnection(connectionId)
  }
}
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
      <!-- 主题切换按钮 -->
      <n-button
        quaternary
        circle
        size="small"
        class="theme-toggle"
        @click="themeStore.toggleTheme"
      >
        <template #icon>
          <n-icon size="20">
            <!-- 太阳图标 (亮色模式时显示，点击切换到暗色) -->
            <svg v-if="!themeStore.isDark" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="5"/>
              <line x1="12" y1="1" x2="12" y2="3"/>
              <line x1="12" y1="21" x2="12" y2="23"/>
              <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
              <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
              <line x1="1" y1="12" x2="3" y2="12"/>
              <line x1="21" y1="12" x2="23" y2="12"/>
              <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
              <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
            </svg>
            <!-- 月亮图标 (暗色模式时显示，点击切换到亮色) -->
            <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
          </n-icon>
        </template>
      </n-button>

      <!-- 连接选择下拉菜单 -->
      <n-dropdown
        :options="connectionOptions"
        @select="handleConnectionSelect"
        trigger="click"
        placement="bottom-end"
      >
        <div>
          <n-button
            v-if="connectionStore.activeConnection"
            class="connection-trigger"
            :loading="connectionStore.loading"
            secondary
            type="primary"
          >
            <template #icon>
              <span
                class="status-dot"
                :class="{ connected: connectionStore.isConnected }"
              ></span>
            </template>
            {{ connectionStore.activeConnection.name }}
            <template #suffix>
              <n-icon size="14">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6 9 12 15 18 9"/>
                </svg>
              </n-icon>
            </template>
          </n-button>
          <n-button
            v-else
            class="connection-trigger"
            :loading="connectionStore.loading"
            secondary
            type="primary"
          >
            <template #icon>
              <n-icon size="16">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M5 12.55a11 11 0 0 1 14.08 0"/>
                  <path d="M1.42 9a16 16 0 0 1 21.16 0"/>
                  <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                  <line x1="12" y1="20" x2="12.01" y2="20"/>
                </svg>
              </n-icon>
            </template>
            选择连接
            <template #suffix>
              <n-icon size="14">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6 9 12 15 18 9"/>
                </svg>
              </n-icon>
            </template>
          </n-button>
        </div>
      </n-dropdown>
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
  gap: 12px;
}

.theme-toggle {
  color: white !important;
}

.theme-toggle:hover {
  background-color: rgba(255, 255, 255, 0.1) !important;
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

.connection-trigger {
  min-width: 120px;
  background-color: rgba(255, 255, 255, 0.95) !important;
  color: #333 !important;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.connection-trigger:hover {
  background-color: white !important;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

/* 下拉菜单中的状态圆点 */
.dropdown-status-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #ccc;
  margin-right: 8px;
}

.dropdown-status-dot.connected {
  background-color: #18a058;
}
</style>
