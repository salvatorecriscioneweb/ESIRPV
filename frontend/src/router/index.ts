import useToken from '@/hooks/useToken'
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

router.beforeEach(async (to) => {
  if (to.path === '/login') return true
  const user = useToken()
  if (!!user && user !== '') {
    return true
  } else {
    return {
      name: 'Login',
    }
  }
})

export default router
