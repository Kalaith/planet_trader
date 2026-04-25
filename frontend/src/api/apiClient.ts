import axios from 'axios';

const BASE_URL = import.meta.env.VITE_API_BASE_URL || '';
const GUEST_AUTH_STORAGE_KEY = 'planet-trader-guest-session';

const readToken = (): string | null => {
  try {
    const frontpageRaw = localStorage.getItem('auth-storage');
    if (frontpageRaw) {
      const frontpageParsed = JSON.parse(frontpageRaw);
      const frontpageToken = frontpageParsed?.state?.token;
      if (typeof frontpageToken === 'string' && frontpageToken.trim() !== '') {
        return frontpageToken;
      }
    }

    const guestRaw = localStorage.getItem(GUEST_AUTH_STORAGE_KEY);
    if (guestRaw) {
      const guestParsed = JSON.parse(guestRaw) as { token?: string | null };
      if (typeof guestParsed?.token === 'string' && guestParsed.token.trim() !== '') {
        return guestParsed.token;
      }
    }
  } catch (error) {
    console.warn('Failed to parse auth token from local storage', error);
  }

  return null;
};

export const apiClient = axios.create({
  baseURL: BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

apiClient.interceptors.request.use(
  config => {
    const token = readToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  error => Promise.reject(error)
);

apiClient.interceptors.response.use(
  response => response,
  error => {
    if (error.response?.status === 401) {
      const loginUrl =
        error.response?.data?.login_url ||
        import.meta.env.VITE_WEB_HATCHERY_LOGIN_URL;

      if (loginUrl) {
        try {
          const raw = localStorage.getItem('auth-storage');
          const parsed = raw ? JSON.parse(raw) : {};
          const state = parsed?.state ?? {};
          const next = {
            ...parsed,
            state: {
              ...state,
              loginUrl,
            },
          };
          localStorage.setItem('auth-storage', JSON.stringify(next));
          window.dispatchEvent(new CustomEvent('webhatchery:login-required', { detail: { loginUrl } }));
        } catch (storageError) {
          console.warn('Failed to persist login URL to auth storage', storageError);
        }
      }
    }
    return Promise.reject(error);
  }
);
