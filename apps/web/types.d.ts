// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type ResponsePayload<T = any> = {
	status: number;
	message: string;
	data?: T;
};

export type Note = {
	note_id: string;
	title: string;
	tags: string[];
	content: Array;
};

export type NoteCreatePayload = Pick<Note, 'title' | 'tags' | 'content'>;
