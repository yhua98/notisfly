<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import type { AppState } from './editor/ManualProvider.vue';
import { getFullUrl } from '~/constants';
import { useToast } from '~/components/toast/use-toast'
import type { DocMeta } from '@blocksuite/store'

const { toast } = useToast()

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
    docMetas.value = metas
}

watch(() => docMetas.value, (v) => {
    console.log(v)
})

onMounted(async () => {
    await collection.waitForSynced();
    isConnected.value = true
    updateDocMetaList()
    // collection.meta.docMetaAdded.on((v) => {
    //     const meta = collection.meta.getDocMeta(v)
    //     if (!meta) return;
    //     docMetas.value = [...docMetas.value, {
    //         tags: Array.from(meta.tags),
    //         title: meta.title,
    //         id: meta.id,
    //         createDate: meta.createDate,
    //         type: meta.type
    //     }]
    // })
    // collection.meta.docMetaRemoved.on((v) => {
    //     docMetas.value = docMetas.value.filter(meta => meta.id !== v)
    // })
    collection.meta.docMetaUpdated.on(updateDocMetaList)
})



</script>

<template>
    <div class="pb-32px flex flex-col overflow-hidden">
        <div class="flex items-center mt-32px px-8px">
            <div>文章</div>
            <Icon v-if="!metasSynced" class="ml-8px cursor-pointer animate-spin" icon="lucide:loader" />
            <template v-else>
                <Icon v-if="!isConnected" class="ml-8px cursor-pointer text-red" icon="lucide:link-2-off" />
                <Icon v-else class="ml-8px cursor-pointer text-[var(--accent-100)]" icon="lucide:link-2" />
                <Icon class="ml-auto cursor-pointer" icon="lucide:circle-plus" />
            </template>
        </div>
        <div class=" overflow-y-auto my-16px pr-4px">
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