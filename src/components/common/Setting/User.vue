<script setup lang='ts'>
import { computed, ref } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { NButton, NForm, NFormItem, NInput, useMessage } from 'naive-ui'
import { useUserStore } from '@/store'
import { t } from '@/locales'

const userStore = useUserStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)
const userInfo = computed(() => userStore.userInfo)

const model = ref({
  name: userInfo.value.name,
  avatar: userInfo.value.avatar,
  apiKey: userInfo.value.apiKey,
})

const rules: FormRules = {
  name: [
    {
      required: true,
      message: '请输入昵称',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  apiKey: [
    {
      required: true,
      message: '请输入 api-key',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^sk-\w+$/.test(value))
          return new Error('请输入正确的api-key')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
}

function saveUserInfo() {
  formRef.value?.validate((errors) => {
    if (!errors) {
      userInfo.value.name = model.value.name
      userInfo.value.avatar = model.value.avatar
      userInfo.value.apiKey = model.value.apiKey
      userStore.recordState()
      ms.success(t('common.success'))
      // window.location.reload()
    }
  })
}
</script>

<template>
  <div class="p-4 space-y-5 min-h-[200px]">
    <NForm ref="formRef" :model="model" :rules="rules">
      <NFormItem path="avatar" :label="$t('setting.avatarLink')">
        <NInput v-model:value="model.avatar" placeholder="" />
      </NFormItem>
      <NFormItem path="name" :label="$t('setting.name')">
        <NInput v-model:value="model.name" placeholder="" />
      </NFormItem>
      <NFormItem path="apiKey" label="Openai API Key">
        <NInput v-model:value="model.apiKey" placeholder="" />
      </NFormItem>
      <div class="flex items-center justify-end">
        <NButton size="small" @click="saveUserInfo">
          {{ $t('setting.saveUserInfoBtn') }}
        </NButton>
      </div>
    </NForm>
  </div>
</template>
