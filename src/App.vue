<script setup lang="ts">
import { onMounted } from 'vue'
import { useConnectionStore } from './stores/connection'
import { useThemeStore } from './stores/theme'
import AppHeader from './components/common/AppHeader.vue'

const connectionStore = useConnectionStore()
const themeStore = useThemeStore()

onMounted(() => {
  connectionStore.loadConnections()
})

// 主题覆盖配置
const themeOverrides = {
  common: {
    primaryColor: '#667eea',
    primaryColorHover: '#5a6fd6',
    primaryColorPressed: '#4a5fc4',
    successColor: '#4CAF50',
    warningColor: '#FF9800',
    errorColor: '#f44336',
  }
}
</script>

<template>
  <n-config-provider :theme="themeStore.naiveTheme" :theme-overrides="themeOverrides">
    <n-dialog-provider>
      <n-message-provider>
        <n-notification-provider>
          <div class="app" :class="{ 'dark-mode': themeStore.isDark }">
            <AppHeader />
            <main class="main-content">
              <router-view />
            </main>
          </div>
        </n-notification-provider>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  background-color: #f5f5f5;
  color: #333;
}

/* 亮色模式默认变量 */
:root {
  --bg-color: #f5f5f5;
  --card-bg: #ffffff;
  --text-color: #333333;
  --text-color-secondary: #666666;
  --text-color-disabled: #888888;
  --border-color: #eeeeee;
  --border-color-light: #f0f0f0;
  --hover-bg: #f5f5f5;
  --table-header-bg: #f5f5f5;
  --stat-bg: #f5f5f5;
  --badge-bg: #e0e0e0;
  --badge-primary-bg: #e3f2fd;
  --primary-color: #1976d2;
  --pk-bg: #fff8e1;
  --pk-bg-hover: #ffecb3;
  --error-bg: #ffebee;
  --error-color: #c62828;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* 暗黑模式样式变量 */
.dark-mode {
  --bg-color: #18181c;
  --card-bg: #26262a;
  --text-color: #e0e0e0;
  --text-color-secondary: #a0a0a0;
  --text-color-disabled: #666666;
  --border-color: #333333;
  --border-color-light: #3a3a3a;
  --hover-bg: #2a2a2e;
  --table-header-bg: #1f1f23;
  --stat-bg: #1f1f23;
  --badge-bg: #3a3a3e;
  --badge-primary-bg: #1e3a5f;
  --primary-color: #4fc3f7;
  --pk-bg: #3d3a2a;
  --pk-bg-hover: #4a4628;
  --error-bg: #3d1f1f;
  --error-color: #ff6b6b;
}

/* 暗黑模式下 body 背景色 */
body.dark-mode {
  background-color: #0d0d0d;
  color: #e0e0e0;
}
</style>
