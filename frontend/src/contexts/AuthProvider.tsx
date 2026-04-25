import React, { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';
import { AuthContext, type AuthContextType, type AuthUser, type AuthMode } from './authContext';

interface AuthProviderProps {
  children: ReactNode;
}

interface FrontpageStoredUser {
  id?: number | string;
  email?: string | null;
  username?: string | null;
  display_name?: string | null;
  role?: string;
}

interface GuestStoredSession {
  token: string;
  user: AuthUser;
}

interface ApiEnvelope<T> {
  success: boolean;
  message?: string;
  error?: string;
  data?: T;
}

interface SessionPayload {
  user: AuthUser;
}

interface GuestSessionPayload {
  token: string;
  user: AuthUser;
}

const FRONTPAGE_AUTH_STORAGE_KEY = 'auth-storage';
const GUEST_AUTH_STORAGE_KEY = 'planet-trader-guest-session';

const trimTrailingSlash = (value: string): string => value.replace(/\/+$/, '');

const buildApiUrl = (path: string): string => {
  const base = trimTrailingSlash(import.meta.env.VITE_API_BASE_URL || '');
  if (!base) {
    return path;
  }
  return `${base}${path}`;
};

const readFrontpageToken = (): string | null => {
  const authStorage = localStorage.getItem(FRONTPAGE_AUTH_STORAGE_KEY);
  if (!authStorage) {
    return null;
  }

  try {
    const parsed = JSON.parse(authStorage) as { state?: { token?: string | null } };
    const token = parsed?.state?.token;
    return typeof token === 'string' && token.trim() !== '' ? token : null;
  } catch {
    return null;
  }
};

const readFrontpageUser = (): FrontpageStoredUser | null => {
  const authStorage = localStorage.getItem(FRONTPAGE_AUTH_STORAGE_KEY);
  if (!authStorage) {
    return null;
  }

  try {
    const parsed = JSON.parse(authStorage) as { state?: { user?: FrontpageStoredUser | null } };
    return parsed?.state?.user ?? null;
  } catch {
    return null;
  }
};

const readGuestSession = (): GuestStoredSession | null => {
  const raw = localStorage.getItem(GUEST_AUTH_STORAGE_KEY);
  if (!raw) {
    return null;
  }

  try {
    const parsed = JSON.parse(raw) as GuestStoredSession;
    if (!parsed?.token || !parsed?.user?.id) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
};

const saveGuestSession = (session: GuestStoredSession): void => {
  localStorage.setItem(GUEST_AUTH_STORAGE_KEY, JSON.stringify(session));
};

const clearGuestSession = (): void => {
  localStorage.removeItem(GUEST_AUTH_STORAGE_KEY);
};

const withRedirectParam = (basePath: string): string => {
  try {
    const url = new URL(basePath, window.location.origin);
    url.searchParams.set('redirect', window.location.href);
    return url.toString();
  } catch {
    return basePath;
  }
};

const appendQueryParam = (urlValue: string, key: string, value: string): string => {
  try {
    const url = new URL(urlValue, window.location.origin);
    url.searchParams.set(key, value);
    return url.toString();
  } catch {
    return urlValue;
  }
};

const getLoginUrl = (): string => withRedirectParam(import.meta.env.VITE_WEB_HATCHERY_LOGIN_URL || '/login');
const getSignupUrl = (): string => withRedirectParam(import.meta.env.VITE_WEB_HATCHERY_SIGNUP_URL || '/signup');

const readGuestUserIdFromUrl = (): string | null => {
  try {
    const value = new URL(window.location.href).searchParams.get('guest_user_id') || '';
    return value.trim() || null;
  } catch {
    return null;
  }
};

const removeGuestUserIdFromUrl = (): void => {
  try {
    const url = new URL(window.location.href);
    url.searchParams.delete('guest_user_id');
    window.history.replaceState({}, '', url.toString());
  } catch {
    // ignore
  }
};

const getStoredAuth = (): { token: string | null; mode: AuthMode; guestUser: AuthUser | null } => {
  const frontpageToken = readFrontpageToken();
  if (frontpageToken) {
    return { token: frontpageToken, mode: 'frontpage', guestUser: null };
  }

  const guest = readGuestSession();
  if (guest?.token) {
    return { token: guest.token, mode: 'guest', guestUser: guest.user };
  }

  return { token: null, mode: null, guestUser: null };
};

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [token, setToken] = useState<string | null>(null);
  const [authMode, setAuthMode] = useState<AuthMode>(null);
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const hasAttemptedGuestLinkRef = useRef(false);

  const syncTokenFromStorage = useCallback(() => {
    const next = getStoredAuth();
    setToken(next.token);
    setAuthMode(next.mode);
    if (next.mode === 'guest') {
      setUser(next.guestUser);
    }
  }, []);

  const loginWithRedirect = useCallback(() => {
    const existingToken = readFrontpageToken();
    setError(null);

    if (existingToken) {
      setToken(existingToken);
      setAuthMode('frontpage');
      return;
    }

    window.location.href = getLoginUrl();
  }, []);

  const getLinkAccountUrl = useCallback((): string => {
    const base = getSignupUrl();
    if (user?.is_guest && user.id) {
      return appendQueryParam(base, 'guest_user_id', String(user.id));
    }
    return base;
  }, [user]);

  const continueAsGuest = useCallback(async (): Promise<void> => {
    setIsLoading(true);
    setError(null);

    try {
      const existingSession = readGuestSession();
      if (existingSession?.token && existingSession.user) {
        setToken(existingSession.token);
        setAuthMode('guest');
        setUser(existingSession.user);
        return;
      }

      const response = await fetch(buildApiUrl('/api/auth/guest-session'), {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      const result = (await response.json()) as ApiEnvelope<GuestSessionPayload>;
      if (!response.ok || !result.success || !result.data?.token || !result.data?.user) {
        throw new Error(result.message || result.error || 'Failed to create guest session');
      }

      const guestSession: GuestStoredSession = {
        token: result.data.token,
        user: {
          ...result.data.user,
          is_guest: true,
          auth_type: 'guest',
        },
      };

      saveGuestSession(guestSession);
      setToken(guestSession.token);
      setAuthMode('guest');
      setUser(guestSession.user);
    } catch (err) {
      clearGuestSession();
      setToken(null);
      setAuthMode(null);
      setUser(null);
      setError(err instanceof Error ? err.message : 'Failed to create guest session');
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  const logout = useCallback(() => {
    if (authMode === 'guest') {
      clearGuestSession();
      setToken(null);
      setAuthMode(null);
      setUser(null);
      setError(null);
      return;
    }

    window.location.href = getLoginUrl();
  }, [authMode]);

  const refreshUser = useCallback(async (): Promise<void> => {
    if (!token) {
      setUser(null);
      setError(null);
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(buildApiUrl('/api/auth/session'), {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      const result = (await response.json()) as ApiEnvelope<SessionPayload>;
      if (!response.ok || !result.success || !result.data?.user) {
        throw new Error(result.message || result.error || 'Authentication check failed');
      }

      const frontpageUser = authMode === 'frontpage' ? readFrontpageUser() : null;
      const isGuest = Boolean(result.data.user.is_guest || authMode === 'guest');
      setUser({
        ...result.data.user,
        id: String(result.data.user.id),
        username: authMode === 'frontpage' ? (frontpageUser?.username || result.data.user.username) : result.data.user.username,
        display_name:
          authMode === 'frontpage'
            ? (frontpageUser?.display_name || frontpageUser?.username || result.data.user.display_name)
            : result.data.user.display_name,
        email: authMode === 'frontpage' ? (frontpageUser?.email || result.data.user.email) : result.data.user.email,
        role: authMode === 'frontpage' ? (frontpageUser?.role || result.data.user.role || 'player') : (result.data.user.role || 'player'),
        is_guest: isGuest,
        auth_type: isGuest ? 'guest' : 'frontpage',
      });
    } catch (err) {
      if (authMode === 'guest') {
        clearGuestSession();
      }
      setUser(null);
      setToken(null);
      setAuthMode(null);
      setError(err instanceof Error ? err.message : 'Failed to validate session');
    } finally {
      setIsLoading(false);
    }
  }, [authMode, token]);

  useEffect(() => {
    const next = getStoredAuth();
    setToken(next.token);
    setAuthMode(next.mode);
    setUser(next.mode === 'guest' ? next.guestUser : null);
    setIsLoading(false);
  }, []);

  useEffect(() => {
    void refreshUser();
  }, [refreshUser]);

  useEffect(() => {
    const onStorage = (event: StorageEvent) => {
      if (event.key === FRONTPAGE_AUTH_STORAGE_KEY || event.key === GUEST_AUTH_STORAGE_KEY) {
        syncTokenFromStorage();
      }
    };

    window.addEventListener('storage', onStorage);
    return () => window.removeEventListener('storage', onStorage);
  }, [syncTokenFromStorage]);

  useEffect(() => {
    const guestUserId = readGuestUserIdFromUrl();
    if (!guestUserId || hasAttemptedGuestLinkRef.current) {
      return;
    }

    if (authMode !== 'frontpage' || !token || !user || user.is_guest) {
      return;
    }

    hasAttemptedGuestLinkRef.current = true;

    (async () => {
      try {
        const response = await fetch(buildApiUrl('/api/auth/link-guest'), {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ guest_user_id: guestUserId }),
        });

        const result = (await response.json()) as ApiEnvelope<unknown>;
        if (!response.ok || !result.success) {
          throw new Error(result.message || result.error || 'Failed to link guest data');
        }

        clearGuestSession();
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to link guest account data');
      } finally {
        removeGuestUserIdFromUrl();
      }
    })();
  }, [authMode, token, user]);

  const value = useMemo<AuthContextType>(
    () => ({
      user,
      isAuthenticated: Boolean(token && user),
      isLoading,
      error,
      authMode,
      loginWithRedirect,
      continueAsGuest,
      getLinkAccountUrl,
      logout,
      refreshUser,
    }),
    [authMode, continueAsGuest, error, getLinkAccountUrl, isLoading, loginWithRedirect, logout, refreshUser, token, user]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
