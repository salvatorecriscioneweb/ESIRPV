import { getFromLocalStorage } from '@/utils/localStorage'

export default function useToken() {
  return getFromLocalStorage('user')
}
