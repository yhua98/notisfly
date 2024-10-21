<script lang="ts" setup>
import { useToast } from './use-toast'
import { Icon } from '@iconify/vue'

const { toasts } = useToast()

</script>

<template>
    <div class="fixed z-999 top-72px right-8px flex flex-col text-[var(--text-100)]">
        <TransitionGroup tag="div" name="list">
            <div class="toast relative w-200px bg-[var(--bg-200)] mt-8px first:mt-0 p-8px rounded-8px"
                v-for="toast in toasts" :key="toast.id">
                <div class="flex items-center">
                    <Icon v-if="toast.type === 'info'" class="mr-4px text-[var(--accent-100)]" icon="lucide:info" />
                    <Icon v-else-if="toast.type === 'error'" class="text-red mr-4px text-[var(--accent-100)]"
                        icon="lucide:circle-x" />
                    {{ toast.title }}
                </div>
                <div class="text-0.75em ml-2px text-[var(--text-200)]">{{ toast.description }}</div>
                <Icon @click="toast.onClose?.()"
                    class="absolute top-8px right-8px text-[var(--text-200)] hover:text-[var(--text-100)]"
                    icon="lucide:x" />
            </div>
        </TransitionGroup>
    </div>
</template>

<style lang="scss" scoped>
.toast {
    box-shadow: 0px 0px 8px 8px rgba(0, 0, 0, 0.1);
}

.list-enter-active,
.list-leave-active {
    transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(150px);
}
</style>