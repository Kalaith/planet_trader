
import React from "react";
import { useAuth } from "../contexts/useAuth";

const UserInfo: React.FC = () => {
  const { user, isAuthenticated, isLoading } = useAuth();

  if (isLoading) return null;
  if (!isAuthenticated || !user) return null;

  return (
    <div className="user-info flex items-center gap-2 text-xs sm:text-sm text-gray-400">
      <span>Logged in as:</span>
      <span className="font-bold text-blue-300">{user.email ?? "Unknown"}</span>
      {user.username && (
        <span className="font-bold text-green-300">({user.username})</span>
      )}
      <span className="text-gray-500">ID: {user.id}</span>
    </div>
  );
};

export default UserInfo;
