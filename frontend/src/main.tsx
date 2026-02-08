import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './styles/global.css'
import App from './App.tsx'
import { GameProvider } from './contexts/GameProvider'
import { AuthProvider } from './contexts/AuthProvider'
import ProtectedRoute from './contexts/ProtectedRoute.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthProvider>
      <ProtectedRoute>
        <GameProvider>
          <App />
        </GameProvider>
      </ProtectedRoute>
    </AuthProvider>
  </StrictMode>,
)
