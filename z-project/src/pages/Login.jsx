import { useNavigate } from 'react-router-dom'
import { login } from '../api/auth'
import AuthForm from '../components/AuthForm'
import { saveUser } from '../lib/session'

export default function Login() {
  const navigate = useNavigate()

  async function handleLogin(email, password) {
    const user = await login({ email, password })
    saveUser(user)
    navigate('/home')
  }

  return (
    <AuthForm
      title="התחברות"
      submitLabel="התחבר"
      alternateText="אין לך חשבון?"
      alternateLink="/register"
      alternateLabel="הירשם"
      onSubmit={handleLogin}
    />
  )
}
