<template>
  <div>
    <div class="status-bar" :class="{ 'offline': !isOnline }">
      {{ isOnline ? 'Online' : 'Offline' }} Mode
    </div>
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
    <button 
      v-if="imageUrl && imageLoaded" 
      @click="convertToGrayscale" 
      :disabled="processing"
      style="margin-top: 1em;"
    >
      {{ processing ? 'Processing...' : 'Convert to Grayscale' }}
    </button>
    <canvas ref="canvas" style="display: none;"></canvas>
    <canvas ref="outputCanvas" style="display: none;"></canvas>

    <!-- Recent Conversions -->
    <div v-if="recentImages.length > 0" class="recent-images">
      <h3>Recent Conversions</h3>
      <div class="recent-grid">
        <div v-for="(img, index) in recentImages" :key="index" class="recent-item">
          <img :src="img.grayscale" alt="Recent conversion" style="max-width: 100px;" />
          <div class="recent-actions">
            <a :href="img.grayscale" download="grayscale.png" class="small-button">Download</a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { initWasm, processImageWithWasm } from '../utils/wasm-processor'

const imageUrl = ref<string | null>(null)
const grayscaleUrl = ref<string | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)
const outputCanvas = ref<HTMLCanvasElement | null>(null)
const originalImage = ref<HTMLImageElement | null>(null)
const imageLoaded = ref(false)
const isOnline = ref(navigator.onLine)
const recentImages = ref<{ original: string; grayscale: string }[]>([])
const processing = ref(false)
let imageFile: File | null = null

// Initialize WebAssembly module
onMounted(async () => {
  try {
    await initWasm()
  } catch (error) {
    console.error('Failed to initialize WASM:', error)
  }

  // Load recent images from localStorage
  const saved = localStorage.getItem('recentImages')
  if (saved) {
    recentImages.value = JSON.parse(saved)
  }

  window.addEventListener('online', updateOnlineStatus)
  window.addEventListener('offline', updateOnlineStatus)
})

function updateOnlineStatus() {
  isOnline.value = navigator.onLine
}

function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    imageFile = target.files[0]
    imageUrl.value = URL.createObjectURL(imageFile)
    grayscaleUrl.value = null
    imageLoaded.value = false
  }
}

async function convertToGrayscale() {
  if (!canvas.value || !outputCanvas.value || !originalImage.value) return
  
  processing.value = true
  
  try {
    const ctx = canvas.value.getContext('2d')
    const outputCtx = outputCanvas.value.getContext('2d')
    if (!ctx || !outputCtx) return

    // Set canvas sizes
    canvas.value.width = originalImage.value.naturalWidth
    canvas.value.height = originalImage.value.naturalHeight
    outputCanvas.value.width = originalImage.value.naturalWidth
    outputCanvas.value.height = originalImage.value.naturalHeight

    // Draw original image to canvas
    ctx.drawImage(originalImage.value, 0, 0)

    // Get image data
    const imageData = ctx.getImageData(0, 0, canvas.value.width, canvas.value.height)

    // Process image using WebAssembly
    const processedImageData = await processImageWithWasm(imageData)

    // Put processed image data on output canvas
    outputCtx.putImageData(processedImageData, 0, 0)

    // Convert to URL
    grayscaleUrl.value = outputCanvas.value.toDataURL('image/png')

    // Save to recent images
    if (imageUrl.value && grayscaleUrl.value) {
      recentImages.value.unshift({
        original: imageUrl.value,
        grayscale: grayscaleUrl.value
      })
      // Keep only last 4 images
      if (recentImages.value.length > 4) {
        recentImages.value.pop()
      }
      // Save to localStorage
      localStorage.setItem('recentImages', JSON.stringify(recentImages.value))
    }
  } catch (error) {
    console.error('Error processing image:', error)
  } finally {
    processing.value = false
  }
}
</script>

<style scoped>
.status-bar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  padding: 8px;
  background-color: #4CAF50;
  color: white;
  text-align: center;
  z-index: 1000;
}

.status-bar.offline {
  background-color: #f44336;
}

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

.recent-images {
  margin-top: 2em;
  padding: 1em;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.recent-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 1em;
  margin-top: 1em;
}

.recent-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5em;
}

.small-button {
  padding: 4px 8px;
  background-color: #4CAF50;
  color: white;
  text-decoration: none;
  border-radius: 4px;
  font-size: 0.8em;
}

.small-button:hover {
  background-color: #45a049;
}
</style> 