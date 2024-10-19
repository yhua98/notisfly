<script lang="ts" setup>
import { Icon } from '@iconify/vue'
definePageMeta({
    layout: 'data-board'
})
</script>

<template>
    <EditorManualProvider>
        <div class="h-100vh flex">
            <SideBoard class="z-100" />
            <!-- <div class="w-400px h-full border-r-(1px solid [var(--bg-200)])">
                <NoteTitleList />
            </div> -->
            <div class="grow-1">
                <div class="h-64px flex items-center px-16px 
                border-b-(1px solid [var(--bg-200)])">
                    <ThemeSwitch class="ml-auto" />
                </div>
                <div class="flex databoard-content-wrapper h-[calc(100%-64px)] overflow-auto">
                    <EditorManualContainer class="w-768px h-full mx-auto" />
                    <div class="relative px-8px w-300px border-l-(1px solid [var(--bg-200)])">
                        <NoteList />
                        <div class="absolute top-50% left-0 -translate-y-50% -translate-x-50% text-[var(--text-200)]">
                            <Icon size="4" icon="lucide:grip-vertical" class="cursor-col-resize" />
                        </div>
                    </div>
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