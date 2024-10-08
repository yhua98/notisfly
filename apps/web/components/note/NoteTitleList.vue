<script lang="ts" setup>
import type { AppState } from '../editor/ManualProvider.vue';


const { collection, docId } = inject<AppState>("appState")!;
const metas = ref<({
    id: string;
    title: string;
    tags: string[];
    createDate: number;
})[]>([]);

const updateNoteTitleList =async () => {
    await collection.waitForSynced();
    metas.value = collection.meta.docMetas.map((meta) => ({
        id: meta.id,
        title: meta.title,
        tags: Array.from(meta.tags),
        createDate: meta.createDate,
    }));
}


onMounted(async () => {
    updateNoteTitleList();
    collection.meta.docMetaUpdated.on(updateNoteTitleList);

})

</script>

<template>
    <div class="">
        <div class="h-64px border-b-(1px solid [var(--bg-200)])"></div>
        <div class="p-8px">
            <div class=" absolute -left-8px top-1/2 transform -translate-y-1/2 w-4px h-12px rounded-2px bg-gradient-to-b bg-gradient-from-transparent bg-gradient-to-[var(--accent-100)]"></div>
            <NoteTitleCard v-for="meta in metas" :meta="meta" :key="meta.id" />
        </div>
    </div>
</template>