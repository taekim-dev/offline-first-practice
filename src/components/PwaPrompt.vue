<template>
  <div v-if="showInstallPrompt || updateAvailable" class="pwa-prompt">
    <div v-if="showInstallPrompt" class="prompt-content">
      <p>Install app for offline use</p>
      <button @click="installPwa" class="install-button">Install</button>
      <button @click="dismissInstall" class="dismiss-button">Not now</button>
    </div>
    <div v-if="updateAvailable" class="prompt-content">
      <p>New version available</p>
      <button @click="updatePwa" class="update-button">Update</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const showInstallPrompt = ref(false)
const updateAvailable = ref(false)
let deferredPrompt: any = null

// Listen for 'beforeinstallprompt' event
onMounted(() => {
  window.addEventListener('beforeinstallprompt', handleInstallPrompt)
})

onUnmounted(() => {
  window.removeEventListener('beforeinstallprompt', handleInstallPrompt)
})

function handleInstallPrompt(e: Event) {
  e.preventDefault()
  deferredPrompt = e
  showInstallPrompt.value = true
}

async function installPwa() {
  if (!deferredPrompt) return
  
  deferredPrompt.prompt()
  const { outcome } = await deferredPrompt.userChoice
  
  if (outcome === 'accepted') {
    console.log('PWA installed successfully')
  }
  
  deferredPrompt = null
  showInstallPrompt.value = false
}

function dismissInstall() {
  showInstallPrompt.value = false
}

function updatePwa() {
  window.location.reload()
  updateAvailable.value = false
}

// Expose method to be called from parent when update is available
defineExpose({
  showUpdatePrompt: () => {
    updateAvailable.value = true
  }
})
</script>

<style scoped>
.pwa-prompt {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: white;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}

.prompt-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.prompt-content p {
  margin: 0;
}

.install-button, .update-button {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.dismiss-button {
  background-color: #757575;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  opacity: 0.9;
}
</style> 