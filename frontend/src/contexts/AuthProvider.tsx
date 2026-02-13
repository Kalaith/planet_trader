
import React, { useEffect, useState, type ReactNode } from "react";
import { AuthContext, type AuthUser } from "./authContext";

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const initUser = () => {
      try {
        const storedAuth = localStorage.getItem("auth-storage");
        if (!storedAuth) {
          setUser(null);
          return;
        }
        const parsed = JSON.parse(storedAuth) as {
          state?: { user?: AuthUser | null };
        };
        setUser(parsed.state?.user ?? null);
      } catch (err) {
        console.error("User initialization error:", err);
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
    <AuthContext.Provider
      value={{ user, isAuthenticated: !!user, isLoading, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
};

