import type { GenericAbortSignal } from 'axios'
import { event, invoke } from '@tauri-apps/api'
import { post } from '@/utils/request'

interface ProgressPayload {
  id: number
  detail: string
  finish_reason: string
}

type ProgressHandler = (payload: ProgressPayload) => void

const handlers: Map<number, ProgressHandler> = new Map()
let listening = false

async function listenToEventIfNeeded(): Promise<void> {
  if (listening)
    return await Promise.resolve()

  listening = true
  await event.listen<ProgressPayload>('CHAT_FETCHEING_PROGRESS', ({ payload }) => {
    const handler = handlers.get(payload.id)
    if (handler != null)
      handler(payload)
  })
  await event.listen<ProgressPayload>('CHAT_FETCHEING_FINISHED', ({ payload }) => {
    const handler = handlers.get(payload.id)
    if (handler != null) {
      handler(payload)
      handlers.delete(payload.id)
    }
  })
}

export async function fetchChatAPIProcess<T = any>(
  prompt: string,
  options?: T,
  signal?: GenericAbortSignal,
  progressHandler?: (detail: string, finish_reason: string, options: T | undefined) => void,
) {
  const ids = new Uint32Array(1)
  window.crypto.getRandomValues(ids)
  const id = ids[0]

  if (progressHandler != null) {
    handlers.set(id, (payload) => {
      progressHandler(payload.detail, payload.finish_reason, options)
    })
  }
  await listenToEventIfNeeded()

  if (signal) {
    signal.onabort = () => {
      handlers.delete(id)
    }
  }

  await invoke('fetch_chat_api', {
    id,
    messages: [{
      role: 'user',
      content: prompt,
    }],
    temperature: 0.6,
    options: JSON.stringify(options),
  })
}

// export function fetchChatAPI<T = any>(
//   prompt: string,
//   options?: { conversationId?: string; parentMessageId?: string },
//   signal?: GenericAbortSignal,
// ) {
//   return post<T>({
//     url: '/chat',
//     data: { prompt, options },
//     signal,
//   })
// }

export function fetchChatConfig<T = any>() {
  return post<T>({
    url: '/config',
  })
}

// export function fetchChatAPIProcess<T = any>(
//   params: {
//     prompt: string
//     options?: { conversationId?: string; parentMessageId?: string }
//     signal?: GenericAbortSignal
//     onDownloadProgress?: (progressEvent: AxiosProgressEvent) => void },
// ) {
//   return post<T>({
//     url: '/chat-process',
//     data: { prompt: params.prompt, options: params.options },
//     signal: params.signal,
//     onDownloadProgress: params.onDownloadProgress,
//   })
// }

export function fetchSession<T>() {
  return post<T>({
    url: '/session',
  })
}

export function fetchVerify<T>(token: string) {
  return post<T>({
    url: '/verify',
    data: { token },
  })
}
