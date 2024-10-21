<script lang="ts" setup>
import { ref, onMounted, inject, useTemplateRef } from 'vue'
import { NotEditor } from '@notisfly/editor'
import { Icon } from '@iconify/vue'
import { useToast } from '~/components/toast/use-toast'
import * as http from '~/utils/http'
import { Job, type DocSnapshot } from '@blocksuite/store'
import { getFullUrl, STATUS_FETCH } from '~/constants'
import type { Note } from '~/types'
import { createEmptyDoc } from '@blocksuite/presets'
import type { AppState } from './ManualProvider.vue'

const route = useRoute()
const { toast } = useToast()
const { collection } = inject<AppState>('appState')!

const editor = ref<NotEditor>(new NotEditor())
const container = useTemplateRef<HTMLElement>("note-editor-container")
const isLoading = ref(false);
const isSaving = ref(false);
const isMaximized = ref(false);
const isAutoSaveing = ref(false);
const isOpenAutoSave = ref(false);


const noteId = route.params.id as string

const initPage = async (noteId: string) => {
    const { data, message, status } = await http.get<DocSnapshot>(getFullUrl(`/api/notes/${noteId}`), {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
    })
    if (status === STATUS_FETCH.SUCCESS && data) {
        const job = new Job({ collection })
        const doc = job.collection.getDoc(noteId)
        if (doc) {
            job.collection.removeDoc(doc.id)
        }
        console.log(data)
        const newDoc = (await job.snapshotToDoc(data))!;
        console.log(newDoc.meta)
        collection.meta.setDocMeta(newDoc.id, {
            type: 'note',
            tags: data.meta?.tags ?? [],
        })
        editor.value.setDoc(newDoc)
        container.value?.appendChild(editor.value);
    } else {
        toast.error({
            title: 'Fetch note data failed',
            description: message,
        })
    }
}

onMounted(async () => {
    initPage(noteId)
})

const onSaveClicked = async () => {
    isSaving.value = true;
    const job = new Job({ collection })

    const doc = collection.getDoc(noteId)

    if (!doc) {
        toast.error({
            title: '文档不存在',
        })
        return
    }

    const tags = doc.spaceDoc.getArray<string>('tags').toArray()

    const docJson = await job.docToSnapshot(doc)

    if (!docJson) {
        toast.error({
            title: '文档转换失败',
        })
        return
    }

    docJson.meta.tags = tags

    console.log(docJson)

    const { status, message } = await http.patch(getFullUrl(`/api/notes`), {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify(docJson),
    })

    isSaving.value = false;
}

</script>

<template>
    <div class="note-editor">
        <div class="editor-btns py-8px flex justify-end">
            <Icon v-if="!isSaving" size="6" @click="onSaveClicked"
                class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
                icon="lucide:save" />
            <Icon v-else size="6"
                class="animate-spin ml-auto mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2"
                icon="lucide:loader" />
            <Icon v-if="isOpenAutoSave" @click="isOpenAutoSave = !isOpenAutoSave"
                :class="isAutoSaveing ? 'text-[var(--accent-100)]' : ''" icon="lucide:wifi" size="6"
                class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
            <Icon v-else @click="isOpenAutoSave = !isOpenAutoSave"
                :class="isAutoSaveing ? 'text-[var(--accent-100)]' : ''" icon="lucide:wifi-off" size="6"
                class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
            <Icon v-if="!isMaximized" @click="isMaximized = !isMaximized" icon="lucide:maximize" size="6"
                class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
            <Icon v-else @click="isMaximized = !isMaximized" icon="lucide:minimize" size="6"
                class="mr-8px p-4px bg-[var(--bg-100)] hover:text-[var(--accent-100)] cursor-pointer rounded-1/2" />
        </div>
        <div ref="note-editor-container" class="note-editor-container"></div>
    </div>
</template>

<style lang="scss" scoped>
.note-editor {
    container-name: editor-btns;
    container-type: inline-size;
}

.editor-btns {
    padding-right: var(--affine-editor-side-padding, 24px);
    padding-left: var(--affine-editor-side-padding, 24px);
    max-width: var(--affine-editor-width);

    width: 100%;
    margin: 0 auto;
}

@container editor-btns (width <=640px) {
    .editor-btns {
        padding-right: 24px;
        padding-left: 24px;
    }
}
</style>