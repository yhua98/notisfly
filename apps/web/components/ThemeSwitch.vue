<script lang="ts" setup>
import { useColorMode } from '@vueuse/core'
import { Icon } from '@iconify/vue'
import { ref, computed } from 'vue'

const mode = useColorMode()

const onSwitchTheme = () => {
    mode.value = mode.value === 'light' ? 'dark' : 'light'
}

const darkModeSwitch = document.createElement('style')
darkModeSwitch.textContent = `
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
`

const lightModeSwitch = document.createElement('style')
lightModeSwitch.textContent = `
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
`

watch(() => mode.value, (value) => {
    if (value === 'dark') {
        if (document.head.contains(lightModeSwitch)) {
            document.head.removeChild(lightModeSwitch)
        }
        document.head.appendChild(darkModeSwitch)
        document.querySelector('html')?.setAttribute('data-theme', 'dark')
    } else {
        if (document.head.contains(darkModeSwitch)) {
            document.head.removeChild(darkModeSwitch)
        }
        document.head.appendChild(lightModeSwitch)
        document.querySelector('html')?.setAttribute('data-theme', 'light')
    }
}, { immediate: true })

</script>

<template>
    <div class="theme-switch flex-inline rounded-1/2 border-(2px solid transparent)">
        <button @click="onSwitchTheme" class="theme-switch-btn flex text-[var(--bg-300)]">
            <Icon size="6" :icon="mode === 'light' ? 'lucide:moon-star' : 'lucide:cloud-sun'" />
        </button>
    </div>
</template>

<style>
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
</style>