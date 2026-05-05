import axios from 'axios';
import { frontpageAuthStorage } from '../stores/authStorage';
import { useAuthStore } from '../stores/authStore';

const BASE_URL = import.meta.env.VITE_API_BASE_URL;
if (!BASE_URL) {
  throw new Error('VITE_API_BASE_URL is required.');
}

const readToken = (): string | null => {
  try {
    const frontpageToken = frontpageAuthStorage.readToken();

    if (frontpageToken) {
      return frontpageToken;
    }

    return useAuthStore.getState().guestSession?.token ?? null;
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
      const loginUrl = error.response?.data?.login_url;

      if (loginUrl) {
        try {
          frontpageAuthStorage.setLoginUrl(loginUrl);
          window.dispatchEvent(new CustomEvent('webhatchery:login-required', { detail: { loginUrl } }));
        } catch (storageError) {
          console.warn('Failed to persist login URL to auth storage', storageError);
        }
      }
    }
    return Promise.reject(error);
  }
);
