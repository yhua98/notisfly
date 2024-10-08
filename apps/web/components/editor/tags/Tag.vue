<script lang="ts" setup>
import {Icon} from '@iconify/vue'
const { tag } = defineProps({
    tag: {
        type: String,
        required: true,
    },
});

const emit = defineEmits({
    remove: (tag) => true,
})

const remove = () => {
    emit('remove', tag)
}

</script>

<template>
    <button class="ml-8px inline-flex items-center first:ml-0 border-(2px solid [var(--accent-100)]) rounded-8px pl-8px pr-4px">
        {{ tag }}
        <Icon @click="remove" class="ml-4px text-[var(--accent-100)]" icon="lucide:x" />
    </button>
</template>