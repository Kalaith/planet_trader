import { createContext } from 'react';

export interface AuthUser {
  id: number | string;
  email?: string | null;
  username?: string | null;
  roles?: string[];
}

export interface AuthContextType {
  user: AuthUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  logout: () => void;
}

export const AuthContext = createContext<AuthContextType | undefined>(undefined);
