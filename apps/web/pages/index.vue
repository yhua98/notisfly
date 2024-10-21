<script lang="ts" setup>

definePageMeta({
    layout: 'board'
})

</script>

<template>
    <div class="h-100vh flex justify-center items-center">
        <div class="flex gap-32px">
            <div class="p-32px cursor-pointer text-2xl border-(1px solid [var(--bg-200)])">
                <RouterLink to="/databoard">Go to Data Board</RouterLink>
            </div>
            <div class="p-32px cursor-pointer text-2xl border-(1px solid [var(--bg-200)])">
                <RouterLink to="/editor">Go to Editor</RouterLink>
            </div>
        </div>
    </div>
</template>