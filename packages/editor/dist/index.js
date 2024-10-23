var __esDecorate = (this && this.__esDecorate) || function (ctor, descriptorIn, decorators, contextIn, initializers, extraInitializers) {
    function accept(f) { if (f !== void 0 && typeof f !== "function") throw new TypeError("Function expected"); return f; }
    var kind = contextIn.kind, key = kind === "getter" ? "get" : kind === "setter" ? "set" : "value";
    var target = !descriptorIn && ctor ? contextIn["static"] ? ctor : ctor.prototype : null;
    var descriptor = descriptorIn || (target ? Object.getOwnPropertyDescriptor(target, contextIn.name) : {});
    var _, done = false;
    for (var i = decorators.length - 1; i >= 0; i--) {
        var context = {};
        for (var p in contextIn) context[p] = p === "access" ? {} : contextIn[p];
        for (var p in contextIn.access) context.access[p] = contextIn.access[p];
        context.addInitializer = function (f) { if (done) throw new TypeError("Cannot add initializers after decoration has completed"); extraInitializers.push(accept(f || null)); };
        var result = (0, decorators[i])(kind === "accessor" ? { get: descriptor.get, set: descriptor.set } : descriptor[key], context);
        if (kind === "accessor") {
            if (result === void 0) continue;
            if (result === null || typeof result !== "object") throw new TypeError("Object expected");
            if (_ = accept(result.get)) descriptor.get = _;
            if (_ = accept(result.set)) descriptor.set = _;
            if (_ = accept(result.init)) initializers.unshift(_);
        }
        else if (_ = accept(result)) {
            if (kind === "field") initializers.unshift(_);
            else descriptor[key] = _;
        }
    }
    if (target) Object.defineProperty(target, contextIn.name, descriptor);
    done = true;
};
var __runInitializers = (this && this.__runInitializers) || function (thisArg, initializers, value) {
    var useValue = arguments.length > 2;
    for (var i = 0; i < initializers.length; i++) {
        value = useValue ? initializers[i].call(thisArg, value) : initializers[i].call(thisArg);
    }
    return useValue ? value : void 0;
};
import { css, html } from 'lit';
import { repeat } from 'lit/directives/repeat.js';
import { customElement, property, query } from 'lit/decorators.js';
import { PageEditor } from '@blocksuite/presets';
import { effects as presetsEffects } from '@blocksuite/presets/effects';
import { effects as blocksEffects } from '@blocksuite/blocks/effects';
import { SignalWatcher, WithDisposable } from '@blocksuite/global/utils';
import { ShadowlessElement } from '@blocksuite/block-std';
// import { RichText } from '@blocksuite/blocks'
import * as Y from 'yjs';
import { enter } from './event.js';
presetsEffects();
blocksEffects();
let NotEditor = (() => {
    let _classDecorators = [customElement('not-editor')];
    let _classDescriptor;
    let _classExtraInitializers = [];
    let _classThis;
    let _classSuper = SignalWatcher(WithDisposable(ShadowlessElement));
    let _name_decorators;
    let _name_initializers = [];
    let _name_extraInitializers = [];
    let _pageEditor_decorators;
    let _pageEditor_initializers = [];
    let _pageEditor_extraInitializers = [];
    let _tags_decorators;
    let _tags_initializers = [];
    let _tags_extraInitializers = [];
    let _doc_decorators;
    let _doc_initializers = [];
    let _doc_extraInitializers = [];
    let _title_decorators;
    let _title_initializers = [];
    let _title_extraInitializers = [];
    let _isReadonly_decorators;
    let _isReadonly_initializers = [];
    let _isReadonly_extraInitializers = [];
    let _yText_decorators;
    let _yText_initializers = [];
    let _yText_extraInitializers = [];
    let _readonly_decorators;
    let _readonly_initializers = [];
    let _readonly_extraInitializers = [];
    let _titleInput_decorators;
    let _titleInput_initializers = [];
    let _titleInput_extraInitializers = [];
    var NotEditor = class extends _classSuper {
        static { _classThis = this; }
        static {
            const _metadata = typeof Symbol === "function" && Symbol.metadata ? Object.create(_classSuper[Symbol.metadata] ?? null) : void 0;
            _name_decorators = [property({ type: String })];
            _pageEditor_decorators = [property({ attribute: false })];
            _tags_decorators = [property({ attribute: false })];
            _doc_decorators = [property({ attribute: false })];
            _title_decorators = [property({ type: String })];
            _isReadonly_decorators = [property({ type: Boolean })];
            _yText_decorators = [property({ attribute: false })];
            _readonly_decorators = [property({ attribute: false })];
            _titleInput_decorators = [query('#title-input', false)];
            __esDecorate(this, null, _name_decorators, { kind: "accessor", name: "name", static: false, private: false, access: { has: obj => "name" in obj, get: obj => obj.name, set: (obj, value) => { obj.name = value; } }, metadata: _metadata }, _name_initializers, _name_extraInitializers);
            __esDecorate(this, null, _pageEditor_decorators, { kind: "accessor", name: "pageEditor", static: false, private: false, access: { has: obj => "pageEditor" in obj, get: obj => obj.pageEditor, set: (obj, value) => { obj.pageEditor = value; } }, metadata: _metadata }, _pageEditor_initializers, _pageEditor_extraInitializers);
            __esDecorate(this, null, _tags_decorators, { kind: "accessor", name: "tags", static: false, private: false, access: { has: obj => "tags" in obj, get: obj => obj.tags, set: (obj, value) => { obj.tags = value; } }, metadata: _metadata }, _tags_initializers, _tags_extraInitializers);
            __esDecorate(this, null, _doc_decorators, { kind: "accessor", name: "doc", static: false, private: false, access: { has: obj => "doc" in obj, get: obj => obj.doc, set: (obj, value) => { obj.doc = value; } }, metadata: _metadata }, _doc_initializers, _doc_extraInitializers);
            __esDecorate(this, null, _title_decorators, { kind: "accessor", name: "title", static: false, private: false, access: { has: obj => "title" in obj, get: obj => obj.title, set: (obj, value) => { obj.title = value; } }, metadata: _metadata }, _title_initializers, _title_extraInitializers);
            __esDecorate(this, null, _isReadonly_decorators, { kind: "accessor", name: "isReadonly", static: false, private: false, access: { has: obj => "isReadonly" in obj, get: obj => obj.isReadonly, set: (obj, value) => { obj.isReadonly = value; } }, metadata: _metadata }, _isReadonly_initializers, _isReadonly_extraInitializers);
            __esDecorate(this, null, _yText_decorators, { kind: "accessor", name: "yText", static: false, private: false, access: { has: obj => "yText" in obj, get: obj => obj.yText, set: (obj, value) => { obj.yText = value; } }, metadata: _metadata }, _yText_initializers, _yText_extraInitializers);
            __esDecorate(this, null, _readonly_decorators, { kind: "accessor", name: "readonly", static: false, private: false, access: { has: obj => "readonly" in obj, get: obj => obj.readonly, set: (obj, value) => { obj.readonly = value; } }, metadata: _metadata }, _readonly_initializers, _readonly_extraInitializers);
            __esDecorate(this, null, _titleInput_decorators, { kind: "accessor", name: "titleInput", static: false, private: false, access: { has: obj => "titleInput" in obj, get: obj => obj.titleInput, set: (obj, value) => { obj.titleInput = value; } }, metadata: _metadata }, _titleInput_initializers, _titleInput_extraInitializers);
            __esDecorate(null, _classDescriptor = { value: _classThis }, _classDecorators, { kind: "class", name: _classThis.name, metadata: _metadata }, null, _classExtraInitializers);
            NotEditor = _classThis = _classDescriptor.value;
            if (_metadata) Object.defineProperty(_classThis, Symbol.metadata, { enumerable: true, configurable: true, writable: true, value: _metadata });
        }
        static { this.styles = css `
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
	`; }
        #name_accessor_storage = __runInitializers(this, _name_initializers, 'world');
        get name() { return this.#name_accessor_storage; }
        set name(value) { this.#name_accessor_storage = value; }
        #pageEditor_accessor_storage = (__runInitializers(this, _name_extraInitializers), __runInitializers(this, _pageEditor_initializers, void 0));
        get pageEditor() { return this.#pageEditor_accessor_storage; }
        set pageEditor(value) { this.#pageEditor_accessor_storage = value; }
        #tags_accessor_storage = (__runInitializers(this, _pageEditor_extraInitializers), __runInitializers(this, _tags_initializers, []));
        get tags() { return this.#tags_accessor_storage; }
        set tags(value) { this.#tags_accessor_storage = value; }
        #doc_accessor_storage = (__runInitializers(this, _tags_extraInitializers), __runInitializers(this, _doc_initializers, void 0));
        get doc() { return this.#doc_accessor_storage; }
        set doc(value) { this.#doc_accessor_storage = value; }
        #title_accessor_storage = (__runInitializers(this, _doc_extraInitializers), __runInitializers(this, _title_initializers, ''));
        get title() { return this.#title_accessor_storage; }
        set title(value) { this.#title_accessor_storage = value; }
        #isReadonly_accessor_storage = (__runInitializers(this, _title_extraInitializers), __runInitializers(this, _isReadonly_initializers, false));
        get isReadonly() { return this.#isReadonly_accessor_storage; }
        set isReadonly(value) { this.#isReadonly_accessor_storage = value; }
        #yText_accessor_storage = (__runInitializers(this, _isReadonly_extraInitializers), __runInitializers(this, _yText_initializers, void 0));
        get yText() { return this.#yText_accessor_storage; }
        set yText(value) { this.#yText_accessor_storage = value; }
        #readonly_accessor_storage = (__runInitializers(this, _yText_extraInitializers), __runInitializers(this, _readonly_initializers, false));
        get readonly() { return this.#readonly_accessor_storage; }
        set readonly(value) { this.#readonly_accessor_storage = value; }
        setTitle(event) {
            this.title = event.target.value;
            this.yText?.delete(0, this.yText.length);
            this.yText?.insert(0, this.title);
            this.doc.collection.setDocMeta(this.doc.id, { title: this.title });
        }
        #titleInput_accessor_storage = (__runInitializers(this, _readonly_extraInitializers), __runInitializers(this, _titleInput_initializers, void 0));
        get titleInput() { return this.#titleInput_accessor_storage; }
        set titleInput(value) { this.#titleInput_accessor_storage = value; }
        setDoc(doc) {
            this.doc = doc;
            if (!doc) {
                return;
            }
            // @ts-ignore
            this.yText = doc.root.title.yText;
            {
                // init tags
                const tags = doc.spaceDoc.getArray('tags');
                console.log('tags', this.doc.meta.tags);
                tags.insert(0, [...(this.doc.meta.tags || [])]);
            }
            this.tags = doc.spaceDoc.getArray('tags').toArray();
            this.readonly = doc.readonly;
            this.pageEditor.doc = doc;
            if (this.titleInput) {
                this.titleInput.value = this.yText.toString();
            }
        }
        addTag(event) {
            const target = event.target;
            const value = target.value;
            const tags = this.doc?.spaceDoc.getArray('tags');
            tags?.push([value]);
            this.tags = [...this.tags, value];
            target.value = '';
            // this.requestUpdate();
        }
        connectedCallback() {
            super.connectedCallback();
            setTimeout(() => {
                if (this.doc && this.titleInput && this.yText) {
                    this.titleInput.value = this.yText.toString();
                }
            });
        }
        render() {
            return html `<div class="editor-container">
			<div class="editor-container-header">
				<div class="title">
					<input .disabled="${this.readonly}" autocomplete="off" id="title-input" @input="${this.setTitle}" placeholder="标题" />
				</div>
				<div class="tags">
					${repeat(this.tags, (tag) => tag, (tag) => html `<span class="tag">#${tag}</span>`)}
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
            __runInitializers(this, _titleInput_extraInitializers);
            this.pageEditor = new PageEditor();
        }
        static {
            __runInitializers(_classThis, _classExtraInitializers);
        }
    };
    return NotEditor = _classThis;
})();
export { NotEditor };
//# sourceMappingURL=index.js.map