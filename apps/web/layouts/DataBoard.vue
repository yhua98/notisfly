<script lang="ts" setup>
import { useColorMode, useMediaQuery } from '@vueuse/core'

const isPreferredDark = useMediaQuery('(prefers-color-scheme: dark)')

watch(() => isPreferredDark.value, (value) => {
    if (value) {
        // dark
        document.querySelector('html')?.setAttribute('data-theme', 'dark')
    } else {
        // light
        document.querySelector('html')?.setAttribute('data-theme', 'light')
    }
}, { immediate: true })

</script>

<template>
    <div class="data-board-layout-wrapper bg-[var(--bg-100)] text-[var(--text-100)]">
        <slot></slot>
    </div>
</template>

<style lang="scss">
@media (prefers-color-scheme: dark) {
    :root {
        --primary-100: #0085ff;
        --primary-200: #69b4ff;
        --primary-300: #e0ffff;
        --accent-100: #006fff;
        --accent-200: #e1ffff;
        --text-100: #ffffff;
        --text-200: #9e9e9e;
        --text-300: lighten(#9e9e9e, 30%);
        --bg-50: #141414;
        --bg-100: #1e1e1e;
        --bg-150: #1e2129;
        --bg-200: #2d2d2d;
        --bg-300: #454545;
    }
}

@media (prefers-color-scheme: light) {
    :root {
        --primary-100: #d4eaf7;
        --primary-200: #b6ccd8;
        --primary-300: #3b3c3d;
        --accent-100: #71c4ef;
        --accent-200: #00668c;
        --text-100: #1d1c1c;
        --text-200: #313d44;
        --text-300: #afafaf;
        --bg-50: #ffffff;
        --bg-100: #fffefb;
        --bg-150: #e5eaea;
        --bg-200: #f5f4f1;
        --bg-300: #cccbc8;
    }
}

html {
    font-family: system-ui, sans-serif;
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

*::-webkit-scrollbar {
    height: 5px;
    width: 4px;
}

*::-webkit-scrollbar-thumb {
    background-color: var(--bg-200);
    border-radius: 2px;
}
</style>