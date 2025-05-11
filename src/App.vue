<script setup lang="ts">
import { ref } from 'vue'
import ImageGrayscale from './components/ImageGrayscale.vue'
import AuthWrapper from './components/AuthWrapper.vue'
import PwaPrompt from './components/PwaPrompt.vue'

const pwaPrompt = ref()

</script>

<template>
  <AuthWrapper>
    <ImageGrayscale />
    <PwaPrompt ref="pwaPrompt" />
  </AuthWrapper>
</template>

<style>
body {
  margin: 0;
  padding: 20px;
  font-family: Arial, sans-serif;
}
</style>
