import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './styles/global.css';
import App from './App.tsx';
import ProtectedRoute from './contexts/ProtectedRoute.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ProtectedRoute>
      <App />
    </ProtectedRoute>
  </StrictMode>
);
