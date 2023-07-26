export interface Client {
  name: string
  surname: string
  id: string
  address: string
}
export interface Order {
  id: string
  selected: boolean
  image: string
  client: Client
  date: string
  products: number
}
export interface ProductList {
  id: number
  order_id: number
  number: string
  product_id: number
  quantity: number
}
export interface ILoginRequest {
  username: string
  password: string
}
export interface ILoginResponse {
  token: string
}

export interface IOrderResponse {
  order: Order
  client: Client[]
  list: ProductList[]
}
