import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import { VueQueryPlugin } from '@tanstack/vue-query'

import FormInput from './components/formInput.vue'
import FormButton from './components/formButton.vue'

import './index.css'
const pinia = createPinia()
const app = createApp(App).use(router).use(pinia).use(VueQueryPlugin)

app.component('FormInput', FormInput)
app.component('FormButton', FormButton)

app.mount('#app')
