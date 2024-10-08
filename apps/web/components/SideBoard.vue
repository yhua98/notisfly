<script lang="ts" setup>
import { ref } from 'vue'
import { Icon } from '@iconify/vue'
import DateTime from './DateTime.vue'
const isFloat = ref(false)

const update = () => {
    console.log(isFloat.value)
    isFloat.value = !isFloat.value
}
</script>

<template>
    <div class="group relative">
        <div :class="'h-100% transition-width ease-out transition-5000 overflow-hidden border-r-(1px [var(--bg-200)] solid) bg-[var(--bg-100)] ' + (isFloat
            ? ' fixed transform w-0 group-hover:(w-300px shadow-2xl) h-100vh' : 'relative w-300px')">
            <div class=" w-300px p-8px">
                <div class="h-64px flex items-center">
                    <img class=" hero animate-rotate-in-down-left border-2px hover:border-[var(--accent-100)] w-48px h-48px rounded-1/2"
                        src="https://picsum.photos/200/200" />
                    <span class="ml-8px text-20px">Notisfly</span>
                </div>
                <div class="mt-16px">
                    <DateTime />
                </div>
                <div class="mt-16px">
                    <NoteList />
                </div>
            </div>
        </div>
        <div @click="isFloat = !isFloat" :class="'transform -translate-y-50% hover:color-blue color-[var(--text-200)] cursor-pointer bg-[var(--bg-100)] ' + (isFloat
            ? 'fixed -left-2px top-32px border-(1px solid [var(--bg-200)]) shadow-xl rounded-r-1/2 pr-4px'
            : 'absolute top-32px right-8px')">
            <Icon v-if="isFloat" size="6" icon="lucide:chevrons-right" />
            <Icon class="text-[var(--bg-200)] hover:color-[var(--accent-100)]" v-else="isFloat" size="6"
                icon="lucide:panel-left-close" />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.hero {
    animation: rotate 2s infinite;
    transform-origin: center;

    @keyframes rotate {
        0% {
            transform: rotate(0deg);
        }

        100% {
            transform: rotate(360deg);
        }
    }
}
</style>