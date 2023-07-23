<template>
  <div class="min-w-[400px]">
    <h1 class="text-h2 font-bold text-black">Login</h1>
    <div class="flex flex-col gap-8 mt-10">
      <FormInput v-model="username" placeholder="Username" />
      <FormInput v-model="password" placeholder="Password" secure />
      <div class="flex justify-center">
        <FormButton :click="login"> Login </FormButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import router from '../../router'
import dbg from '../../utils/debug'

const username = ref('')
const password = ref('')

function login() {
  dbg(`Login func start with ${username.value} ${password.value}`)

  if (!username.value || !password.value) {
    dbg('Login missing validation')
    return
  }

  // TODO: Make request + md5 password
  // TODO: Save to local storage

  router.replace('/orders')
  dbg('Login func ends')
}
</script>
