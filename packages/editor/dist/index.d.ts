import { AffineEditorContainer } from '@blocksuite/presets';
import type { Doc } from '@blocksuite/store';
import { ShadowlessElement } from '@blocksuite/block-std';
declare const LitTest_base: typeof ShadowlessElement & import("@blocksuite/global/utils").Constructor<import("@blocksuite/global/utils").DisposableClass>;
export declare class LitTest extends LitTest_base {
    accessor name: string;
    accessor container: AffineEditorContainer;
    accessor doc: Doc;
    render(): import("lit").TemplateResult<1>;
    constructor();
}
export {};
//# sourceMappingURL=index.d.ts.map