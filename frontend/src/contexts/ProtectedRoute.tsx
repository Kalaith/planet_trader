import React from 'react';
import { useAuth } from './useAuth';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { isAuthenticated, isLoading, continueAsGuest, loginWithRedirect } = useAuth();

  if (isLoading) {
    return <div className="min-h-screen flex items-center justify-center text-gray-300">Loading...</div>;
  }

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center px-4">
        <div className="max-w-md w-full rounded-2xl border border-gray-800 bg-gray-900 p-6 shadow-2xl">
          <p className="text-xs uppercase tracking-[0.3em] text-cyan-400">Planet Trader</p>
          <h2 className="mt-3 text-2xl font-bold">Choose how to enter</h2>
          <p className="mt-2 text-sm text-gray-400">
            Start instantly as a guest or sign in to keep your progress attached to your account.
          </p>
          <div className="mt-6 grid gap-3">
            <button
              className="rounded-lg bg-cyan-500 px-4 py-3 font-semibold text-gray-950 transition hover:bg-cyan-400"
              onClick={() => {
                void continueAsGuest();
              }}
            >
              Continue as Guest
            </button>
            <button
              className="rounded-lg border border-gray-700 px-4 py-3 font-semibold text-white transition hover:border-cyan-400 hover:text-cyan-300"
              onClick={loginWithRedirect}
            >
              Sign In
            </button>
          </div>
        </div>
      </div>
    );
  }
  return <>{children}</>;
};

export default ProtectedRoute;
