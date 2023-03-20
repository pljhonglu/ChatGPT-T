import * as _ from 'lodash'
import { ss } from '@/utils/storage'

const LOCAL_NAME = 'userStorage'

export interface UserInfo {
  avatar: string
  name: string | null
  modelName: string
  apiKey: string
  proxy: string | null
}

export interface UserState {
  userInfo: UserInfo
}

export function defaultSetting(): UserState {
  return {
    userInfo: {
      avatar: '',
      name: null,
      modelName: 'gpt-3.5-turbo',
      apiKey: import.meta.env.VITE_GLOB_OPENAI_KEY,
      proxy: null,
    },
  }
}

export function allModels(): string[] {
  return ['gpt-3.5-turbo', 'gpt-3.5-turbo-0301', 'gpt-4', 'gpt-4-0314', 'gpt-4-32k', 'gpt-4-32k-0314']
}

export function getLocalState(): UserState {
  const localSetting: UserState | undefined = ss.get(LOCAL_NAME)
  const ds = defaultSetting()
  const state = _.merge(ds, localSetting)
  return state
}

export function setLocalState(setting: UserState): void {
  ss.set(LOCAL_NAME, setting)
}
