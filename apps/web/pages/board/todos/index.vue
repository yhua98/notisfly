<script lang="ts" setup>
import { ref, onMounted, inject, useTemplateRef } from 'vue'
import { NotEditor } from '@notisfly/editor'
import { Icon } from '@iconify/vue'
import { useToast } from '~/components/toast/use-toast'
import * as http from '~/utils/http'
import { Job, type DocSnapshot } from '@blocksuite/store'
import { createDocJson, getFullUrl, STATUS_FETCH } from '~/constants'
import type { Note, NoteBlock } from '~/types'
import { createEmptyDoc } from '@blocksuite/presets'
import type { AppState } from '~/components/editor/ManualProvider.vue'

const { toast } = useToast()
const { collection } = inject<AppState>('appState')!

const editor = ref<NotEditor>(new NotEditor())
const container = useTemplateRef<HTMLElement>("note-editor-container")

const route = useRoute()

const noteId = route.params.id as string

const initPage = async () => {
    const { data, message, status } = await http.get<Note[]>(getFullUrl(`/api/notes/todos`), {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
    })
    if (status === STATUS_FETCH.SUCCESS && data) {
        const job = new Job({ collection })
        const doc = job.collection.getDoc('notisfly-todos')
        if (doc) {
            job.collection.removeDoc(doc.id)
        }

        // @ts-ignore
        const todos = data.map(item => ({
            flavour: item.blocks.flavour,
            id: item.blocks.id,
            props: item.blocks.props,
            type: item.blocks.type,
            version: item.blocks.version,
            children: [] as NoteBlock[]
        })) as NoteBlock[]

        // @ts-ignore
        const docJson = createDocJson("notisfly-todos", {
            id: "notisfly-todos",
            title: "🎯Todos",
            tags: [],
            createDate: Date.now(),
        }, todos) as DocSnapshot


        const _newDoc = (await job.snapshotToDoc(docJson))!;

        const newDoc = collection.getDoc('notisfly-todos', {
            readonly: true
        })!

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
    initPage()
})

</script>

<template>
    <div class="w-full">
        <div ref="note-editor-container" class="note-editor-container h-full"></div>
    </div>
</template>