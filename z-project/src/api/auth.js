const API_URL = import.meta.env.VITE_API_URL ?? 'http://127.0.0.1:8080'

async function authRequest(path, body) {
  const response = await fetch(`${API_URL}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    const message = await response.text()
    throw new Error(message || response.statusText)
  }

  return response.json()
}

export function login(body) {
  return authRequest('/login', body)
}

export function register(body) {
  return authRequest('/register', body)
}
