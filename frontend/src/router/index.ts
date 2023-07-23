import { RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Login',
    component: () => import(/* webpackChunkName: "login" */ '../views/login/appLogin.vue'),
  },
  {
    path: '/orders',
    name: 'Orders',
    component: () => import(/* webpackChuckName: "orders" */ '../views/orders/allOrders.vue'),
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
