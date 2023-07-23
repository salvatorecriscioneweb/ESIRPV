const DEBUG_ENABLED = true

export default function dbg(str: string) {
  if (!DEBUG_ENABLED) return
  console.log(`[!!!] ${str}`)
}
