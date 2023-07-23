import { Order } from '@/interfaces'
import { defineStore } from 'pinia'

export const useMainStore = defineStore('main', {
  state: () => ({
    selected: <string[]>[],
    allOrders: new Array(10)
      .fill({
        id: '01239',
        selected: false,
        image: 'https://placehold.co/200x200',
        client: {
          name: 'Salvatore',
          surname: 'Criscione',
          id: 'BHKSLMJKJ98',
        },
        date: '12/08/39',
        products: 5,
      })
      .map((i) => ({
        ...i,
        id: Math.round(Math.random() * 100000).toString(),
      })),
  }),
  getters: {
    allSelected: (state) => state.selected.length === state.allOrders.length,
  },
  actions: {
    setOrders(orders: Order[]) {
      this.allOrders = orders
    },
    selectItem(id: string) {
      if (this.selected.includes(id)) {
        this.selected = this.selected.filter((i) => i !== id)
      } else {
        this.selected.push(id)
      }
    },
    selectAll() {
      this.selected = this.allOrders.map((order: Order) => order.id)
    },
    unSelectAll() {
      this.selected = []
    },
  },
})
