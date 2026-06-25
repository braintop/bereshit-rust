const STORAGE_KEY = 'auth_user'

export function saveUser(user) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(user))
}

export function getUser() {
  const raw = localStorage.getItem(STORAGE_KEY)
  if (!raw) return null

  try {
    return JSON.parse(raw)
  } catch {
    return null
  }
}

export function clearUser() {
  localStorage.removeItem(STORAGE_KEY)
}
