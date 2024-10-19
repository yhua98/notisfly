<script lang="ts" setup>

const { meta } = defineProps<{
    meta: {
        id: string;
        title: string;
        tags: string[];
        createDate: number;
    }
}>()

</script>


<template>
    <div
        class="relative flex hover:bg-[var(--bg-200)] border-b-(1px solid [var(--bg-200)]) mt-2px first:mt-0 pl-8px pr-4px py-8px cursor-pointer">
        <slot name="left" :meta="meta"></slot>
        <div class="grow-1">
            <div class="relative">
                <span class="line-clamp-2 relative 
            before:(content-[''] absolute right-0 left-0 top-0 bottom-0 
            bg-gradient-to-r bg-gradient-from-transparent bg-gradient-to-[var(--bg-100)])">
                    {{ meta.title }}
                </span>
                <span class="absolute right-0 top-0 text-0.75rem leading-2em text-[var(--text-200)]">{{ new
                    Date(meta.createDate).toDateString() }}</span>
            </div>
            <div class="mt-0px text-0.75rem text-[var(--text-200)]">
                <span>{{ meta.tags.join(', ') }}</span>
            </div>
        </div>
    </div>
</template>