<template>
  <div class="min-h-[300px]">
    <h2 class="text-h3 font-bold text-black">Orders</h2>
    <RadioGroup>
      <div class="flex flex-col space-y-8 mt-4 w-full">
        <RadioGroupOption
          v-for="(order, i) in allOrders"
          :key="i"
          as="template"
          :value="i"
          @click="selectItem(order.id)"
        >
          <OrderTile
            :key="order.id"
            :image="order.image"
            :selected="!!selected.includes(order.id)"
            :client="order.client"
            :index="i"
            :date="order.date"
            :products="order.products"
            :order-id="order.id"
          ></OrderTile>
        </RadioGroupOption>
      </div>
    </RadioGroup>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { RadioGroupOption, RadioGroup } from '@headlessui/vue'
import OrderTile from './ordersTile/index.vue'
import { useMainStore } from '@/stores/main'
import { mapActions, mapState, mapStores } from 'pinia'
import { loadProducts } from '@/api'

export default defineComponent({
  components: { RadioGroupOption, RadioGroup, OrderTile },
  data() {
    return {
      isLoading: false,
    }
  },
  computed: {
    ...mapStores(useMainStore),
    ...mapState(useMainStore, ['allSelected', 'selected', 'allOrders']),
  },
  async mounted() {
    this.loadOrders()
  },
  methods: {
    ...mapActions(useMainStore, ['selectItem']),
    async loadOrders() {
      const store = useMainStore()
      const res = await loadProducts()
      store.setOrders(res.orders)
    },
  },
})
</script>
