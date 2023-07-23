<template>
  <div
    class="bg-bg min-w-[500px] max-h-[380px] rounded-[48px] px-[36px] py-[20px] shadow-lg overflow-y-auto overflow-x-hidden mt-10 mx-auto"
  >
    <slot />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ContainerApp',
})
</script>
