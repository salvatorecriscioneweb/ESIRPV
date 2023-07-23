<template>
  <button :class="'' + (active ? 'text-black' : 'text-white')">
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps({
  active: {
    type: Boolean,
    default: false,
  },
})
</script>
