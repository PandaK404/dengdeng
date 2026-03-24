<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

interface ReplyConfig {
  enable_continue_reply: boolean
  auto_continue_threshold: number
  continue_prompt: string
  enable_timeout_auto_submit: boolean
  timeout_auto_submit_seconds: number
  timeout_auto_submit_action: 'retry_xuyan' | 'custom_input'
  timeout_auto_submit_custom_input: string
}

const localConfig = ref<ReplyConfig>({
  enable_continue_reply: true,
  auto_continue_threshold: 1000,
  continue_prompt: '请按照最佳实践继续',
  enable_timeout_auto_submit: true,
  timeout_auto_submit_seconds: 400,
  timeout_auto_submit_action: 'retry_xuyan',
  timeout_auto_submit_custom_input: '',
})

function normalizeReplyConfig(config: Partial<ReplyConfig>): ReplyConfig {
  return {
    enable_continue_reply: config.enable_continue_reply ?? true,
    auto_continue_threshold: config.auto_continue_threshold ?? 1000,
    continue_prompt: config.continue_prompt ?? '请按照最佳实践继续',
    enable_timeout_auto_submit: config.enable_timeout_auto_submit ?? true,
    timeout_auto_submit_seconds: Math.max(1, Math.floor(config.timeout_auto_submit_seconds ?? 400)),
    timeout_auto_submit_action: config.timeout_auto_submit_action === 'custom_input' ? 'custom_input' : 'retry_xuyan',
    timeout_auto_submit_custom_input: config.timeout_auto_submit_custom_input ?? '',
  }
}

// 加载配置
async function loadConfig() {
  try {
    const config = await invoke('get_reply_config')
    localConfig.value = normalizeReplyConfig(config as Partial<ReplyConfig>)
  }
  catch (error) {
    console.error('加载继续回复配置失败:', error)
  }
}

// 更新配置
async function updateConfig() {
  try {
    localConfig.value = normalizeReplyConfig(localConfig.value)
    await invoke('set_reply_config', { replyConfig: localConfig.value })
  }
  catch (error) {
    console.error('保存继续回复配置失败:', error)
  }
}

function handleTimeoutSecondsUpdate(value: number | null) {
  localConfig.value.timeout_auto_submit_seconds = Math.max(1, Math.floor(value ?? 400))
  updateConfig()
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <!-- 设置内容 -->
  <n-space vertical size="large">
    <!-- 启用继续回复 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            启用继续回复
          </div>
          <div class="text-xs opacity-60">
            启用后将显示继续按钮
          </div>
        </div>
      </div>
      <n-switch
        v-model:value="localConfig.enable_continue_reply"
        size="small"
        @update:value="updateConfig"
      />
    </div>

    <!-- 继续提示词 -->
    <div v-if="localConfig.enable_continue_reply">
      <div class="flex items-center mb-3">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            继续提示词
          </div>
          <div class="text-xs opacity-60">
            点击继续按钮时发送的提示词
          </div>
        </div>
      </div>
      <n-input
        v-model:value="localConfig.continue_prompt"
        size="small"
        placeholder="请按照最佳实践继续"
        @input="updateConfig"
      />
    </div>

    <!-- 超时自动提交 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            启用超时自动提交
          </div>
          <div class="text-xs opacity-60">
            弹窗出现后开始计时，到点自动提交
          </div>
        </div>
      </div>
      <n-switch
        v-model:value="localConfig.enable_timeout_auto_submit"
        size="small"
        @update:value="updateConfig"
      />
    </div>

    <div v-if="localConfig.enable_timeout_auto_submit">
      <div class="flex items-center mb-3">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            超时时间
          </div>
          <div class="text-xs opacity-60">
            默认 400 秒，从弹窗出现时开始计时
          </div>
        </div>
      </div>
      <n-input-number
        :value="localConfig.timeout_auto_submit_seconds"
        size="small"
        :min="1"
        :step="10"
        placeholder="400"
        @update:value="handleTimeoutSecondsUpdate"
      />
    </div>

    <div v-if="localConfig.enable_timeout_auto_submit">
      <div class="flex items-center mb-3">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            超时后提交内容
          </div>
          <div class="text-xs opacity-60">
            选择重新调用 xuyan，或提交一段固定输入
          </div>
        </div>
      </div>
      <n-radio-group
        v-model:value="localConfig.timeout_auto_submit_action"
        size="small"
        @update:value="updateConfig"
      >
        <n-space vertical size="small">
          <n-radio value="retry_xuyan">
            重新调用 xuyan
          </n-radio>
          <n-radio value="custom_input">
            指定输入
          </n-radio>
        </n-space>
      </n-radio-group>
    </div>

    <div
      v-if="localConfig.enable_timeout_auto_submit && localConfig.timeout_auto_submit_action === 'custom_input'"
    >
      <div class="flex items-center mb-3">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            指定输入内容
          </div>
          <div class="text-xs opacity-60">
            超时后自动提交这段内容；留空时会退回为“重新调用xuyan”
          </div>
        </div>
      </div>
      <n-input
        v-model:value="localConfig.timeout_auto_submit_custom_input"
        size="small"
        placeholder="请输入超时后自动提交的内容"
        @input="updateConfig"
      />
    </div>
  </n-space>
</template>
