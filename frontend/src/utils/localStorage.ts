export function getFromLocalStorage(key: string): string | null {
  if (!localStorage) return null
  return localStorage.getItem(key)
}

export function saveToLocalStorage(key: string, content: string): void {
  if (!localStorage) return
  localStorage.setItem(key, content)
}
