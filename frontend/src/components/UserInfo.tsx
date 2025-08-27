import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const UserInfo: React.FC = () => {
  const { user, isAuthenticated, isLoading, logout } = useAuth();

  if (isLoading) return null;
  if (!isAuthenticated || !user) return null;

  return (
    <div className="user-info flex items-center gap-2 text-xs sm:text-sm text-gray-400">
      <span>Logged in as:</span>
      <span className="font-bold text-blue-300">{user.email}</span>
      {user.username && <span className="font-bold text-green-300">({user.username})</span>}
      <span className="text-gray-500">ID: {user.user_id}</span>
      <button className="ml-2 px-2 py-1 bg-gray-700 rounded hover:bg-gray-600 text-white" onClick={logout}>Logout</button>
    </div>
  );
};

export default UserInfo;
