import type { AuthUser } from '../contexts/authContext';

export interface FrontpageStoredUser {
  id?: number | string;
  email?: string | null;
  username?: string | null;
  display_name?: string | null;
  role?: string;
  is_guest?: boolean;
  auth_type?: 'frontpage' | 'guest';
}

interface FrontpageAuthState {
  token?: string | null;
  user?: FrontpageStoredUser | null;
  loginUrl?: string | null;
}

interface FrontpageAuthStorage {
  state?: FrontpageAuthState;
}

export interface GuestStoredSession {
  token: string;
  user: AuthUser;
}

const FRONTPAGE_AUTH_STORAGE_KEY = 'auth-storage';

const readFrontpageStorage = (): FrontpageAuthStorage | null => {
  const raw = globalThis.localStorage?.getItem(FRONTPAGE_AUTH_STORAGE_KEY);
  if (!raw) {
    return null;
  }

  try {
    return JSON.parse(raw) as FrontpageAuthStorage;
  } catch {
    return null;
  }
};

export const frontpageAuthStorage = {
  readToken(): string | null {
    const parsed = readFrontpageStorage();
    const token = parsed?.state?.token;
    const user = parsed?.state?.user;
    const isGuestUser = Boolean(user?.is_guest || user?.auth_type === 'guest');

    return !isGuestUser && typeof token === 'string' && token.trim() !== '' ? token : null;
  },

  readUser(): FrontpageStoredUser | null {
    return readFrontpageStorage()?.state?.user ?? null;
  },

  setLoginUrl(loginUrl: string): void {
    const parsed = readFrontpageStorage() ?? {};
    const state = parsed.state ?? {};
    globalThis.localStorage?.setItem(
      FRONTPAGE_AUTH_STORAGE_KEY,
      JSON.stringify({
        ...parsed,
        state: {
          ...state,
          loginUrl,
        },
      })
    );
  },

  isStorageEvent(event: StorageEvent): boolean {
    return event.key === FRONTPAGE_AUTH_STORAGE_KEY;
  },
};

export const withRedirectParam = (basePath: string): string => {
  try {
    const url = new URL(basePath, window.location.origin);
    url.searchParams.set('redirect', window.location.href);
    return url.toString();
  } catch {
    return basePath;
  }
};
