<script lang="ts" setup>
import '@unocss/reset/tailwind.css'
import Toaster from '~/components/toast/Toaster.vue'
</script>
<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
  <Toaster />
</template>
