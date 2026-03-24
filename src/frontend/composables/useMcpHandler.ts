import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import type { McpRequest } from '../types/popup'

/**
 * MCP处理组合式函数
 */
export function useMcpHandler() {
  const mcpRequest = ref<McpRequest | null>(null)
  const showMcpPopup = ref(false)

  function currentSessionId() {
    return mcpRequest.value?.session_id ?? mcpRequest.value?.id ?? null
  }

  /**
   * 统一的MCP响应处理
   */
  async function handleMcpResponse(response: any) {
    try {
      // 通过Tauri命令发送响应并退出应用
      await invoke('send_mcp_response', { response, sessionId: currentSessionId() })
      await invoke('exit_app')
    }
    catch (error) {
      console.error('MCP响应处理失败:', error)
    }
  }

  /**
   * 统一的MCP取消处理
   */
  async function handleMcpCancel() {
    try {
      // 发送取消信息并退出应用
      await invoke('send_mcp_response', { response: 'CANCELLED', sessionId: currentSessionId() })
      await invoke('exit_app')
    }
    catch (error) {
      // 静默处理MCP取消错误
      console.error('MCP取消处理失败:', error)
    }
  }

  /**
   * 显示MCP弹窗
   */
  async function showMcpDialog(request: McpRequest) {
    // 获取Telegram配置，检查是否需要隐藏前端弹窗
    let shouldShowFrontendPopup = true
    try {
      const telegramConfig = await invoke('get_telegram_config')
      // 如果Telegram启用且配置了隐藏前端弹窗，则不显示前端弹窗
      if (telegramConfig && (telegramConfig as any).enabled && (telegramConfig as any).hide_frontend_popup) {
        shouldShowFrontendPopup = false
        console.log('🔕 根据Telegram配置，隐藏前端弹窗')
      }
    }
    catch (error) {
      console.error('获取Telegram配置失败:', error)
      // 配置获取失败时，保持默认行为（显示弹窗）
    }

    // 根据配置决定是否显示前端弹窗
    if (shouldShowFrontendPopup) {
      // 设置请求数据和显示状态
      mcpRequest.value = request
      showMcpPopup.value = true
    }
    else {
      console.log('🔕 跳过前端弹窗显示，仅使用Telegram交互')
    }

    // 播放音频通知（无论是否显示弹窗都播放）
    try {
      await invoke('play_notification_sound')
    }
    catch (error) {
      console.error('播放音频通知失败:', error)
    }

    // GUI 模式下不再自动启动 Telegram 同步，避免多会话共享同一个 Bot / Chat 后串台。
    // 纯 Telegram 模式由后端 CLI 直接处理，不经过前端。
  }

  /**
   * 检查MCP模式
   */
  async function checkMcpMode() {
    try {
      const args = await invoke('get_cli_args')

      if (args && (args as any).mcp_request) {
        // 读取MCP请求文件
        const content = await invoke('read_mcp_request', { filePath: (args as any).mcp_request })

        if (content) {
          await showMcpDialog(content)
        }
        return { isMcp: true, mcpContent: content }
      }
    }
    catch (error) {
      console.error('检查MCP模式失败:', error)
    }
    return { isMcp: false, mcpContent: null }
  }

  /**
   * 设置MCP事件监听器
   */
  async function setupMcpEventListener() {
    try {
      await listen('mcp-request', (event) => {
        showMcpDialog(event.payload)
      })
    }
    catch (error) {
      console.error('设置MCP事件监听器失败:', error)
    }
  }

  return {
    mcpRequest,
    showMcpPopup,
    handleMcpResponse,
    handleMcpCancel,
    showMcpDialog,
    checkMcpMode,
    setupMcpEventListener,
  }
}
