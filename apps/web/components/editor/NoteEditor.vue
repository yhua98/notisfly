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
        const newDoc = (await job.snapshotToDoc(data))!;
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

</script>

<template>
    <div class="note-editor">
        <div ref="note-editor-container" class="note-editor-container pt-16px"></div>
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