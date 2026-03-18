import { createApp } from 'vue'
import { createPinia } from 'pinia'
import {
  create,
  NAlert,
  NButton,
  NCard,
  NConfigProvider,
  NDataTable,
  NDialog,
  NDialogProvider,
  NDivider,
  NEmpty,
  NForm,
  NFormItem,
  NIcon,
  NInput,
  NInputNumber,
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NMenu,
  NMessageProvider,
  NModal,
  NNotificationProvider,
  NPopconfirm,
  NPopover,
  NSelect,
  NSpace,
  NSpin,
  NSwitch,
  NTabPane,
  NTabs,
  NTag,
  NTooltip,
  NTree,
} from 'naive-ui'
import router from './router'
import App from './App.vue'

const naive = create({
  components: [
    NAlert,
    NButton,
    NCard,
    NConfigProvider,
    NDataTable,
    NDialog,
    NDialogProvider,
    NDivider,
    NEmpty,
    NForm,
    NFormItem,
    NIcon,
    NInput,
    NInputNumber,
    NLayout,
    NLayoutContent,
    NLayoutSider,
    NMenu,
    NMessageProvider,
    NModal,
    NNotificationProvider,
    NPopconfirm,
    NPopover,
    NSelect,
    NSpace,
    NSpin,
    NSwitch,
    NTabPane,
    NTabs,
    NTag,
    NTooltip,
    NTree,
  ],
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(naive)

app.mount('#app')
