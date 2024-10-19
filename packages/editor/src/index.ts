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
	:host {
		padding: 24px;
	}

	.editor-container{
		padding: 24px;
	}

	.tags {
		
		padding-left: 16px;
	}

	.tags > input {
		outline: none;
		background: transparent;
		width: 4em;
		padding: 2px 8px;
		border: 1px solid #2200aa;
		border-radius: 8px;
	}

	.tag {
		margin-left: 4px;
	}
	.tag:first-child{
		margin-left: 0;
	}
	.tag:first-child {
		margin-left: 0;
		padding-left: 0;
	}
	.title {
		margin-bottom: 16px;
	}
	.title > input {
		background: transparent;
		outline: none;
		padding: 0 16px;
		font-size: 20px;
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

	setDoc(doc: Doc) {
		this.doc = doc;
		if (!doc) {
			return
		}
		this.yText = undefined as any;
		this.yText = doc.spaceDoc.getText('title');
		this.tags = doc.spaceDoc.getArray<string>('tags').toArray();
		this.pageEditor.doc = doc;

		if (this.titleInput) {
			this.titleInput.value = this.yText.toString();
		}

		console.log('doc', this.titleInput);
	}

	override connectedCallback() {
		super.connectedCallback();
		console.log('connected');

		if (this.doc && this.titleInput && this.yText) {
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
		console.log(value, tags, this.doc?.spaceDoc.toJSON());
		// this.requestUpdate();
	}

	@property({ type: String })
	override accessor title = '';

	@property({ type: Boolean })
	accessor isReadonly = false;

	@property({ attribute: false })
	accessor yText!: Y.Text;

	setTitle(event: InputEvent) {
		this.title = (event.target as HTMLInputElement).value;
		this.yText?.delete(0, this.yText.length);
		this.yText?.insert(0, this.title);
	}

	@query('input', false)
	accessor titleInput!: HTMLInputElement;

	override render() {
		return html`<div class="editor-container">
			<div class="title">
				<input id="title-input" @input="${this.setTitle}" placeholder="标题" />
			</div>
            <div class="tags">
				${repeat(this.tags, (tag) => tag, (tag) => html`<span class="tag">#${tag}</span>`)}
				<input @keyup="${enter.call(this, this.addTag)}" placeholder="+ tag" />
			</div>
			<div>
				${this.doc && this.pageEditor}
			</div>
        </div>`;
	}

	constructor() {
		super();
		this.pageEditor = new PageEditor();
	}
}
