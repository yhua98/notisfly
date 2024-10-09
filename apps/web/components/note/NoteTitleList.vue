<script lang="ts" setup>
import type { AppState } from '../editor/ManualProvider.vue';


// biome-ignore lint/style/noNonNullAssertion: <explanation>
const { collection } = inject<AppState>("appState")!;
const metas = ref<({
    id: string;
    title: string;
    tags: string[];
    createDate: number;
    type: string;
})[]>([]);

const updateNoteTitleList = async () => {
    await collection.waitForSynced();
    metas.value = collection.meta.docMetas.map((meta) => ({
        id: meta.id,
        title: meta.title,
        tags: Array.from(meta.tags),
        createDate: meta.createDate,
        type: meta.type,
    }));
}


onMounted(async () => {
    updateNoteTitleList();
    collection.meta.docMetaUpdated.on(updateNoteTitleList);

})

</script>

<template>
    <div class="h-full title-list">
        <div class="h-64px border-b-(1px solid [var(--bg-200)])">
            <div class="h-100% flex items-center px-16px">
                <span class="text-20px">Notes</span>
            </div>
        </div>
        <div class="p-8px h-[calc(100%-64px)] overflow-y-auto">
            <template v-for="meta in metas" :key="meta.id">
                <NoteTitleCard v-if="meta.type === 'note'" :meta="meta" />
            </template>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.title-list {
    *::-webkit-scrollbar {
        height: 5px;
        width: 3px;
    }

    *::-webkit-scrollbar-thumb {
        background-color: var(--bg-200);
        border-radius: 0;
    }
}
</style>