import { useState } from 'react'
import { Link } from 'react-router-dom'

export default function AuthForm({
  title,
  submitLabel,
  alternateText,
  alternateLink,
  alternateLabel,
  onSubmit,
}) {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)

  async function handleSubmit(event) {
    event.preventDefault()
    setError('')
    setLoading(true)

    try {
      await onSubmit(email, password)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="auth-card">
      <h1>{title}</h1>
      <p className="auth-subtitle">מתחבר לשרת Rust ב־{import.meta.env.VITE_API_URL}</p>

      <form className="auth-form" onSubmit={handleSubmit}>
        <label>
          אימייל
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="you@example.com"
            required
            autoComplete="email"
          />
        </label>

        <label>
          סיסמה
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="••••••••"
            required
            minLength={6}
            autoComplete={submitLabel === 'התחבר' ? 'current-password' : 'new-password'}
          />
        </label>

        {error && <p className="auth-error">{error}</p>}

        <button type="submit" disabled={loading}>
          {loading ? 'שולח...' : submitLabel}
        </button>
      </form>

      <p className="auth-switch">
        {alternateText}{' '}
        <Link to={alternateLink}>{alternateLabel}</Link>
      </p>
    </div>
  )
}
