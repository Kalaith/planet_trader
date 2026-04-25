import { createContext } from 'react';

export type AuthMode = 'frontpage' | 'guest' | null;

export interface AuthUser {
  id: number | string;
  email?: string | null;
  username?: string | null;
  display_name?: string | null;
  roles?: string[];
  role?: string;
  is_guest?: boolean;
  auth_type?: 'frontpage' | 'guest';
}

export interface AuthContextType {
  user: AuthUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  authMode: AuthMode;
  loginWithRedirect: () => void;
  continueAsGuest: () => Promise<void>;
  getLinkAccountUrl: () => string;
  logout: () => void;
  refreshUser: () => Promise<void>;
}

export const AuthContext = createContext<AuthContextType | undefined>(undefined);
