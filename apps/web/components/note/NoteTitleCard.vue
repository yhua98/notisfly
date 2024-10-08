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
    <div class="relative flex border-(1px solid [var(--bg-200)]) mt-8px first:mt-0 p-8px rounded-8px cursor-pointer">
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
            <div class="mt-8px text-0.75rem text-[var(--text-200)]">
                <span>{{ meta.tags.join(', ') }}</span>
            </div>
        </div>
    </div>
</template>