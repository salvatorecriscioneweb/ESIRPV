export interface Client {
  name: string
  surname: string
  id: string
}
export interface Order {
  id: string
  selected: boolean
  image: string
  client: Client
  date: string
  products: number
}
export interface ILoginRequest {
  username: string
  password: string
}
export interface ILoginResponse {
  token: string
}
