import React from 'react';
import { useAuth } from '../contexts/useAuth';

const UserInfo: React.FC = () => {
  const { user, isAuthenticated, isLoading, getLinkAccountUrl } = useAuth();

  if (isLoading) return null;
  if (!isAuthenticated || !user) return null;

  return (
    <div className="user-info flex items-center gap-2 text-xs sm:text-sm text-gray-400 flex-wrap justify-end">
      <span>{user.is_guest ? 'Playing as guest:' : 'Logged in as:'}</span>
      <span className="font-bold text-blue-300">{user.display_name ?? user.email ?? user.username ?? 'Unknown'}</span>
      {user.username && !user.is_guest && <span className="font-bold text-green-300">({user.username})</span>}
      {user.is_guest && (
        <a
          className="rounded-full border border-cyan-500/50 px-3 py-1 text-cyan-300 transition hover:border-cyan-300 hover:text-cyan-100"
          href={getLinkAccountUrl()}
        >
          Link Account
        </a>
      )}
    </div>
  );
};

export default UserInfo;
