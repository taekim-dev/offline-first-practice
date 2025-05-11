<script setup lang="ts">
import { ref } from 'vue'
import ImageGrayscale from './components/ImageGrayscale.vue'
import PwaPrompt from './components/PwaPrompt.vue'
import { registerSW } from 'virtual:pwa-register'

const pwaPrompt = ref()

const updateSW = registerSW({
  onNeedRefresh() {
    pwaPrompt.value?.showUpdatePrompt()
  },
  onOfflineReady() {
    console.log('App ready to work offline')
  },
})
</script>

<template>
  <div>
    <ImageGrayscale />
    <PwaPrompt ref="pwaPrompt" />
  </div>
</template>

<style>
body {
  margin: 0;
  padding: 20px;
  font-family: Arial, sans-serif;
}
</style>
