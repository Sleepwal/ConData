import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { GlobalTheme } from 'naive-ui'
import { darkTheme } from 'naive-ui'

type ThemeName = 'light' | 'dark'

export const useThemeStore = defineStore('theme', () => {
  // 从 localStorage 读取保存的主题
  const savedTheme = localStorage.getItem('theme') as ThemeName | null
  const themeName = ref<ThemeName>(savedTheme || 'light')

  const isDark = computed(() => themeName.value === 'dark')
  const naiveTheme = computed<GlobalTheme | null>(() =>
    isDark.value ? darkTheme : null
  )

  function toggleTheme() {
    themeName.value = isDark.value ? 'light' : 'dark'
  }

  function setTheme(name: ThemeName) {
    themeName.value = name
  }

  // 保存主题到 localStorage
  watch(themeName, (newTheme) => {
    localStorage.setItem('theme', newTheme)
    // 更新 body class 用于非 naive-ui 组件的样式
    if (isDark.value) {
      document.body.classList.add('dark-mode')
    } else {
      document.body.classList.remove('dark-mode')
    }
  }, { immediate: true })

  return {
    themeName,
    isDark,
    naiveTheme,
    toggleTheme,
    setTheme,
  }
})
