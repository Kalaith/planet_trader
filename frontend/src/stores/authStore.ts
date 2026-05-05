import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import type { AuthMode, AuthUser } from '../contexts/authContext';
import { frontpageAuthStorage, withRedirectParam, type GuestStoredSession } from './authStorage';

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

interface AuthStoreState {
  user: AuthUser | null;
  token: string | null;
  authMode: AuthMode;
  isLoading: boolean;
  error: string | null;
  guestSession: GuestStoredSession | null;
  hasAttemptedGuestLink: boolean;
  initializeAuth: () => void;
  syncTokenFromStorage: () => void;
  loginWithRedirect: () => void;
  continueAsGuest: () => Promise<void>;
  getLinkAccountUrl: () => string;
  logout: () => void;
  refreshUser: () => Promise<void>;
  maybeLinkGuestSession: () => Promise<void>;
  clearGuestSession: () => void;
}

const trimTrailingSlash = (value: string): string => value.replace(/\/+$/, '');

const requiredEnv = (
  key: 'VITE_API_BASE_URL' | 'VITE_WEB_HATCHERY_LOGIN_URL' | 'VITE_WEB_HATCHERY_SIGNUP_URL'
): string => {
  const value = import.meta.env[key];
  if (!value) {
    throw new Error(`${key} is required.`);
  }

  return value;
};

const buildApiUrl = (path: string): string => `${trimTrailingSlash(requiredEnv('VITE_API_BASE_URL'))}${path}`;

const normalizeUser = (
  user: AuthUser,
  authMode: AuthMode,
  frontpageUser: ReturnType<typeof frontpageAuthStorage.readUser>
): AuthUser => {
  const isGuest = Boolean(user.is_guest || authMode === 'guest');

  return {
    ...user,
    id: String(user.id),
    username: authMode === 'frontpage' ? (frontpageUser?.username || user.username) : user.username,
    display_name:
      authMode === 'frontpage'
        ? (frontpageUser?.display_name || frontpageUser?.username || user.display_name)
        : user.display_name,
    email: authMode === 'frontpage' ? (frontpageUser?.email || user.email) : user.email,
    role: authMode === 'frontpage' ? (frontpageUser?.role || user.role || 'player') : (user.role || 'player'),
    is_guest: isGuest,
    auth_type: isGuest ? 'guest' : 'frontpage',
  };
};

export const useAuthStore = create<AuthStoreState>()(
  persist(
    (set, get) => ({
      user: null,
      token: null,
      authMode: null,
      isLoading: true,
      error: null,
      guestSession: null,
      hasAttemptedGuestLink: false,

      initializeAuth: () => {
        const frontpageToken = frontpageAuthStorage.readToken();
        if (frontpageToken) {
          set({ token: frontpageToken, authMode: 'frontpage', user: null, isLoading: true });
          return;
        }

        const guestSession = get().guestSession;
        set({
          token: guestSession?.token ?? null,
          authMode: guestSession?.token ? 'guest' : null,
          user: guestSession?.user ?? null,
          isLoading: false,
        });
      },

      syncTokenFromStorage: () => {
        get().initializeAuth();
      },

      loginWithRedirect: () => {
        const existingToken = frontpageAuthStorage.readToken();
        set({ error: null });

        if (existingToken) {
          set({ token: existingToken, authMode: 'frontpage' });
          return;
        }

        window.location.href = withRedirectParam(requiredEnv('VITE_WEB_HATCHERY_LOGIN_URL'));
      },

      getLinkAccountUrl: () => withRedirectParam(requiredEnv('VITE_WEB_HATCHERY_SIGNUP_URL')),

      continueAsGuest: async () => {
        const existingSession = get().guestSession;
        set({ isLoading: true, error: null });

        if (existingSession?.token && existingSession.user) {
          set({
            token: existingSession.token,
            authMode: 'guest',
            user: existingSession.user,
            isLoading: false,
          });
          return;
        }

        try {
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

          set({
            guestSession,
            token: guestSession.token,
            authMode: 'guest',
            user: guestSession.user,
            isLoading: false,
          });
        } catch (error) {
          set({
            guestSession: null,
            token: null,
            authMode: null,
            user: null,
            isLoading: false,
            error: error instanceof Error ? error.message : 'Failed to create guest session',
          });
          throw error;
        }
      },

      logout: () => {
        if (get().authMode === 'guest') {
          set({ guestSession: null, token: null, authMode: null, user: null, error: null });
          return;
        }

        window.location.href = withRedirectParam(requiredEnv('VITE_WEB_HATCHERY_LOGIN_URL'));
      },

      refreshUser: async () => {
        const { token, authMode } = get();
        if (!token) {
          set({ user: null, error: null, isLoading: false });
          return;
        }

        set({ isLoading: true, error: null });

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

          const frontpageUser = authMode === 'frontpage' ? frontpageAuthStorage.readUser() : null;
          set({
            user: normalizeUser(result.data.user, authMode, frontpageUser),
            isLoading: false,
          });
        } catch (error) {
          set({
            guestSession: authMode === 'guest' ? null : get().guestSession,
            user: null,
            token: null,
            authMode: null,
            isLoading: false,
            error: error instanceof Error ? error.message : 'Failed to validate session',
          });
        }
      },

      maybeLinkGuestSession: async () => {
        const { authMode, token, user, guestSession, hasAttemptedGuestLink } = get();
        if (!guestSession?.token || hasAttemptedGuestLink || authMode !== 'frontpage' || !token || !user || user.is_guest) {
          return;
        }

        set({ hasAttemptedGuestLink: true });

        try {
          const response = await fetch(buildApiUrl('/api/auth/link-guest'), {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({ guest_token: guestSession.token }),
          });

          const result = (await response.json()) as ApiEnvelope<unknown>;
          if (!response.ok || !result.success) {
            throw new Error(result.message || result.error || 'Failed to link guest data');
          }

          set({ guestSession: null });
        } catch (error) {
          set({ error: error instanceof Error ? error.message : 'Failed to link guest account data' });
        }
      },

      clearGuestSession: () => {
        set({ guestSession: null });
      },
    }),
    {
      name: 'planet-trader-auth',
      storage: createJSONStorage(() => localStorage),
      partialize: state => ({
        guestSession: state.guestSession,
      }),
    }
  )
);
