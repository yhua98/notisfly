import { css, html } from 'lit';
import { repeat } from 'lit/directives/repeat.js';
import { customElement, property, query } from 'lit/decorators.js';
import { PageEditor } from '@blocksuite/presets';
import { effects as presetsEffects } from '@blocksuite/presets/effects';
import { effects as blocksEffects } from '@blocksuite/blocks/effects';
import type { Doc } from '@blocksuite/store';
import { SignalWatcher, WithDisposable } from '@blocksuite/global/utils';
import { ShadowlessElement } from '@blocksuite/block-std';
// import { RichText } from '@blocksuite/blocks'
import * as Y from 'yjs';
import { enter } from './event.js'

presetsEffects();
blocksEffects();

@customElement('not-editor')
export class NotEditor extends SignalWatcher(WithDisposable(ShadowlessElement)) {
	static override styles = css`
	.editor-container{
		box-sizing: border-box;
		height: 100%;
		display: flex;
		flex-direction: column;
		container-name: editor-container;
		container-type: inline-size;
	}

	.editor-container-header {
		padding-right: var(--affine-editor-side-padding, 24px);
		padding-left: var(--affine-editor-side-padding, 24px);
		margin: 0 auto;
		max-width: var(--affine-editor-width);
		width: 100%;
	}

	@container editor-container (width <= 640px) {
		.editor-container-header {
			padding-right: 24px;
			padding-left: 24px;
		}
	}

	.editor-container-page-editor{
		flex-grow: 1;
	}

	.editor-container-page-editor .h1 {
		font-size: 26px;
		font-weight: 600;
		height: auto !important;
	}

	.tags {
		margin-bottom: 8px;
		display: flex;
		align-items: center;
	}

	.tag {
		margin-left: 4px;
		background-color: var(--custom-tag-bg);
		padding: 4px 8px;
		color: var(--custom-tag-color);
	}
	.tag:first-child{
		margin-left: 0;
	}
	.tag:first-child {
		margin-left: 0;
	}
	.tags > input {
		outline: none;
		background: transparent;
		width: 4em;
		padding: 2px 8px;
		border: 2px solid var(--custom-tag-border);
		border-radius: 8px;
		margin-left: 8px;
	}

	.tags > input:focus {
		border-color: var(--custom-tag-focus-border);
	}

	.tags > input:first-child {
		margin-left: 0;
	}
	.title {
		margin-bottom: 8px;
	}
	.title > input {
		background: transparent;
		outline: none;
		font-size: 24px;
		font-weight: 600;
	}
	`

	@property({ type: String })
	accessor name = 'world';

	@property({ attribute: false })
	accessor pageEditor: PageEditor;

	@property({ attribute: false })
	accessor tags: string[] = [];

	@property({ attribute: false })
	accessor doc!: Doc;



	@property({ type: String })
	override accessor title = '';

	@property({ type: Boolean })
	accessor isReadonly = false;

	@property({ attribute: false })
	accessor yText!: Y.Text;

	@property({ attribute: false })
	accessor readonly = false;

	setTitle(event: InputEvent) {
		this.title = (event.target as HTMLInputElement).value;
		this.yText?.delete(0, this.yText.length);
		this.yText?.insert(0, this.title);
		this.doc.collection.setDocMeta(this.doc.id, { title: this.title });
	}

	@query('#title-input', false)
	accessor titleInput!: HTMLInputElement;

	setDoc(doc: Doc) {
		this.doc = doc;
		if (!doc) {
			return
		}
		// @ts-ignore
		this.yText = doc.root.title.yText;

		{
			// init tags
			const tags = doc.spaceDoc.getArray<string>('tags');
			console.log('tags', this.doc.meta!.tags);
			tags.insert(0, [...(this.doc.meta!.tags || [])]);
		}

		this.tags = doc.spaceDoc.getArray<string>('tags').toArray();
		this.readonly = doc.readonly;
		this.pageEditor.doc = doc;

		if (this.titleInput) {
			this.titleInput.value = this.yText.toString();
		}
	}

	private addTag(event: KeyboardEvent) {
		const target = event.target as HTMLInputElement;
		const value = target.value;
		const tags = this.doc?.spaceDoc.getArray<string>('tags');
		tags?.push([value]);
		this.tags = [...this.tags, value];
		target.value = '';
		// this.requestUpdate();
	}

	override connectedCallback() {
		super.connectedCallback();

		setTimeout(() => {
			if (this.doc && this.titleInput && this.yText) {
				this.titleInput.value = this.yText.toString();
			}
		})
	}

	override render() {
		return html`<div class="editor-container">
			<div class="editor-container-header">
				<div class="title">
					<input .disabled="${this.readonly}" autocomplete="off" id="title-input" @input="${this.setTitle}" placeholder="标题" />
				</div>
				<div class="tags">
					${repeat(this.tags, (tag) => tag, (tag) => html`<span class="tag">#${tag}</span>`)}
					<input .disabled="${this.readonly}" @keyup="${enter.call(this, this.addTag)}" placeholder="+ tag" />
				</div>
			</div>
			<div class="editor-container-page-editor">
				${this.doc && this.pageEditor}
			</div>
        </div>`;
	}

	constructor() {
		super();
		this.pageEditor = new PageEditor();
	}
}
