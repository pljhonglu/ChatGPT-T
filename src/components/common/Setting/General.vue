<script lang="ts" setup>
import { computed, ref } from 'vue'
import { NButton, NPopconfirm, NSelect, useMessage } from 'naive-ui'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import type { Language, Theme } from '@/store/modules/app/helper'
import { SvgIcon } from '@/components/common'
import { useAppStore } from '@/store'
import { useBasicLayout } from '@/hooks/useBasicLayout'

const appStore = useAppStore()

const { isMobile } = useBasicLayout()

const ms = useMessage()

const theme = computed(() => appStore.theme)

const language = computed({
  get() {
    return appStore.language
  },
  set(value: Language) {
    appStore.setLanguage(value)
  },
})

const themeOptions: { label: string; key: Theme; icon: string }[] = [
  {
    label: 'Auto',
    key: 'auto',
    icon: 'ri:contrast-line',
  },
  {
    label: 'Light',
    key: 'light',
    icon: 'ri:sun-foggy-line',
  },
  {
    label: 'Dark',
    key: 'dark',
    icon: 'ri:moon-foggy-line',
  },
]

const languageOptions: { label: string; key: Language; value: Language }[] = [
  { label: '简体中文', key: 'zh-CN', value: 'zh-CN' },
  { label: '繁體中文', key: 'zh-TW', value: 'zh-TW' },
  { label: 'English', key: 'en-US', value: 'en-US' },
]

function clearData(): void {
  localStorage.removeItem('chatStorage')
  location.reload()
}

const updateLoading = ref<boolean>(false)

async function checkAppUpdate() {
  const update_info = await checkUpdate()
  if (update_info.shouldUpdate) {
    try {
      ms.info('发现新版本，正在更新...')
      updateLoading.value = true
      await installUpdate()
      await relaunch()
      updateLoading.value = false
    }
    catch (error) {
      updateLoading.value = false
      ms.error(error as string)
    }
  }
  else {
    ms.info('当前是最新版本！')
  }
}
</script>

<template>
  <div class="p-4 space-y-5 min-h-[200px]">
    <div class="space-y-6">
      <div
        class="flex items-center space-x-4"
        :class="isMobile && 'items-start'"
      >
        <span class="flex-shrink-0 w-[100px]">{{ $t('setting.chatHistory') }}</span>

        <div class="flex flex-wrap items-center gap-4">
          <NPopconfirm placement="bottom" @positive-click="clearData">
            <template #trigger>
              <NButton size="small">
                <template #icon>
                  <SvgIcon icon="ri:close-circle-line" />
                </template>
                {{ $t('common.clear') }}
              </NButton>
            </template>
            {{ $t('chat.clearHistoryConfirm') }}
          </NPopconfirm>
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <span class="flex-shrink-0 w-[100px]">{{ $t('setting.theme') }}</span>
        <div class="flex flex-wrap items-center gap-4">
          <template v-for="item of themeOptions" :key="item.key">
            <NButton
              size="small"
              :type="item.key === theme ? 'primary' : undefined"
              @click="appStore.setTheme(item.key)"
            >
              <template #icon>
                <SvgIcon :icon="item.icon" />
              </template>
            </NButton>
          </template>
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <span class="flex-shrink-0 w-[100px]">{{ $t('setting.language') }}</span>
        <div class="flex flex-wrap items-center gap-4">
          <NSelect
            style="width: 140px"
            :value="language"
            :options="languageOptions"
            @update-value="value => appStore.setLanguage(value)"
          />
        </div>
      </div>
      <div class="flex items-center space-x-4">
        <span class="flex-shrink-0 w-[100px]">更新</span>
        <div class="flex flex-wrap items-center gap-4">
          <NButton size="small" :loading="updateLoading" :disabled="updateLoading" @click="checkAppUpdate">
            <template #icon>
              <SvgIcon icon="ri:download-2-fill" />
            </template>
            检查更新
          </NButton>
        </div>
      </div>
    </div>
  </div>
</template>
