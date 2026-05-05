import { useEffect } from 'react';
import type { AuthContextType } from './authContext';
import { frontpageAuthStorage } from '../stores/authStorage';
import { useAuthStore } from '../stores/authStore';

export const useAuth = (): AuthContextType => {
  const store = useAuthStore();

  return {
    user: store.user,
    isAuthenticated: Boolean(store.token && store.user),
    isLoading: store.isLoading,
    error: store.error,
    authMode: store.authMode,
    loginWithRedirect: store.loginWithRedirect,
    continueAsGuest: store.continueAsGuest,
    getLinkAccountUrl: store.getLinkAccountUrl,
    logout: store.logout,
    refreshUser: store.refreshUser,
  };
};

export const useAuthEffects = (): void => {
  const initializeAuth = useAuthStore(state => state.initializeAuth);
  const refreshUser = useAuthStore(state => state.refreshUser);
  const maybeLinkGuestSession = useAuthStore(state => state.maybeLinkGuestSession);
  const syncTokenFromStorage = useAuthStore(state => state.syncTokenFromStorage);
  const token = useAuthStore(state => state.token);
  const authMode = useAuthStore(state => state.authMode);
  const user = useAuthStore(state => state.user);

  useEffect(() => {
    initializeAuth();
  }, [initializeAuth]);

  useEffect(() => {
    void refreshUser();
  }, [refreshUser, token]);

  useEffect(() => {
    void maybeLinkGuestSession();
  }, [authMode, maybeLinkGuestSession, token, user]);

  useEffect(() => {
    const onStorage = (event: StorageEvent): void => {
      if (frontpageAuthStorage.isStorageEvent(event)) {
        syncTokenFromStorage();
      }
    };

    window.addEventListener('storage', onStorage);
    return () => window.removeEventListener('storage', onStorage);
  }, [syncTokenFromStorage]);
};
