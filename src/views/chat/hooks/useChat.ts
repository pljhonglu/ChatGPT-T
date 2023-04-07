import * as _ from 'lodash'
import GPT3Tokenizer from 'gpt3-tokenizer'
import { useChatStore, useUserStore } from '@/store'

export function useChat() {
  const chatStore = useChatStore()
  const userStore = useUserStore()
  const tokenizer = new GPT3Tokenizer({ type: 'gpt3' })

  const getChatByUuidAndIndex = (uuid: number, index: number) => {
    return chatStore.getChatByUuidAndIndex(uuid, index)
  }

  const addChat = (uuid: number, chat: Chat.Chat) => {
    chatStore.addChatByUuid(uuid, chat)
  }

  const updateChat = (uuid: number, index: number, chat: Chat.Chat) => {
    chatStore.updateChatByUuid(uuid, index, chat)
  }

  const updateChatSome = (uuid: number, index: number, chat: Partial<Chat.Chat>) => {
    chatStore.updateChatSomeByUuid(uuid, index, chat)
  }

  const getSessionConfig = (uuid: number): Chat.ChatOptions => {
    const session = chatStore.getChatSessionByUuid(uuid)
    const opt = session?.opt

    const hostUrl = new URL(userStore.userConfig.host)
    const host = `${hostUrl.protocol}//${hostUrl.host}`
    const defaultOpt = {
      apiKey: userStore.userConfig.apiKey,
      host,
      proxy: userStore.userConfig.proxy,
      model: userStore.userConfig.modelName,
      systemMessage: _getDefaultSystemMessage(),
      temperature: 0.6,
    }
    return _.merge(defaultOpt, opt)
  }

  const buildRequestMessages = async (uuid: number, toIndex: number) => {
    const chatSession = chatStore.getChatSessionByUuid(uuid)
    const MaxTokenNumber = 4096
    const ds = chatSession?.data ? chatSession?.data : []
    const messages: Chat.RequestMessage[] = []
    if (toIndex >= ds.length)
      return []

    let tokenTotalNum = 0
    const systemMessage = chatSession?.opt?.systemMessage ? chatSession?.opt.systemMessage : _getDefaultSystemMessage()
    const enableSysMsg = (systemMessage && systemMessage.length > 0)
    if (enableSysMsg)
      tokenTotalNum += _getTokenCount(systemMessage)

    let currentIndex = toIndex
    while (currentIndex >= 0) {
      const chat = ds[currentIndex]
      let tokenNum = chat.tokenNum
      if (tokenNum === undefined)
        tokenNum = _getTokenCount(chat.text)
      chat.tokenNum = tokenNum
      chatStore.updateChatSomeByUuid(uuid, currentIndex, chat)
      tokenTotalNum += tokenNum
      if (tokenTotalNum >= MaxTokenNumber)
        break

      messages.unshift({ role: chat.rule, content: chat.text })
      currentIndex -= 1
    }
    // add last message
    if (enableSysMsg)
      messages.unshift({ role: 'system', content: systemMessage })
    return messages
  }

  function _getTokenCount(text: string) {
    text = text.replace(/<\|endoftext\|>/g, '')
    const encoded = tokenizer.encode(text)
    return encoded.bpe.length
  }

  function _getDefaultSystemMessage() {
    const currentDate = (/* @__PURE__ */ new Date()).toISOString().split('T')[0]
    return `You are ChatGPT, a large language model trained by OpenAI. Answer as concisely as possible.
    Knowledge cutoff: 2021-09-01
    Current date: ${currentDate}`
  }

  return {
    addChat,
    updateChat,
    updateChatSome,
    getChatByUuidAndIndex,
    getSessionConfig,
    buildRequestMessages,
  }
}
