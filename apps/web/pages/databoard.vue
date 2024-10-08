<script lang="ts" setup>
definePageMeta({
    layout: 'data-board'
})
</script>

<template>
    <EditorManualProvider>
        <div class="h-100vh flex">
            <SideBoard class="z-100" />
            <!-- <div class="w-400px border-r-(1px solid [var(--bg-200)])">
                <NoteTitleList />
            </div> -->
            <div class="grow-1">
                <div class="h-64px flex items-center px-16px 
                border-b-(1px solid [var(--bg-200)])">
                    <ThemeSwitch class="ml-auto" />
                </div>
                <div class="flex databoard-content-wrapper p-8px h-[calc(100%-64px)] overflow-auto">
                    <EditorManualContainer class="w-512px mx-auto" />
                </div>
            </div>
        </div>
    </EditorManualProvider>
</template>

<style lang="scss" scoped>
.databoard-content-wrapper {
    position: relative;

    &::-webkit-scrollbar {
        width: 0;
    }
}
</style>