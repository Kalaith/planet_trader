import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './styles/global.css'
import App from './App.tsx'
import { GameProvider } from './contexts/GameContext.tsx'
import { AuthProvider } from './contexts/AuthContext.tsx'
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
