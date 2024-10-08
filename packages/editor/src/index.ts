import { html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { AffineEditorContainer, createEmptyDoc } from '@blocksuite/presets';
import { effects as presetsEffects } from '@blocksuite/presets/effects';
import { effects as blocksEffects } from '@blocksuite/blocks/effects';
import type { Doc } from '@blocksuite/store';
import { SignalWatcher, WithDisposable } from '@blocksuite/global/utils';
import { ShadowlessElement } from '@blocksuite/block-std';

presetsEffects();
blocksEffects();

@customElement('lit-test')
export class LitTest extends SignalWatcher(WithDisposable(ShadowlessElement)) {
	@property({ type: String })
	accessor name = 'world';

	@property({ attribute: false })
	accessor container: AffineEditorContainer;

	@property({ attribute: false })
	accessor doc: Doc;

	override render() {
		return html`<div>
            <h1>Hello, ${this.name}!</h1>
            ${this.container}
        </div>`;
	}

	constructor() {
		super();
		this.container = new AffineEditorContainer();
		this.doc = createEmptyDoc().init();
		this.container.doc = this.doc;
	}
}
