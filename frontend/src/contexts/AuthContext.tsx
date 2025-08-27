import React, { createContext, useContext, useEffect, useState } from 'react';
import type { ReactNode } from 'react';

interface AuthUser {
  user_id: number;
  email: string;
  username?: string;
  role: string;
  roles: string[];
  exp: number;
  iat: number;
}

interface AuthContextType {
  user: AuthUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: () => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (!token) {
      setIsLoading(false);
      return;
    }
    const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || '/backend/api';
    fetch(`${apiBaseUrl}/me`, {
      headers: { Authorization: `Bearer ${token}` }
    })
      .then(res => res.json())
      .then(data => {
        if (data.success && data.data) {
          setUser(data.data);
        }
        setIsLoading(false);
      })
      .catch(() => setIsLoading(false));
  }, []);

  const login = () => {
    const authPortalUrl = import.meta.env.VITE_AUTH_PORTAL_URL || '/auth';
    window.location.href = `${authPortalUrl}/login?redirect=${encodeURIComponent(window.location.href)}`;
  };

  const logout = () => {
    localStorage.removeItem('token');
    setUser(null);
    const authPortalUrl = import.meta.env.VITE_AUTH_PORTAL_URL || '/auth';
    window.location.href = `${authPortalUrl}/logout`;
  };

  return (
    <AuthContext.Provider value={{ user, isAuthenticated: !!user, isLoading, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
};
