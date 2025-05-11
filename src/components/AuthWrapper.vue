<template>
  <div>
    <div v-if="!isAuthenticated" class="auth-screen">
      <div class="auth-container">
        <h2>Authentication Required</h2>
        <p v-if="!keyFromUrl">Please enter your access key to continue:</p>
        <div v-if="!keyFromUrl" class="input-group">
          <input 
            type="text" 
            v-model="accessKey" 
            placeholder="Enter your access key"
            @keyup.enter="validateKey"
          />
          <button @click="validateKey" :disabled="!accessKey">
            Validate Key
          </button>
        </div>
        <p v-if="error" class="error">{{ error }}</p>
        <div v-if="!keyFromUrl" class="purchase-section">
          <p>Don't have an access key?</p>
          <button @click="showPurchaseModal = true" class="purchase-button">
            Purchase Access
          </button>
        </div>
      </div>
    </div>
    <slot v-else></slot>

    <PurchaseAccess 
      v-if="showPurchaseModal"
      :show="showPurchaseModal"
      @close="showPurchaseModal = false"
      @payment-success="handlePaymentSuccess"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useRoute } from 'vue-router'
import PurchaseAccess from './PurchaseAccess.vue'

const authStore = useAuthStore()
const route = useRoute()

const isAuthenticated = ref(false)
const accessKey = ref('')
const error = ref('')
const keyFromUrl = ref(false)
const showPurchaseModal = ref(false)

onMounted(async () => {
  // Check for key in URL
  const urlKey = route.query.authkey as string
  if (urlKey) {
    keyFromUrl.value = true
    if (await validateKeyFromUrl(urlKey)) {
      isAuthenticated.value = true
    } else {
      error.value = 'Invalid or expired key'
    }
    return
  }

  // Check for stored key
  if (authStore.checkStoredKey()) {
    isAuthenticated.value = true
  }
})

async function validateKey() {
  if (!accessKey.value) {
    error.value = 'Please enter an access key'
    return
  }

  if (authStore.validateKey(accessKey.value)) {
    isAuthenticated.value = true
    error.value = ''
  } else {
    error.value = 'Invalid or expired key'
  }
}

async function validateKeyFromUrl(key: string): Promise<boolean> {
  return authStore.validateKey(key)
}

function handlePaymentSuccess(key: string) {
  accessKey.value = key
  validateKey()
}
</script>

<style scoped>
.auth-screen {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.auth-container {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  width: 90%;
  max-width: 400px;
}

.input-group {
  display: flex;
  gap: 8px;
  margin: 1rem 0;
}

input {
  flex: 1;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.error {
  color: #f44336;
  margin: 8px 0;
}

.purchase-section {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid #eee;
}

.purchase-button {
  background: #2196F3;
  width: 100%;
  margin-top: 8px;
}

.purchase-button:hover {
  background: #1976D2;
}
</style> 