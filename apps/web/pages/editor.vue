<script lang="ts" setup>
import { NotEditor } from '@notisfly/editor'
import { createEmptyDoc } from '@blocksuite/presets'
import * as http from '~/utils/http'
import { getFullUrl } from '~/constants';
import * as Yjs from 'yjs'
import { useColorMode } from '@vueuse/core'
import type { DocCollection } from '@blocksuite/store';
import type { Note, NoteCreatePayload } from '~/types';
import { useToast } from '~/components/toast/use-toast'
const { toast } = useToast()

definePageMeta({
    title: 'Editor',
    description: 'Editor page',
    layout: 'data-board'
})

let docId = "tsfAZqaQoXtZbSNL"

const mode = useColorMode()

mode.value = 'dark'

const container = ref<HTMLElement | null>(null)
const editor = ref<NotEditor>(new NotEditor())

onMounted(async () => {
    container.value?.appendChild(editor.value)
    let doc = createEmptyDoc().init()

    // request doc from server
    // YJp6MsqxyXSpzjwg
    const { data, status } = await http.get<Note>(
        getFullUrl(`/api/notes/${docId}`),
        {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
            },
        });

    console.log(data)

    if (status === 200) {
        const content = new Uint8Array(data!.content)
        Yjs.applyUpdateV2(doc.spaceDoc, content)

        console.log(doc.spaceDoc.getArray('tags').toArray())

        // const yText = doc.spaceDoc.getText('title');

        // yText.insert(0, data!.title);
    }

    editor.value.setDoc(doc)
})

const onCreateClicked = async () => {
    console.log('create clicked')
    const { data, status } = await http.put<{ note_id: string }>(getFullUrl('/api/notes/create'), {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify({
            note_type: 'note',
        }),
    })

    if (status === 201) {
        console.log(data)
        docId = data?.note_id ?? 'tsfAZqaQoXtZbSNL'
        const doc = createEmptyDoc().init()
        editor.value.setDoc(doc)
    }
}

const onSaveClicked = async () => {
    console.log('save clicked')
    const collection = editor.value.doc?.collection!
    console.log(collection)
    console.log('synced')
    const doc = editor.value.doc!
    const content = Yjs.encodeStateAsUpdateV2(doc.spaceDoc)
    const noteCreatePayload: NoteCreatePayload = {
        title: doc.meta?.title!,
        tags: Array.from(doc.meta?.tags!),
        content: Array.from(content),
    }
    // update note remote content
    try {
        const { status } = await http.patch<Note>(getFullUrl(`/api/notes/${docId}`), {
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
        } else {
            console.error('save failed')
            toast({
                title: '保存失败',
                description: '保存失败',
                duration: 5000,
            })
        }
    } catch (e) {
        console.error(e)
        toast({
            title: '保存失败',
            description: '保存失败',
            duration: 5000,
        })
        return
    }
}

</script>

<template>
    <div class="h-100vh overflow-auto max-w-768px mx-auto">
        <div class="h-64px">
            <div class="flex justify-start items-center h-full px-4">
                <div class="text-[var(--text-500)]">Editor</div>
                <button @click="onCreateClicked" class="ml-auto">新建</button>
                <button @click="onSaveClicked" class="ml-16px">保存</button>
            </div>
        </div>
        <div class="bg-[var(--bg-200)]" ref="container">
        </div>
    </div>
</template>
