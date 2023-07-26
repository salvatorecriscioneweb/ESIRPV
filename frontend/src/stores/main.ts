import { Order } from '@/interfaces'
import { saveToLocalStorage } from '@/utils/localStorage'
import { defineStore } from 'pinia'

export const useMainStore = defineStore('main', {
  state: () => ({
    selected: <string[]>[],
    token: '',
    allOrders: [] as Order[],
  }),
  getters: {
    allSelected: (state) => state.selected.length === state.allOrders.length,
  },
  actions: {
    setOrders(orders: IOrderResponse[]) {
      this.allOrders = orders || []
    },
    selectItem(id: string) {
      if (this.selected.includes(id)) {
        this.selected = this.selected.filter((i) => i !== id)
      } else {
        this.selected.push(id)
      }
    },
    setToken(t: string) {
      this.token = t
    },
    logout() {
      this.token = ''
      saveToLocalStorage('user', '')
    },
    selectAll() {
      this.selected = this.allOrders.map((order: Order) => order.id)
    },
    unSelectAll() {
      this.selected = []
    },
  },
})
