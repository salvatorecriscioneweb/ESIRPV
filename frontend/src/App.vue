<template>
  <div class="w-full min-w-[500px] max-w-md px-2 py-16 mx-auto sm:px-0">
    <TabGroup>
      <TabList v-if="haveBar" class="flex space-x-1 rounded-xl bg-primary text-base p-1 mx-20">
        <Tab v-for="tab in tabsToSee" :key="tab.action" as="template">
          <button
            :class="[
              'w-full rounded-lg py-2.5 text-base text-white',
              'ring-white ring-opacity-60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2 text-blue-100 hover:bg-white/[0.12] hover:text-white',
            ]"
            @click="onClick(tab.action)"
          >
            {{ tab.label }}
          </button>
        </Tab>
      </TabList>
      <TabPanels class="mt-2">
        <Container>
          <router-view />
        </Container>
      </TabPanels>
    </TabGroup>
  </div>
</template>

<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router'
import { computed } from 'vue'
import { TabGroup, TabList, Tab } from '@headlessui/vue' //TabPanels, TabPanel
import Container from './components/container.vue'
import { useMainStore } from './stores/main'

const store = useMainStore()
const router = useRouter()

const haveBar = computed(() => {
  const route = useRoute()

  const haveBar = route.path !== '/'
  return haveBar
})

const tabsToSee = computed(() => {
  const TABS = [
    {
      action: 'SEND',
      label: 'Send',
    },
    {
      action: 'SELECT_UNSELECT',
      label: store.allSelected ? 'Unselect All' : 'Select all',
    },
    {
      action: 'SETTINGS',
      label: 'Settings',
    },
    {
      action: 'ORDERS',
      label: 'All orders',
    },
    {
      action: 'LOGOUT',
      label: 'Logout',
    },
  ]
  const route = useRoute()
  switch (route.path) {
    case '/settings':
      return TABS.filter((i) => i.action !== 'SETTINGS')
    case '/orders':
      return TABS.filter((i) => i.action !== 'ORDERS')
    default:
      return TABS
  }
})

function onClick(action?: string) {
  if (!action) return

  switch (action) {
    case 'SEND':
      console.log('send')
      break
    case 'LOGOUT':
      store.logout()
      break
    case 'SELECT_UNSELECT':
      if (store.allSelected) {
        store.unSelectAll()
      } else {
        store.selectAll()
      }
      break
    case 'SETTINGS':
      router.push('/settings')
      break
    case 'ORDERS':
      router.push('/orders')
      break
    default:
  }
  return
}
</script>
