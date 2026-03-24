<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { lightTheme } from 'naive-ui'
import { computed, onMounted } from 'vue'
import { useShortcuts } from '../../composables/useShortcuts'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
  canSubmit?: boolean
  connectionStatus?: string
  continueReplyEnabled?: boolean
  inputStatusText?: string
  layoutMode?: string
}

interface Emits {
  submit: []
  continue: []
  enhance: []
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
  canSubmit: false,
  connectionStatus: '已连接',
  continueReplyEnabled: true,
  inputStatusText: '',
  layoutMode: 'vertical',
})

const emit = defineEmits<Emits>()
const isSplitLayout = computed(() => props.layoutMode === 'split')

// 使用自定义快捷键系统
const {
  quickSubmitShortcutText,
  enhanceShortcutText,
  continueShortcutText,
  useQuickSubmitShortcut,
  useEnhanceShortcut,
  useContinueShortcut,
  loadShortcutConfig,
} = useShortcuts()

const shortcutText = quickSubmitShortcutText

const statusText = computed(() => {
  // 如果可以提交，直接显示快捷键提示
  if (props.canSubmit) {
    return shortcutText.value
  }

  // 如果有输入状态文本且不是默认状态，显示输入状态
  if (props.inputStatusText && props.inputStatusText !== '等待输入...') {
    return props.inputStatusText
  }

  // 根据请求类型显示不同的提示
  if (props.request?.predefined_options) {
    return '选择选项或输入文本'
  }
  return '请输入内容'
})

// 处理快捷键
useQuickSubmitShortcut(() => {
  if (props.canSubmit && !props.submitting) {
    handleSubmit()
  }
})

useEnhanceShortcut(() => {
  if (!props.submitting) {
    handleEnhance()
  }
})

useContinueShortcut(() => {
  if (!props.submitting) {
    handleContinue()
  }
})

function handleSubmit() {
  if (props.canSubmit && !props.submitting) {
    emit('submit')
  }
}

function handleContinue() {
  if (!props.submitting) {
    emit('continue')
  }
}

function handleEnhance() {
  if (!props.submitting) {
    emit('enhance')
  }
}

// 组件挂载时加载快捷键配置
onMounted(() => {
  loadShortcutConfig()
})
</script>

<template>
  <div :class="isSplitLayout ? 'min-h-[72px] bg-stone-100/70 px-5 py-3 select-none' : 'min-h-[60px] bg-gray-100 px-4 py-3 select-none'">
    <n-config-provider :theme="isSplitLayout ? lightTheme : null">
      <div v-if="!loading" :class="isSplitLayout ? 'flex flex-wrap items-center justify-between gap-3' : 'flex justify-between items-center'">
        <!-- 左侧状态信息 -->
        <div class="flex items-center">
          <div :class="isSplitLayout ? 'flex items-center gap-3 text-xs text-stone-500' : 'flex items-center gap-2 text-xs text-gray-600'">
            <div :class="isSplitLayout ? 'h-2 w-2 rounded-full bg-amber-700' : 'h-2 w-2 rounded-full bg-primary-500'" />
            <span :class="isSplitLayout ? 'font-semibold text-stone-700' : 'font-medium'">{{ connectionStatus }}</span>
            <span class="opacity-40">/</span>
            <span class="tabular-nums opacity-80">{{ statusText }}</span>
          </div>
        </div>

        <!-- 右侧操作按钮 -->
        <div class="flex items-center" data-guide="popup-actions">
          <n-space :size="isSplitLayout ? 'medium' : 'small'">
            <!-- 增强按钮 -->
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <n-button
                  :disabled="!canSubmit || submitting"
                  size="medium"
                  :type="isSplitLayout ? 'default' : 'info'"
                  :secondary="isSplitLayout"
                  class="min-w-[84px]"
                  data-guide="enhance-button"
                  @click="handleEnhance"
                >
                  <template #icon>
                    <div class="i-carbon-magic-wand w-4 h-4" />
                  </template>
                  增强
                </n-button>
              </template>
              {{ enhanceShortcutText }}
            </n-tooltip>

            <!-- 继续按钮 -->
            <n-tooltip v-if="continueReplyEnabled" trigger="hover" placement="top">
              <template #trigger>
                <n-button
                  :disabled="submitting"
                  :loading="submitting"
                  size="medium"
                  :quaternary="isSplitLayout"
                  type="default"
                  class="min-w-[84px]"
                  data-guide="continue-button"
                  @click="handleContinue"
                >
                  <template #icon>
                    <div class="i-carbon-play w-4 h-4" />
                  </template>
                  继续
                </n-button>
              </template>
              {{ continueShortcutText }}
            </n-tooltip>

            <!-- 发送按钮 -->
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <n-button
                  :disabled="!canSubmit || submitting"
                  :loading="submitting"
                  size="medium"
                  :secondary="isSplitLayout"
                  :color="isSplitLayout ? '#6b5a45' : undefined"
                  :text-color="isSplitLayout ? '#fafaf9' : undefined"
                  :border-color="isSplitLayout ? '#6b5a45' : undefined"
                  :type="isSplitLayout ? 'default' : 'primary'"
                  class="min-w-[88px]"
                  data-guide="submit-button"
                  @click="handleSubmit"
                >
                  <template #icon>
                    <div v-if="!submitting" class="i-carbon-send w-4 h-4" />
                  </template>
                  {{ submitting ? '发送中...' : '发送' }}
                </n-button>
              </template>
              {{ shortcutText }}
            </n-tooltip>
          </n-space>
        </div>
      </div>
    </n-config-provider>
  </div>
</template>
