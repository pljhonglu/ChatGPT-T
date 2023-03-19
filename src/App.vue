<script setup lang="ts">
import { NConfigProvider } from 'naive-ui'
import { appWindow } from '@tauri-apps/api/window'
import { NaiveProvider } from '@/components/common'
import { useTheme } from '@/hooks/useTheme'
import { useLanguage } from '@/hooks/useLanguage'

const { theme, themeOverrides } = useTheme()
const { language } = useLanguage()
</script>

<template>
  <div class="absolute h-8 w-full window-top" @mousedown="appWindow.startDragging" @touchstart="appWindow.startDragging" />
  <NConfigProvider
    class="h-full pt-8"
    :theme="theme"
    :theme-overrides="themeOverrides"
    :locale="language"
  >
    <NaiveProvider>
      <RouterView />
    </NaiveProvider>
  </NConfigProvider>
</template>
