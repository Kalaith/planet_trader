import React from 'react';
import { useAuth } from './useAuth';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) return <div>Loading...</div>;
  if (!isAuthenticated) {
    return (
      <div>
        <h2>Not logged in</h2>
      </div>
    );
  }
  return <>{children}</>;
};

export default ProtectedRoute;
