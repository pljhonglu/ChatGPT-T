<script setup lang="ts">
import { NConfigProvider } from 'naive-ui'
import { appWindow } from '@tauri-apps/api/window'
import { os } from '@tauri-apps/api'
import { computed, onMounted, ref } from 'vue'
import { NaiveProvider } from '@/components/common'
import { useTheme } from '@/hooks/useTheme'
import { useLanguage } from '@/hooks/useLanguage'

const { theme, themeOverrides } = useTheme()
const { language } = useLanguage()

const isDarwin = ref<boolean>(false)

onMounted(async () => {
  const o_s = await os.platform()
  isDarwin.value = (o_s === 'darwin')
})

const configClass = computed(() => {
  if (isDarwin.value)
    return ['h-full', 'pt-8']

  else
    return ['h-full']
})
</script>

<template>
  <div v-if="isDarwin" class="absolute h-8 w-full window-top" @mousedown="appWindow.startDragging" @touchstart="appWindow.startDragging" />
  <NConfigProvider
    :class="configClass"
    :theme="theme"
    :theme-overrides="themeOverrides"
    :locale="language"
  >
    <NaiveProvider>
      <RouterView />
    </NaiveProvider>
  </NConfigProvider>
</template>
