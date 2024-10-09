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

const updateDays = (date: Date) => {
    const today = date
    today.setDate(today.getDate() - 3)
    const _days = []
    for (let i = 0; i < 5; i++) {
        today.setDate(today.getDate() + 1)
        _days.push({
            day: `${today.getDate().toString().padStart(2, '0')}`,
            date: `${today.getFullYear()}${(today.getMonth() + 1).toString().padStart(2, '0')}${today.getDate().toString().padStart(2, '0')}`
        })
    }
    days.value = _days
}

const updateDocMetaList = () => {
    docMetas.value = collection.meta.docMetas.map((meta: DocMeta) => ({
        tags: Array.from(meta.tags),
        title: meta.title,
        id: meta.id,
        createDate: meta.createDate,
        type: meta.type
    }))
}

onMounted(async () => {
    updateDays(new Date())
    await collection.waitForSynced();
    isConnected.value = true
    updateDocMetaList()
    collection.meta.docMetaAdded.on((v) => {
        const meta = collection.meta.getDocMeta(v)
        if (!meta) return;
        docMetas.value = [...docMetas.value, {
            tags: Array.from(meta.tags),
            title: meta.title,
            id: meta.id,
            createDate: meta.createDate,
            type: meta.type
        }]
    })
    collection.meta.docMetaRemoved.on((v) => {
        docMetas.value = docMetas.value.filter(meta => meta.id !== v)
    })
    collection.meta.docMetaUpdated.on(updateDocMetaList)
})

const onAddClicked = async () => {

}

const onTitleClicked = async (meta: DocMeta) => {
    docId.value = meta.id
    const doc = collection.getDoc(docId.value)
    if (!doc) {
        console.error('doc not found', docId.value)
        return
    }
}

const onDnidClicked = (event: MouseEvent) => {
    // const dnid = (event.target as HTMLDivElement).getAttribute('data-dnid')

    // // update days
    // const index = days.value.findIndex(day => day.date === dnid)
    // if (index === -1) return
    // // move the clicked day to the middle
    // // biome-ignore lint/style/useTemplate: <explanation>
    // const formatDay = `${days.value[index].date.slice(0, 4)}` + '-' +
    //     `${days.value[index].date.slice(4, 6)}` + '-' +
    //     `${days.value[index].date.slice(6, 8)}`
    // const date = new Date(formatDay)
    // updateDays(date)
}

</script>

<template>
    <div class="flex flex-col">
        <div class="flex items-center mt-32px">
            <div>文章</div>
            <Icon v-if="!metasSynced" class="ml-8px cursor-pointer animate-spin" icon="lucide:loader" />
            <template v-else>
                <Icon v-if="!isConnected" class="ml-8px cursor-pointer text-red" icon="lucide:link-2-off" />
                <Icon v-else class="ml-8px cursor-pointer text-[var(--accent-100)]" icon="lucide:link-2" />
                <!-- <div class="flex mx-16px grow-1 items-center">
                    <Icon class="cursor-pointer text-[var(--text-200)] hover:text-[var(--text-100)]" size="4"
                        icon="lucide:chevrons-left" />
                    <div @click="onDnidClicked" class="grow-1 flex justify-between items-center mx-4px">
                        <div v-for="(day, index) in days" :key="day.date" :data-dnid="day.date"
                            :style="{ '--scale1': 1.2 - 0.6 * Math.abs(index - 2) / 2 }"
                            :class="index == -1 ? `text-[var(--accent-100)] scale-[var(--scale)]` : 'scale-[var(--scale)] text-[var(--text-200)]'"
                            class="cursor-pointer">
                            {{ day.day }}
                        </div>
                    </div>
                    <Icon class="cursor-pointer text-[var(--text-200)] hover:text-[var(--text-100)]" size="4"
                        icon="lucide:chevrons-right" />
                </div> -->
                <Icon @click="onAddClicked" class="ml-auto cursor-pointer" icon="lucide:circle-plus" />
            </template>
        </div>
        <div class="grow-1 overflow-y-auto">
            <!-- <div @click="onTitleClicked(meta)" class="text-[var(--text-200)] relative my-4px rounded-8px px-4px py-2px border-(1px [var(--bg-200)]) cursor-pointer ml-8px" v-for="meta in docMetas" :key="meta.id">
            {{ meta.title || `Note(#${meta.id})` }}
            <div v-if="docId === meta.id" class=" absolute -left-8px top-1/2 transform -translate-y-1/2 w-4px h-12px rounded-2px bg-gradient-to-b bg-gradient-from-transparent bg-gradient-to-[var(--accent-100)]"></div>
        </div> -->
            <template v-for="meta in docMetas" :key="meta.id">
                <NoteTitleCard v-if="meta.type === 'note'" class="first:mt-16px" @click="onTitleClicked(meta)"
                    :meta="meta">
                    <template #left="{ meta }">
                        <div v-if="docId === meta.id"
                            class=" absolute -left-2px top-1/2 transform -translate-y-1/2 w-4px h-16px rounded-2px bg-gradient-to-b bg-gradient-from-transparent bg-gradient-to-[var(--accent-100)]">
                        </div>
                    </template>
                </NoteTitleCard>
            </template>

        </div>
    </div>
</template>