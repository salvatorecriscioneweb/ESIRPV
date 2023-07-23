<script lang="ts" setup>
defineProps({
  selected: {
    type: Boolean,
    default: false,
  },
  orderId: {
    type: String,
    default: '',
  },
  image: {
    type: String,
    default: null,
  },
  client: {
    type: Object,
    default: () => ({
      name: '',
      surname: '',
      clientId: '',
    }),
  },
  index: {
    type: Number,
    default: -1,
  },
  date: {
    type: String,
    default: null,
  },
  products: {
    type: Number,
    default: 0,
  },
})
</script>
<script lang="ts">
export default {
  name: 'OrdersTile',
}
</script>
<template>
  <div
    :class="[selected ? 'bg-sky-900 bg-opacity-75 text-white ' : 'bg-white ']"
    class="relative flex cursor-pointer rounded-[1rem] min-h-[90px] px-5 py-4 shadow-md focus:outline-none select-none"
  >
    <div class="flex grow items-center justify-between">
      <div class="flex w-full items-center pl-[90px]">
        <img :src="image" class="top-0 left-0 absolute object-cover h-full" />
        <div
          class="w-full grid grid-cols-3 gap-x-4 items-start text-base justify-start {{ selected ? 'text-white' : 'text-gray-900' }}"
        >
          <div class="">#{{ orderId }}</div>
          <div class="">
            {{ client.name }}<br />
            {{ client.surname }}
          </div>
          <div class="">
            {{ date }}
          </div>
          <div class="text-small font-semibold tracking-wider text-[#AAAAAA]">
            {{ products }} {{ products > 1 ? 'products' : 'product' }}
          </div>
          <div class="text-small font-semibold tracking-wider text-[#AAAAAA]">{{ client.id }}</div>
        </div>
      </div>
      <div class="shrink-0 text-white">
        <svg class="h-6 w-6" viewBox="0 0 24 24" :fill="selected ? '#303030' : 'none'">
          <circle cx="12" cy="12" r="12" :fill="selected ? '#303030' : '#fff'" fill-opacity="0.2" />
          <path
            d="M7 13l3 3 7-7"
            stroke="#fff"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
    </div>
  </div>
</template>
