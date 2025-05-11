import { defineStore } from 'pinia'
import CryptoJS from 'crypto-js'

const SECRET_KEY = import.meta.env.VITE_HMAC_SECRET || 'your-dev-secret-key'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    isAuthenticated: false,
    accessKey: null as string | null,
  }),

  actions: {
    validateKey(key: string): boolean {
      try {
        // Key format: userId.timestamp.hmac
        const [userId, timestamp, hmac] = key.split('.')
        
        if (!userId || !timestamp || !hmac) {
          return false
        }

        // Check if the key is not expired (e.g., valid for 1 year)
        const keyTimestamp = parseInt(timestamp)
        const oneYearMs = 365 * 24 * 60 * 60 * 1000
        if (Date.now() - keyTimestamp > oneYearMs) {
          return false
        }

        // Verify HMAC
        const message = `${userId}.${timestamp}`
        const expectedHmac = CryptoJS.HmacSHA256(message, SECRET_KEY).toString()

        if (hmac === expectedHmac) {
          this.isAuthenticated = true
          this.accessKey = key
          localStorage.setItem('accessKey', key)
          return true
        }

        return false
      } catch (error) {
        console.error('Key validation error:', error)
        return false
      }
    },

    checkStoredKey(): boolean {
      const storedKey = localStorage.getItem('accessKey')
      if (storedKey) {
        return this.validateKey(storedKey)
      }
      return false
    },

    logout() {
      this.isAuthenticated = false
      this.accessKey = null
      localStorage.removeItem('accessKey')
    },

    // Helper method to generate a key (normally done on server)
    generateKey(userId: string): string {
      const timestamp = Date.now().toString()
      const message = `${userId}.${timestamp}`
      const hmac = CryptoJS.HmacSHA256(message, SECRET_KEY).toString()
      return `${userId}.${timestamp}.${hmac}`
    }
  }
}) 