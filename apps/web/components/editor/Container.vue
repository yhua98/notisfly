<script setup lang="ts">
import { inject, ref } from 'vue';
import type { AppState } from './Provider.vue';
import Tags from './tags/Tags.vue';
import { WebsocketProvider } from 'y-websocket';
import { AffineEditorContainer } from '@blocksuite/presets'
import { RefNodeSlotsExtension, RefNodeSlotsProvider } from '@blocksuite/blocks';
import { Icon } from '@iconify/vue'

const { saveMode = 'manual' } = defineProps<{
  saveMode: 'auto' | 'manual'
}>()

// biome-ignore lint/style/noNonNullAssertion: <explanation>
const { collection, docId: docIdRef } = inject<AppState>('appState')!;
const editorContainerRef = ref<HTMLDivElement>();

const tags = ref<string[]>([]);
const provider = ref<WebsocketProvider>()
const isInit = ref(false)
const title = ref<string>('')

const editor: AffineEditorContainer = new AffineEditorContainer();
const editorMode = ref<'edgeless' | 'page'>('edgeless');
const refNodeSlotsExtension = RefNodeSlotsExtension();

watch(() => editorMode.value, (mode) => {
  if (mode === 'edgeless') {
    editor.mode = 'page';
  } else {
    editor.mode = 'edgeless';
  }
});

watch(() => tags.value, (tags) => {
  if (!docIdRef.value) return;
  collection.meta.setDocMeta(docIdRef.value, { tags: tags });
});

watch(() => docIdRef.value, async (docId) => {
  await collection.waitForSynced();
  if (!docId) return;
  // biome-ignore lint/style/noNonNullAssertion: <explanation>
  const meta = collection.meta.getDocMeta(docId)!
  if (provider.value) {
    provider.value.disconnect()
    provider.value = undefined
  }
  const doc = collection.getDoc(docId)
  if (!doc) {
    console.error('doc not found: docId = ', docId)
    return
  };
  provider.value = new WebsocketProvider('ws://127.0.0.1:6789/api/notag/ws', `notfly-${docId}`, doc.spaceDoc)
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  provider.value.once('status', async (event: any) => {
    if (event.status === 'connected') {
      console.log('connected')
      doc.load();
      console.log("ROOT: ", !!doc.root)
      if (!doc.root) {
        await new Promise(resolve => doc.slots.rootAdded.once(resolve));
      }
      console.log("ROOT: ", !!doc.root)
      editor.doc = doc;

      if (editor && editorContainerRef.value && !isInit.value) {
        editor.pageSpecs = [refNodeSlotsExtension, ...editor.pageSpecs]

        editorContainerRef.value.appendChild(editor);
        editor.std.get(RefNodeSlotsProvider).docLinkClicked.on((event) => {
          console.log('docLinkClicked', event.pageId)
          docIdRef.value = event.pageId
        });
        isInit.value = true;
      }
      tags.value = Array.from(meta?.tags ?? [])
    }
  })
})
</script>

<template>
  <div
    class="editor-container-wrapper border-(1px solid [var(--bg-200)]) bg-[var(--bg-200)] rounded-16px flex flex-col overflow-hidden">
    <div class=" py-4px px-16px flex ">
      <Icon size="6" class="ml-auto p-4px bg-[var(--bg-100)] cursor-pointer rounded-1/2" @click="editorMode = 'page'"
        v-if="editorMode === 'edgeless'" icon="lucide:drafting-compass" />
      <Icon size="6" class="ml-auto p-4px bg-[var(--bg-100)] cursor-pointer rounded-1/2"
        @click="editorMode = 'edgeless'" v-else icon="lucide:notepad-text" />
    </div>
    <div class="editor-container bg-[var(--bg-50)] grow-1 overflow-auto" ref="editorContainerRef"></div>
    <div class="pb-16px pt-8px px-8px bg-[var(--bg-200)] border-t-1px border-t-[var(--bg-150)]">
      <div class="">
        <Tags v-model="tags" />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.editor-container-wrapper {

  // *::-webkit-scrollbar {
  //   height: 5px;
  // }

  // *::-webkit-scrollbar-thumb {
  //   background-color: var(--bg-200);
  //   border-radius: 0;
  // }

  :deep([data-block-is-title="true"]) {
    font-size: 20px;
    line-height: 1.5rem;
    font-weight: 400;
    margin-bottom: 0;
    margin-top: 0;
    padding-top: 16px;
    padding-bottom: 8px;
    padding-left: 8px;
    padding-right: 8px;
    border-bottom: 1px solid var(--bg-150);
  }

  :deep(.affine-page-root-block-container) {
    padding: 0 8px;
  }
}
</style>
