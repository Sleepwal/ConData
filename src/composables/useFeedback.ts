import { useMessage, useDialog, useNotification } from 'naive-ui'

export function useFeedback() {
  const message = useMessage()
  const dialog = useDialog()
  const notification = useNotification()

  // 消息提示
  const msg = {
    success: (content: string, options?: any) => message.success(content, { duration: 3000, ...options }),
    error: (content: string, options?: any) => message.error(content, { duration: 5000, ...options }),
    warning: (content: string, options?: any) => message.warning(content, { duration: 3000, ...options }),
    info: (content: string, options?: any) => message.info(content, { duration: 3000, ...options }),
    loading: (content: string, options?: any) => message.loading(content, { duration: 0, ...options }),
  }

  // 确认对话框
  const confirm = {
    warning: (options: {
      title?: string
      content: string
      positiveText?: string
      negativeText?: string
      onPositive?: () => void
      onNegative?: () => void
    }) => {
      dialog.warning({
        title: options.title || '确认',
        content: options.content,
        positiveText: options.positiveText || '确认',
        negativeText: options.negativeText || '取消',
        onPositiveClick: options.onPositive,
        onNegativeClick: options.onNegative,
      })
    },
    error: (options: {
      title?: string
      content: string
      positiveText?: string
      onPositive?: () => void
    }) => {
      dialog.error({
        title: options.title || '错误',
        content: options.content,
        positiveText: options.positiveText || '确定',
        onPositiveClick: options.onPositive,
      })
    },
    success: (options: {
      title?: string
      content: string
      positiveText?: string
      onPositive?: () => void
    }) => {
      dialog.success({
        title: options.title || '成功',
        content: options.content,
        positiveText: options.positiveText || '确定',
        onPositiveClick: options.onPositive,
      })
    },
    info: (options: {
      title?: string
      content: string
      positiveText?: string
      negativeText?: string
      onPositive?: () => void
      onNegative?: () => void
    }) => {
      dialog.info({
        title: options.title || '提示',
        content: options.content,
        positiveText: options.positiveText || '确认',
        negativeText: options.negativeText || '取消',
        onPositiveClick: options.onPositive,
        onNegativeClick: options.onNegative,
      })
    },
  }

  // 通知
  const notify = {
    success: (options: { title?: string; content: string; duration?: number }) =>
      notification.success({
        title: options.title || '成功',
        content: options.content,
        duration: options.duration ?? 4500,
      }),
    error: (options: { title?: string; content: string; duration?: number }) =>
      notification.error({
        title: options.title || '错误',
        content: options.content,
        duration: options.duration ?? 4500,
      }),
    warning: (options: { title?: string; content: string; duration?: number }) =>
      notification.warning({
        title: options.title || '警告',
        content: options.content,
        duration: options.duration ?? 4500,
      }),
    info: (options: { title?: string; content: string; duration?: number }) =>
      notification.info({
        title: options.title || '提示',
        content: options.content,
        duration: options.duration ?? 4500,
      }),
  }

  return {
    message,
    dialog,
    notification,
    msg,
    confirm,
    notify,
  }
}
