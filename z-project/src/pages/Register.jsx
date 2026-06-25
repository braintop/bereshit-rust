import { useNavigate } from 'react-router-dom'
import { register } from '../api/auth'
import AuthForm from '../components/AuthForm'
import { saveUser } from '../lib/session'

export default function Register() {
  const navigate = useNavigate()

  async function handleRegister(email, password) {
    const user = await register({ email, password })
    saveUser(user)
    navigate('/home')
  }

  return (
    <AuthForm
      title="הרשמה"
      submitLabel="צור חשבון"
      alternateText="כבר יש לך חשבון?"
      alternateLink="/login"
      alternateLabel="התחבר"
      onSubmit={handleRegister}
    />
  )
}
