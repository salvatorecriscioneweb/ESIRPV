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
import { login as loginAPI } from '../../api/index'
import { ILoginResponse } from '@/interfaces'
import { saveToLocalStorage } from '@/utils/localStorage'
const username = ref('')
const password = ref('')

async function login() {
  dbg(`Login func start with ${username.value} ${password.value}`)

  if (!username.value || !password.value) {
    dbg('Login missing validation')
    return
  }
  const { data, status } = (await loginAPI({
    username: username.value,
    password: password.value,
  })) as { data: ILoginResponse; status: number }

  console.log(data)

  if (data.token && status == 200) {
    saveToLocalStorage('user', data.token)
    router.replace('/orders')
  }
  dbg('Login func ends')
}
</script>
