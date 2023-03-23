<script setup lang='ts'>
import { computed, onMounted, ref } from 'vue'
import { NLayout, NLayoutContent } from 'naive-ui'
import { useRouter } from 'vue-router'
import { os } from '@tauri-apps/api'
import { appWindow } from '@tauri-apps/api/window'
import Sider from './sider/index.vue'
import Permission from './Permission.vue'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { useAppStore, useAuthStore, useChatStore } from '@/store'

const router = useRouter()
const appStore = useAppStore()
const chatStore = useChatStore()
const authStore = useAuthStore()

router.replace({ name: 'Chat', params: { uuid: chatStore.active } })

const { isMobile } = useBasicLayout()

const collapsed = computed(() => appStore.siderCollapsed)

const needPermission = computed(() => !!authStore.session?.auth && !authStore.token)

const getMobileClass = computed(() => {
  if (isMobile.value)
    return ['rounded-none', 'shadow-none']
  return ['border', 'rounded-md', 'shadow-md', 'dark:border-neutral-800']
})

const getContainerClass = computed(() => {
  return [
    'h-full',
    { 'pl-[260px]': !isMobile.value && !collapsed.value },
  ]
})

const isDarwin = ref<boolean>(false)

onMounted(async () => {
  const o_s = await os.platform()
  isDarwin.value = (o_s === 'darwin')
})

const configClass = computed(() => {
  const all_class = ['h-full', 'dark:bg-[#24272e]', 'transition-all']
  if (isDarwin.value)
    all_class.push('pt-8')

  return all_class
})
</script>

<template>
  <div v-if="isDarwin" class="absolute h-8 w-full window-top" @mousedown="appWindow.startDragging" @touchstart="appWindow.startDragging" />

  <div :class="configClass">
    <div class="h-full overflow-hidden" :class="getMobileClass">
      <NLayout class="z-40 transition" :class="getContainerClass" has-sider>
        <Sider />
        <NLayoutContent class="h-full">
          <RouterView v-slot="{ Component, route }">
            <component :is="Component" :key="route.fullPath" />
          </RouterView>
        </NLayoutContent>
      </NLayout>
    </div>
    <Permission :visible="needPermission" />
  </div>
</template>
