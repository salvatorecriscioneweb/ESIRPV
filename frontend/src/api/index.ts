import { ILoginRequest, ILoginResponse, IOrderResponse } from '@/interfaces'
import getApiHost from '@/utils/host'
import axios from 'axios'

export function login(data: ILoginRequest): Promise<{ data: ILoginResponse; status: number }> {
  const bodyFormData = new FormData()
  bodyFormData.append('username', data.username)
  bodyFormData.append('password', data.password)
  return axios(`${getApiHost()}/api-token-auth/`, {
    method: 'POST',
    data: bodyFormData,
    headers: { 'Content-Type': 'multipart/form-data' },
  })
}

export function loadProducts(): Promise<{
  orders: IOrderResponse[]
}> {
  return axios(`${getApiHost()}/get-orders`, {
    method: 'GET',
  }).then((res) => res.data)
}
