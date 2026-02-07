import React, { createContext, useContext, useEffect, useState } from 'react';
import type { ReactNode } from 'react';

interface AuthUser {
  id: number | string;
  email?: string | null;
  username?: string | null;
  roles?: string[];
}

interface AuthContextType {
  user: AuthUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const initUser = () => {
      try {
        const storedAuth = localStorage.getItem('auth-storage');
        if (!storedAuth) {
          setUser(null);
          return;
        }
        const parsed = JSON.parse(storedAuth) as { state?: { user?: AuthUser | null } };
        setUser(parsed.state?.user ?? null);
      } catch (err) {
        console.error('User initialization error:', err);
        setUser(null);
      } finally {
        setIsLoading(false);
      }
    };

    initUser();
  }, []);

  const logout = () => {
    // Web Hatchery manages auth state; no-op here.
  };

  return (
    <AuthContext.Provider value={{ user, isAuthenticated: !!user, isLoading, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
};
