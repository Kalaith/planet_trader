import React from 'react';
import { useAuth } from './AuthContext';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { isAuthenticated, isLoading, login } = useAuth();

  if (isLoading) return <div>Loading...</div>;
  if (!isAuthenticated) {
    return (
      <div>
        <h2>Authentication Required</h2>
        <button onClick={login}>Login with WebHatchery</button>
      </div>
    );
  }
  return <>{children}</>;
};

export default ProtectedRoute;
