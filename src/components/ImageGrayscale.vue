<template>
  <div>
    <h2>Image Grayscale Converter</h2>
    <input type="file" accept="image/*" @change="onFileChange" />
    <div v-if="imageUrl" style="margin-top: 1em;">
      <h3>Original Image Preview:</h3>
      <img :src="imageUrl" alt="Original Preview" style="max-width: 300px; max-height: 300px;" 
           ref="originalImage" @load="imageLoaded = true" />
    </div>
    <div v-if="grayscaleUrl" style="margin-top: 1em;">
      <h3>Grayscale Preview:</h3>
      <img :src="grayscaleUrl" alt="Grayscale Preview" style="max-width: 300px; max-height: 300px;" />
      <div style="margin-top: 1em;">
        <a :href="grayscaleUrl" download="grayscale.png" class="download-link">Download Grayscale Image</a>
      </div>
    </div>
    <button v-if="imageUrl && imageLoaded" @click="convertToGrayscale" style="margin-top: 1em;">
      Convert to Grayscale
    </button>
    <canvas ref="canvas" style="display: none;"></canvas>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const imageUrl = ref<string | null>(null)
const grayscaleUrl = ref<string | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)
const originalImage = ref<HTMLImageElement | null>(null)
const imageLoaded = ref(false)
let imageFile: File | null = null

function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    imageFile = target.files[0]
    imageUrl.value = URL.createObjectURL(imageFile)
    grayscaleUrl.value = null
    imageLoaded.value = false
  }
}

function convertToGrayscale() {
  if (!canvas.value || !originalImage.value) return

  const ctx = canvas.value.getContext('2d')
  if (!ctx) return

  // Set canvas size to match original image
  canvas.value.width = originalImage.value.naturalWidth
  canvas.value.height = originalImage.value.naturalHeight

  // Draw image on canvas
  ctx.drawImage(originalImage.value, 0, 0)

  // Get image data
  const imageData = ctx.getImageData(0, 0, canvas.value.width, canvas.value.height)
  const data = imageData.data

  // Convert to grayscale
  for (let i = 0; i < data.length; i += 4) {
    const avg = (data[i] + data[i + 1] + data[i + 2]) / 3
    data[i] = avg     // Red
    data[i + 1] = avg // Green
    data[i + 2] = avg // Blue
    // data[i + 3] is Alpha (unchanged)
  }

  // Put the grayscale image data back on the canvas
  ctx.putImageData(imageData, 0, 0)

  // Convert canvas to URL
  grayscaleUrl.value = canvas.value.toDataURL('image/png')
}
</script>

<style scoped>
.download-link {
  display: inline-block;
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  text-decoration: none;
  border-radius: 4px;
}

.download-link:hover {
  background-color: #45a049;
}

button {
  padding: 8px 16px;
  background-color: #008CBA;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #007B9E;
}
</style> 