declare namespace Chat {

	interface Chat {
		dateTime: string
		text: string
		inversion?: boolean
		error?: boolean
		loading?: boolean
	}

	interface History {
		title: string
		isEdit: boolean
		uuid: number
	}

	interface ChatState {
		active: number | null
		history: History[]
		chat: { uuid: number; data: Chat[] }[]
	}

	interface RequestMessage {
		role: string
		content: string
	}

	// interface ConversationRequest {
	// 	conversationId?: string
	// 	parentMessageId?: string
	// }

	// interface ConversationResponse {
	// 	options: ConversationRequest,
	// 	detail: string,
	// 	role: string,
	// 	finish_reason: string,
	// }
}
