export default function getApiHost(): string {
  console.log(process.env)
  return process.env?.API_URL || process.env?.VUE_APP_API_BASE_URL || ''
}
