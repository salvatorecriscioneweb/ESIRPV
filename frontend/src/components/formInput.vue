<template>
  <input
    :name="name"
    :type="typeToUse"
    :placeholder="placeholder"
    class="text-base px-4 py-2 rounded-xl"
    :value="modelValue"
    @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>

<script setup lang="ts">
defineEmits(['update:modelValue'])
const props = defineProps({
  name: {
    type: String,
    default: '',
  },
  type: {
    type: String,
    default: 'text',
  },
  placeholder: {
    type: String,
    default: '',
  },
  secure: {
    type: Boolean,
    default: false,
  },
  modelValue: {
    type: [String, Number],
    default: '',
  },
})

const typeToUse = props.secure ? 'password' : props.type
</script>
