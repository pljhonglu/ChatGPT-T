import type { GenericAbortSignal } from 'axios'
import { event, invoke } from '@tauri-apps/api'
import { post } from '@/utils/request'

interface ProgressPayload {
  id: number
  detail: string
  finish_reason: string
  role: string
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
  messages: Chat.RequestMessage[],
  progressHandler?: (detail: string, role: string) => void,
  signal?: GenericAbortSignal,
) {
  const ids = new Uint32Array(1)
  window.crypto.getRandomValues(ids)
  const id = ids[0]

  if (progressHandler != null) {
    handlers.set(id, (payload) => {
      progressHandler(payload.detail, payload.role)
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
    messages,
    temperature: 0.6,
  })
}

export function fetchChatConfig<T = any>() {
  return post<T>({
    url: '/config',
  })
}

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
