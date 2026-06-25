import { useEffect } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { clearUser, getUser } from '../lib/session'

export default function Home() {
  const navigate = useNavigate()
  const user = getUser()

  useEffect(() => {
    if (!user) {
      navigate('/login', { replace: true })
    }
  }, [user, navigate])

  if (!user) return null

  function handleLogout() {
    clearUser()
    navigate('/login', { replace: true })
  }

  return (
    <div className="auth-card">
      <h1>ברוך הבא</h1>
      <p className="auth-subtitle">התחברת בהצלחה לשרת</p>

      <div className="user-info">
        <p>
          <strong>אימייל:</strong> {user.email}
        </p>
        <p>
          <strong>מזהה:</strong> {user.id ?? '—'}
        </p>
      </div>

      <div className="home-actions">
        <button type="button" onClick={handleLogout}>
          התנתק
        </button>
        <Link to="/login" className="text-link">
          חזרה להתחברות
        </Link>
      </div>
    </div>
  )
}
