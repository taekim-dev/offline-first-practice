<template>
  <div class="purchase-modal" v-if="show">
    <div class="modal-content">
      <h2>Purchase Access</h2>
      <div class="price-info">
        <p>One-time payment for lifetime access</p>
        <h3>$9.99</h3>
      </div>
      
      <div id="stripe-payment-element"></div>
      
      <button 
        @click="handlePayment" 
        :disabled="processing"
        class="pay-button"
      >
        {{ processing ? 'Processing...' : 'Pay Now' }}
      </button>
      
      <div v-if="error" class="error">
        {{ error }}
      </div>
      
      <button @click="$emit('close')" class="close-button">
        ✕
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, defineEmits } from 'vue'
import { loadStripe } from '@stripe/stripe-js'
import { useAuthStore } from '../stores/auth'

const props = defineProps<{
  show: boolean
}>()

const emit = defineEmits(['close', 'payment-success'])

const processing = ref(false)
const error = ref('')
const stripe = ref<any>(null)
const elements = ref<any>(null)
const authStore = useAuthStore()

const STRIPE_PUBLIC_KEY = import.meta.env.VITE_STRIPE_PUBLIC_KEY || 'your-stripe-public-key'

onMounted(async () => {
  // Initialize Stripe
  stripe.value = await loadStripe(STRIPE_PUBLIC_KEY)
  
  // For demo purposes, we'll simulate the payment
  // In production, you would create a payment intent on your server
})

async function handlePayment() {
  processing.value = true
  error.value = ''
  
  try {
    // Simulate payment success
    // In production, you would handle real Stripe payment here
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    // Generate a new access key
    const userId = 'user_' + Math.random().toString(36).substr(2, 9)
    const accessKey = authStore.generateKey(userId)
    
    // Simulate sending email with access key
    console.log('Access Key generated:', accessKey)
    console.log('Email would be sent with link:', `${window.location.origin}/?authkey=${accessKey}`)
    
    emit('payment-success', accessKey)
    emit('close')
  } catch (err) {
    error.value = 'Payment failed. Please try again.'
  } finally {
    processing.value = false
  }
}
</script>

<style scoped>
.purchase-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1100;
}

.modal-content {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  width: 90%;
  max-width: 400px;
  position: relative;
}

.price-info {
  text-align: center;
  margin: 1.5rem 0;
}

.price-info h3 {
  font-size: 2rem;
  margin: 0.5rem 0;
  color: #2196F3;
}

.pay-button {
  width: 100%;
  padding: 12px;
  background: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1.1rem;
  cursor: pointer;
  margin-top: 1rem;
}

.pay-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.close-button {
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #666;
}

.error {
  color: #f44336;
  margin-top: 1rem;
  text-align: center;
}
</style> 