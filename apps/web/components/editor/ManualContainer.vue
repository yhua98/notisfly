<script setup lang="ts">
import { inject, ref, toValue } from 'vue';
import { onEffectCleanup } from "@vue/reactivity"
import type { AppState } from './Provider.vue';
import Tags from './tags/Tags.vue';
import { AffineEditorContainer, createEmptyDoc } from '@blocksuite/presets'
import type { Doc } from '@blocksuite/store';
import { RefNodeSlotsExtension, RefNodeSlotsProvider } from '@blocksuite/blocks';
import { Icon } from '@iconify/vue'
import type { NoteCreatePayload } from '~/types';
import * as Yjs from 'yjs'
import { getFullUrl } from '~/constants';
import File from '~/components/icons/File.vue';
import { useToast } from '~/components/toast/use-toast'

const { toast } = useToast()

// biome-ignore lint/style/noNonNullAssertion: <explanation>
const { collection, docId: docIdRef, docSyncUrl } = inject<AppState>('appState')!;
const editorContainerRef = ref<HTMLDivElement>();

const tags = ref<string[]>([]);
const isInit = ref(false)

const editor: AffineEditorContainer = new AffineEditorContainer();
const editorMode = ref<'edgeless' | 'page'>('edgeless');
const refNodeSlotsExtension = RefNodeSlotsExtension();
const isLoading = ref(false);
const isSaving = ref(false);
const isMaximized = ref(false);
const isAutoSaveing = ref(false);
const isOpenAutoSave = ref(false);
const isDiary = ref(false);
const doc = ref<Doc>()

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
  if (!docId) return;
  isLoading.value = true;
  await collection.waitForSynced();

  let _doc = null

  // request short_note remote content
  {
    // biome-ignore lint/complexity/noExtraBooleanCast: <explanation>
    const url = getFullUrl(!!docSyncUrl.value ? docSyncUrl.value?.() : `/api/shortnote/note/${docId}`);
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
      },
    });
    const { status, data } = await response.json();
    if (status === 200 && data) {
      _doc = (() => {
        let doc = collection.getDoc(docId)
        isDiary.value = false
        if (!doc && !!data.dnid) {
          isDiary.value = true
          doc = createEmptyDoc().init();
          doc.collection.start()
        }
        return doc
      })()
      if (!_doc) {
        console.error('doc not found: docId = ', docId)
        return
      };
      if (!isDiary.value) {
        _doc.load();
      }
      // content is empty, do nothing
      if (data.content.length) {
        const content = new Uint8Array(data.content)
        Yjs.applyUpdateV2(_doc.spaceDoc, content)
      };
      _doc.resetHistory()
    }
  }
  if (!isDiary.value) {
    if (!docId) return;
    _doc = collection.getDoc(docId)
    if (!_doc) {
      console.error('doc not found: docId = ', docId)
      return
    };
  }
  // biome-ignore lint/style/noNonNullAssertion: <explanation>
  const meta = _doc.collection.meta.getDocMeta(docId)!
  _doc.load();
  if (_doc.isEmpty) {
    const pageBlockId = _doc.addBlock('affine:page', {});
    _doc.addBlock('affine:surface', {}, pageBlockId);
    const noteId = _doc.addBlock('affine:note', {}, pageBlockId);
    _doc.addBlock('affine:paragraph', {}, noteId);
  }
  if (!_doc.root) {
    await new Promise(resolve => _doc.slots.rootAdded.once(resolve));
  }
  if (isDiary.value) {
    // doc.addBlock("prop:title", { text: new Text("20241007日志") })
    // clear title
    _doc.root.title.yText.delete(0, _doc.root.title.yText.length)
    _doc.root.title.yText.insert(0, `📌 ${docId}日志`)
    // doc.root.yBlock.get("prop:title").insert(1, "20241007日志")
  }
  editor.doc = _doc;
  doc.value = _doc

  if (editor && editorContainerRef.value && !isInit.value) {
    isInit.value = true;
    editor.pageSpecs = [refNodeSlotsExtension, ...editor.pageSpecs]

    editorContainerRef.value.appendChild(editor);
    editor.std.get(RefNodeSlotsProvider).docLinkClicked.on((event) => {
      console.log('docLinkClicked', event.pageId)
      docIdRef.value = event.pageId
    });
  }
  tags.value = Array.from(meta?.tags ?? [])
  window.doc = _doc
  setTimeout(() => {
    isLoading.value = false;
    toast({
      title: '加载成功',
      description: '文章加载成功',
      duration: 1000,
    })
  }, 300);
})

watchEffect(() => {
  if (isOpenAutoSave.value) {
    const interval = setInterval(() => {
      isAutoSaveing.value = true;
      onSaveClicked().finally(() => {
        isAutoSaveing.value = false;
      })
    }, 1000 * 60);
    onEffectCleanup(() => {
      clearInterval(interval);
    })
  }
})

const onSaveClicked = async () => {
  isSaving.value = true;
  if (isDiary.value) {
    const _doc = toValue(doc)
    await _doc.collection.waitForSynced();
    // if (docIdRef.value === "20241008") {
    //   _doc?.clear()
    // }
    const content = Yjs.encodeStateAsUpdateV2(_doc.spaceDoc)
    const noteCreatePayload: NoteCreatePayload = {
      title: _doc.meta.title,
      tags: Array.from(_doc.meta.tags),
      content: Array.from(content),
    }
    {
      const response = await fetch(getFullUrl(`/api/shortnote/diary/${docIdRef.value}`), {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify(noteCreatePayload),
      });
      const { status } = await response.json();
      if (status === 200) {
        console.log('save success')
      }
    }
    isSaving.value = false;
    toast({
      title: '保存成功',
      description: '保存成功',
      duration: 5000,
    })
  } else {
    if (!docIdRef.value) return;
    await collection.waitForSynced();
    const doc = collection.getDoc(docIdRef.value)
    if (!doc || !doc.meta) return;
    const content = Yjs.encodeStateAsUpdateV2(doc.spaceDoc)
    const noteCreatePayload: NoteCreatePayload = {
      title: doc.meta.title,
      tags: Array.from(doc.meta.tags),
      content: Array.from(content),
    }
    // update short_note remote content
    {
      const response = await fetch(getFullUrl(`/api/shortnote/note/${docIdRef.value}`), {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify(noteCreatePayload),
      });
      const { status } = await response.json();
      if (status === 200) {
        console.log('save success')
      }
    }
    isSaving.value = false;
    toast({
      title: '保存成功',
      description: '保存成功',
      duration: 5000,
    })
  }
}
</script>

<template>
  <div
    :class="isMaximized ? 'p-32px fixed top-0 left-0 z-100 w-full h-full flex justify-center items-center bg-[var(--bg-100)]' : ''">
    <div :class="isMaximized ? 'h-full w-768px shadow-xl' : ''" class="transition-all transition-2000 min-h-200px editor-container-wrapper relative border-(1px solid [var(--bg-200)]) 
    bg-[var(--bg-50)] rounded-16px flex flex-col overflow-hidden">
      <div class=" py-4px px-16px flex text-[var(--text-200)] ">
        <Icon v-if="!isSaving" @click="onSaveClicked" size="6"
          class="ml-auto mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
          icon="lucide:save" />
        <Icon v-else size="6"
          class="animate-spin ml-auto mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
          icon="lucide:loader" />
        <Icon v-if="isOpenAutoSave" @click="isOpenAutoSave = !isOpenAutoSave"
          :class="isAutoSaveing ? 'text-[var(--accent-100)]' : ''" icon="lucide:wifi" size="6"
          class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
        <Icon v-else @click="isOpenAutoSave = !isOpenAutoSave" :class="isAutoSaveing ? 'text-[var(--accent-100)]' : ''"
          icon="lucide:wifi-off" size="6"
          class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
        <Icon v-if="!isMaximized" @click="isMaximized = !isMaximized" icon="lucide:maximize" size="6"
          class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
        <Icon v-else @click="isMaximized = !isMaximized" icon="lucide:minimize" size="6"
          class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
        <Icon size="6" class="p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
          @click="editorMode = 'page'" v-if="editorMode === 'edgeless'" icon="lucide:drafting-compass" />
        <Icon size="6" class="p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
          @click="editorMode = 'edgeless'" v-else icon="lucide:notepad-text" />
      </div>
      <div class="transition transition-2000 editor-container bg-[var(--bg-50)] grow-1 overflow-auto"
        ref="editorContainerRef"></div>
      <div class="pb-16px pt-8px px-8px bg-[var(--bg-50)] border-t-1px border-t-[var(--bg-150)]">
        <div class="">
          <Tags v-model="tags" />
        </div>
      </div>
      <div v-if="isLoading"
        class="absolute right-0 bottom-0 left-0 top-0 bg-[var(--bg-50)] flex items-center justify-center">
        <div class="relative">
          <File class="text-[var(--text-200)]" size="8" />
          <Icon size="5" class="animate-spin absolute top-12px left-0" icon="lucide:loader" />
        </div>
      </div>
      <div v-if="!docIdRef"
        class="text-[var(--text-200)] absolute right-0 bottom-0 left-0 top-0 bg-[var(--bg-50)] flex flex-col items-center justify-center">
        <Icon icon="lucide:file-pen-line" size="12" />
        <div class="mt-16px">没有选择文档</div>
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
