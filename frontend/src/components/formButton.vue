<template>
  <button
    class="bg-white text-base px-6 py-4 rounded-xl"
    @click="(e : MouseEvent) => { click(e); return e;}"
  >
    <slot />
  </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'FormButton',
  props: {
    click: {
      type: Function,
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      default: () => {},
    },
  },
})
</script>
