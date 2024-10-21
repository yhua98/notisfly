// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type ResponsePayload<T> = {
	status: number;
	message: string;
	data?: T;
}

export type Note = {
	id: string;
	type: string;
	meta: NoteMeta;
	blocks: NoteBlock;
}

export type NoteMeta = {
	id: string;
	tags: string[];
	title: string;
	createDate: number;
}

export type NoteBlock = {
	id: string;
	type: string;
	flavour: string;
	props: Record<string, unknown>;
	version: number;
	children: NoteBlock[];
}