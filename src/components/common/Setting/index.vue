<script setup lang='ts'>
import { computed, ref } from 'vue'
import { NModal, NTabPane, NTabs } from 'naive-ui'
import General from './General.vue'
import About from './About.vue'
import User from './User.vue'
import { SvgIcon } from '@/components/common'

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

interface Props {
  visible: boolean
}

interface Emit {
  (e: 'update:visible', visible: boolean): void
}

const active = ref('User')

const show = computed({
  get() {
    return props.visible
  },
  set(visible: boolean) {
    emit('update:visible', visible)
  },
})
</script>

<template>
  <NModal v-model:show="show" :auto-focus="false" preset="card" :title="$t('setting.setting')" style="width: 95%; max-width: 640px;">
    <div>
      <NTabs v-model:value="active" type="line" animated>
        <NTabPane name="User" tab="User">
          <template #tab>
            <SvgIcon class="text-lg" icon="ri:file-user-line" />
            <span class="ml-2">用户</span>
          </template>
          <div class="min-h-[100px]">
            <User />
          </div>
        </NTabPane>
        <NTabPane name="Config" tab="Config">
          <template #tab>
            <SvgIcon class="text-lg" icon="ri:list-settings-line" />
            <span class="ml-2">{{ $t('setting.config') }}</span>
          </template>
          <General />
        </NTabPane>
        <NTabPane name="Aboud" tab="Aboud">
          <template #tab>
            <SvgIcon class="text-lg" icon="ri:information-line" />
            <span class="ml-2">{{ $t('setting.aboud') }}</span>
          </template>
          <div class="min-h-[100px]">
            <About />
          </div>
        </NTabPane>
      </NTabs>
    </div>
  </NModal>
</template>
