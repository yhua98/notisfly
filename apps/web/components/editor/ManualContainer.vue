<script setup lang="ts">
import { inject, ref } from 'vue';
import { onEffectCleanup } from "@vue/reactivity"
import type { AppState } from './ManualProvider.vue';
import Tags from './tags/Tags.vue';
import { AffineEditorContainer } from '@blocksuite/presets'
import { NotEditor } from '@notisfly/editor'
import { RefNodeSlotsExtension, RefNodeSlotsProvider } from '@blocksuite/blocks';
import { Icon } from '@iconify/vue'
import type { Note, NoteCreatePayload } from '~/types';
import * as Yjs from 'yjs'
import { getFullUrl } from '~/constants';
import File from '~/components/icons/File.vue';
import { useToast } from '~/components/toast/use-toast'
import * as http from '~/utils/http'
import { Job } from '@blocksuite/store'

const { toast } = useToast()

// biome-ignore lint/style/noNonNullAssertion: <explanation>
const { collection, docId: docIdRef } = inject<AppState>('appState')!;
const editorContainerRef = ref<HTMLDivElement>();

const tags = ref<string[]>([]);
const isInit = ref(false)

// const editor: AffineEditorContainer = new AffineEditorContainer();
const editor: NotEditor = new NotEditor()
const editorMode = ref<'edgeless' | 'page'>('edgeless');
const refNodeSlotsExtension = RefNodeSlotsExtension();
const isLoading = ref(false);
const isSaving = ref(false);
const isMaximized = ref(false);
const isAutoSaveing = ref(false);
const isOpenAutoSave = ref(false);

watch(() => editorMode.value, (mode) => {
  // if (mode === 'edgeless') {
  //   editor.mode = 'page';
  // } else {
  //   editor.mode = 'edgeless';
  // }
});

watch(() => tags.value, (tags) => {
  if (!docIdRef.value) return;
  collection.meta.setDocMeta(docIdRef.value, { tags: tags });
});

watch(() => docIdRef.value, async (docId) => {
  if (!docId) return;
  isLoading.value = true;
  await collection.waitForSynced();
  // request note remote content
  {
    const { data, status } = await http.get<Note>(
      getFullUrl(`/api/notes/${docId}`),
      {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
      });
    if (status === 200 && data) {
      const doc = collection.getDoc(docId)
      if (!doc) {
        console.error('doc not found: docId = ', docId)
        setTimeout(() => {
          isLoading.value = false;
          toast({
            title: '加载失败',
            description: '文章加载失败',
            duration: 1000,
          })
        }, 300);
        return
      };
      // content is empty, do nothing
      if (data.content.length > 0) {
        const content = new Uint8Array(data.content)
        Yjs.applyUpdateV2(doc.spaceDoc, content)
      };
    }
  }
  const doc = collection.getDoc(docId)
  if (!doc) {
    console.error('doc not found: docId = ', docId)
    setTimeout(() => {
      isLoading.value = false;
      toast({
        title: '加载失败',
        description: '文章加载失败',
        duration: 1000,
      })
    }, 300);
    return
  };
  // biome-ignore lint/style/noNonNullAssertion: <explanation>
  const meta = doc.collection.meta.getDocMeta(docId)!
  doc.load();
  if (doc.isEmpty) {
    const pageBlockId = doc.addBlock('affine:page', {});
    doc.addBlock('affine:surface', {}, pageBlockId);
    const noteId = doc.addBlock('affine:note', {}, pageBlockId);
    doc.addBlock('affine:paragraph', {}, noteId);
  }
  if (!doc.root) {
    await new Promise(resolve => doc.slots.rootAdded.once(resolve));
  }
  collection.meta.setDocMeta(docId, {
    title: doc.spaceDoc.getText("title").toString(),
    tags: doc.spaceDoc.getArray<string>("tags").toArray()
  });
  editor.setDoc(doc);

  if (editor && editorContainerRef.value && !isInit.value) {
    isInit.value = true;
    // editor.pageSpecs = [refNodeSlotsExtension, ...editor.pageSpecs]

    editorContainerRef.value.appendChild(editor);
    // editor.std.get(RefNodeSlotsProvider).docLinkClicked.on((event) => {
    //   console.log('docLinkClicked', event.pageId)
    //   docIdRef.value = event.pageId
    // });
  }
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
  if (!docIdRef.value) {
    toast({
      title: '保存失败',
      description: '没有选择文档',
      duration: 5000,
    })
    isSaving.value = false;
    return
  };
  await collection.waitForSynced();

  const doc = collection.getDoc(docIdRef.value)!

  // const { collection } = doc
  const job = new Job({ collection })
  const json = await job.docToSnapshot(doc)
  console.log('json', json)
  isSaving.value = false;
  return
  if (!doc || !doc.meta) {
    console.error('doc not found: docId = ', docIdRef.value)
    toast({
      title: '保存失败',
      description: '没有选择文档',
      duration: 5000,
    })
    isSaving.value = false;
    return
  };
  const content = Yjs.encodeStateAsUpdateV2(doc.spaceDoc)
  const noteCreatePayload: NoteCreatePayload = {
    title: doc.spaceDoc.getText("title").toString(),
    tags: doc.spaceDoc.getArray<string>("tags").toArray(),
    content: Array.from(content),
  }
  // update note remote content
  try {
    const { status } = await http.patch<Note>(getFullUrl(`/api/notes/${docIdRef.value}`), {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
      },
      body: JSON.stringify(noteCreatePayload),
    });
    if (status === 200) {
      console.log('save success')
      toast({
        title: '保存成功',
        description: '保存成功',
        duration: 5000,
      })
      isSaving.value = false;

    } else {
      console.error('save failed')
      toast({
        title: '保存失败',
        description: '保存失败',
        duration: 5000,
      })
      isSaving.value = false;
    }
  } catch (e) {
    console.error(e)
    toast({
      title: '保存失败',
      description: '保存失败',
      duration: 5000,
    })
    isSaving.value = false;
    return
  }
}
</script>

<template>
  <div
    :class="isMaximized ? 'p-32px fixed top-0 left-0 z-100 w-full h-full flex justify-center items-center bg-[var(--bg-100)]' : ''">
    <div :class="isMaximized ? 'h-full w-768px shadow-xl' : ''" class="transition-all h-full transition-2000 min-h-400px editor-container-wrapper relative border-(1px solid [var(--bg-200)]) 
    bg-[var(--bg-50)] rounded-0px flex flex-col overflow-hidden">
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
      <div class="h-full transition transition-2000 bg-[var(--bg-50)] grow-1 overflow-auto" ref="editorContainerRef">
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
    padding: 0 16px;
  }
}
</style>
