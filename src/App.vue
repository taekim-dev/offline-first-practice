<script setup lang="ts">
import { ref } from 'vue'
import ImageGrayscale from './components/ImageGrayscale.vue'
import PwaPrompt from './components/PwaPrompt.vue'

const pwaPrompt = ref()

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
