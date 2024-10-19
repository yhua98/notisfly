import { PageEditor } from '@blocksuite/presets';
import type { Doc } from '@blocksuite/store';
import { ShadowlessElement } from '@blocksuite/block-std';
import * as Y from 'yjs';
declare const NotEditor_base: typeof ShadowlessElement & import("@blocksuite/global/utils").Constructor<import("@blocksuite/global/utils").DisposableClass>;
export declare class NotEditor extends NotEditor_base {
    static styles: import("lit").CSSResult;
    accessor name: string;
    accessor pageEditor: PageEditor;
    accessor tags: string[];
    accessor doc: Doc;
    setDoc(doc: Doc): void;
    connectedCallback(): void;
    private addTag;
    accessor title: string;
    accessor isReadonly: boolean;
    accessor yText: Y.Text;
    setTitle(event: InputEvent): void;
    accessor titleInput: HTMLInputElement;
    render(): import("lit").TemplateResult<1>;
    constructor();
}
export {};
//# sourceMappingURL=index.d.ts.map