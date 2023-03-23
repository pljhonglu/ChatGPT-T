<script setup lang='ts'>
import { NLayout, NLayoutContent } from 'naive-ui'
</script>

<template>
  <div class="h-full dark:bg-[#24272e] transition-all">
    <div class="h-full overflow-hidden">
      <NLayout class="z-40 transition h-full">
        <NLayoutContent class="m-3">
          <RouterView v-slot="{ Component, route }">
            <component :is="Component" :key="route.fullPath" />
          </RouterView>
        </NLayoutContent>
      </NLayout>
    </div>
  </div>
</template>
