<script setup lang='ts'>
import { ref } from 'vue'
import { NDropdown } from 'naive-ui'
import AvatarComponent from './Avatar.vue'
import TextComponent from './Text.vue'
import { SvgIcon } from '@/components/common'
import { copyText } from '@/utils/format'
import { useIconRender } from '@/hooks/useIconRender'
import { t } from '@/locales'

interface Props {
  dateTime?: string
  text?: string
  isBot?: boolean
  error?: boolean
  loading?: boolean
}

interface Emit {
  (ev: 'regenerate'): void
  (ev: 'delete'): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

const { iconRender } = useIconRender()

const textRef = ref<HTMLElement>()

const options = [
  {
    label: t('chat.copy'),
    key: 'copyText',
    icon: iconRender({ icon: 'ri:file-copy-2-line' }),
  },
  {
    label: t('common.delete'),
    key: 'delete',
    icon: iconRender({ icon: 'ri:delete-bin-line' }),
  },
]

function handleSelect(key: 'copyRaw' | 'copyText' | 'delete') {
  switch (key) {
    case 'copyText':
      copyText({ text: props.text ?? '' })
      return
    case 'delete':
      emit('delete')
  }
}

function handleRegenerate() {
  emit('regenerate')
}
</script>

<template>
  <div class="flex w-full mb-6 overflow-hidden" :class="[{ 'flex-row-reverse': !isBot }]">
    <div
      class="flex items-center justify-center flex-shrink-0 h-8 overflow-hidden rounded-full basis-8"
      :class="[isBot ? 'mr-2' : 'ml-2']"
    >
      <AvatarComponent :image="!isBot" />
    </div>
    <div class="overflow-hidden text-sm " :class="[isBot ? 'items-start' : 'items-end']">
      <p class="text-xs text-[#b4bbc4]" :class="[isBot ? 'text-left' : 'text-right']">
        {{ dateTime }}
      </p>
      <div
        class="flex items-end gap-1 mt-2"
        :class="[isBot ? 'flex-row' : 'flex-row-reverse']"
      >
        <TextComponent
          ref="textRef"
          :is-bot="isBot"
          :error="error"
          :text="text"
          :loading="loading"
        />
        <div class="flex flex-col">
          <button
            v-if="isBot"
            class="mb-2 transition text-neutral-300 hover:text-neutral-800 dark:hover:text-neutral-300"
            @click="handleRegenerate"
          >
            <SvgIcon icon="ri:restart-line" />
          </button>
          <NDropdown :placement="isBot ? 'right' : 'left'" :options="options" @select="handleSelect">
            <button class="transition text-neutral-300 hover:text-neutral-800 dark:hover:text-neutral-200">
              <SvgIcon icon="ri:more-2-fill" />
            </button>
          </NDropdown>
        </div>
      </div>
    </div>
  </div>
</template>
