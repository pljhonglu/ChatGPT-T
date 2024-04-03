<script setup lang='ts'>
import { computed, ref } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { NButton, NForm, NFormItem, NInput, NSelect, useMessage } from 'naive-ui'
import { useUserStore } from '@/store'
import { t } from '@/locales'

const userStore = useUserStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)
const userInfo = computed(() => userStore.userInfo)
const userConfig = computed(() => userStore.userConfig)

const hostUrl = new URL(userConfig.value.host)

const model = ref({
  name: userInfo.value.name,
  avatar: userInfo.value.avatar,
  apiKey: userConfig.value.apiKey,
  modelName: userConfig.value.modelName,
  host: `${hostUrl.protocol}//${hostUrl.host}`,
  proxy: userConfig.value.proxy,
})

const models = userStore.allModels().map(v => ({
  label: v,
  value: v,
}))

const rules: FormRules = {
  name: [
    {
      required: true,
      message: t('setting.namePlaceholder'),
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error(t('setting.nameNotEmptyError'))

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  proxy: [{
    required: false,
    validator(rule: FormItemRule, value: string) {
      if (!value || value.length === 0)
        return true

      else if (!/^(socks5):\/\/.+$/.test(value))
        return new Error('Proxy must start with socks5://')
      return true
    },
    trigger: ['input', 'blur'],
  }],
  apiKey: [
    {
      required: true,
      message: '请输入 api-key',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^\w+-\w+$/.test(value))
          return new Error('请输入正确的api-key')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  host: [
    {
      required: true,
      message: '请输入openai api host',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^(http|https):\/\/[^ "]+(:\d+)?$/i.test(value))
          return new Error('请输入正确的host')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
}

function saveUserInfo() {
  formRef.value?.validate((errors) => {
    if (!errors) {
      const hostUrl = new URL(model.value.host)

      userInfo.value.name = model.value.name
      userInfo.value.avatar = model.value.avatar
      userConfig.value.apiKey = model.value.apiKey
      userConfig.value.modelName = model.value.modelName
      userConfig.value.proxy = model.value.proxy
      userConfig.value.host = `${hostUrl.protocol}//${hostUrl.host}`
      userStore.recordState()
      ms.success(t('common.success'))
    }
  })
}
</script>

<template>
  <div class="p-4 space-y-5 min-h-[200px] max-h-80 overflow-auto">
    <NForm ref="formRef" :model="model" :rules="rules">
      <NFormItem path="avatar" :label="$t('setting.avatarLink')">
        <NInput v-model:value="model.avatar" :placeholder="$t('setting.avatarLinkPlaceholder')" />
      </NFormItem>
      <NFormItem path="name" :label="$t('setting.name')">
        <NInput v-model:value="model.name" :placeholder="$t('setting.namePlaceholder')" />
      </NFormItem>
      <NFormItem path="apiKey" label="Openai API Key">
        <NInput v-model:value="model.apiKey" placeholder="Openai API Key" />
      </NFormItem>
      <NFormItem path="modelName" label="Model Name">
        <NSelect v-model:value="model.modelName" placeholder="Select" :options="models" />
      </NFormItem>
      <NFormItem path="host" label="Host">
        <NInput v-model:value="model.host" placeholder="" />
      </NFormItem>
      <NFormItem path="proxy" label="Proxy">
        <NInput v-model:value="model.proxy" placeholder="socks5://127.0.0.1:7890" />
      </NFormItem>
      <div class="flex items-center justify-end">
        <NButton size="small" @click="saveUserInfo">
          {{ $t('setting.saveUserInfoBtn') }}
        </NButton>
      </div>
    </NForm>
  </div>
</template>
