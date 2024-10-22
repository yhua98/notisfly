<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import type { AppState } from './editor/ManualProvider.vue';
import { getFullUrl, STATUS_FETCH } from '~/constants';
import { useToast } from '~/components/toast/use-toast'
import { Job, type DocMeta } from '@blocksuite/store'
import * as http from '~/utils/http'

const { toast } = useToast()
const router = useRouter()

// biome-ignore lint/style/noNonNullAssertion: <explanation>
const { collection, docId, metasSynced, docSyncUrl } = inject<AppState>('appState')!;

const isConnected = ref(false)
const docMetas = ref<DocMeta[]>([])
const days = ref<({
    day: string;
    date: string;
}[])>([])

const updateDocMetaList = () => {
    const metas = collection.meta.docMetas.map((meta: DocMeta) => ({
        tags: Array.from(meta.tags),
        title: meta.title,
        id: meta.id,
        createDate: meta.createDate,
        type: meta.type
    }))
    // console.log(metas)
    docMetas.value = metas.reverse()
}

onMounted(async () => {
    await collection.waitForSynced();
    isConnected.value = true
    updateDocMetaList()
    collection.meta.docMetaUpdated.on(updateDocMetaList)
})

const onAddClicked = async () => {
    const id = collection.idGenerator()
    collection.meta.addDocMeta({
        id,
        title: 'Untitled',
        tags: [],
        createDate: Date.now(),
        type: 'note'
    })
    const doc = collection.getDoc(id)!
    doc.load(() => {
        const parentId = doc.addBlock("affine:page")
        const noteId = doc.addBlock("affine:note", {}, parentId)
        doc.addBlock("affine:paragraph", {}, noteId)
    })
    const job = new Job({ collection })

    const docJson = (await job.docToSnapshot(doc))!

    const { status, message } = await http.post(getFullUrl(`/api/notes`), {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify(docJson),
    })

    if (status === STATUS_FETCH.SUCCESS) {
        toast({
            title: '添加成功',
            description: '添加成功',
            duration: 3000,
        })
        router.push(`/board/note/${id}`)
    } else {
        toast.error({
            title: '添加失败',
            description: message,
            duration: 5000,
        })
    }
}

</script>

<template>
    <div class="h-full flex flex-col overflow-hidden">
        <div class="flex justify-end">
            <button @click="onAddClicked" class="cursor-pointer hover:border-(2px solid [var(--accent-100)]) 
                py-4px px-8px rounded-8px text-[var(--text-100)] bg-[var(--accent-100)] 
                border-(2px solid [var(--bg-200)])">新建文章</button>
        </div>
        <div class="mb-8px">
            <div class="text-0.75rem text-[var(--text-200)]">最近更新</div>
        </div>
        <div class="flex border-b-(1px solid [var(--bg-200)]) pb-8px">
            <div class="text-1rem text-[var(--text-200)]">标题</div>
            <div class="ml-auto"></div>
            <div class="text-1rem text-[var(--text-200)] w-150px">标签</div>
            <div class="text-1rem text-[var(--text-200)] w-150px">更新时间</div>
            <div class="text-1rem text-[var(--text-200)] w-150px">创建时间</div>
        </div>
        <div class="pr-4px overflow-y-auto">
            <template v-for="meta in docMetas" :key="meta.id">
                <NuxtLink :to="`/board/note/${meta.id}`">
                    <NoteTitleCard v-if="meta.type === 'note'" class="" :meta="meta">
                        <template #left="{ meta }">
                            <div v-if="docId === meta.id"
                                class=" absolute -left-2px top-1/2 transform -translate-y-1/2 w-4px h-16px rounded-2px bg-gradient-to-b bg-gradient-from-transparent bg-gradient-to-[var(--accent-100)]">
                            </div>
                        </template>
                    </NoteTitleCard>
                </NuxtLink>
            </template>
        </div>
    </div>
</template>