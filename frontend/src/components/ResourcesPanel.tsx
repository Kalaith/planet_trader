import React from 'react';
import { useGameContext } from '../contexts/GameContext';
import UserInfo from './UserInfo';

const ResourcesPanel: React.FC = () => {
  const { credits, currentPlanet, showPlanetPurchaseModal } = useGameContext();

  return (
    <header className="bg-gray-800 border border-gray-700 rounded-lg p-3 sm:p-6 shadow-lg">
      <div className="flex flex-col space-y-3 sm:flex-row sm:justify-between sm:items-center sm:space-y-0">
        <div className="company-logo text-center sm:text-left">
          <h1 className="text-xl sm:text-2xl font-bold text-blue-400">🪐 Terraforming Co.</h1>
        </div>
        <div className="resources-info flex flex-col space-y-2 sm:flex-row sm:space-y-0 sm:gap-4 sm:items-center">
          <div className="resource-item flex items-center justify-between sm:justify-start gap-2">
            <span className="text-gray-300 text-xs sm:text-sm">Credits:</span>
            <span className="text-green-400 font-bold text-sm sm:text-lg">{credits.toLocaleString()}₵</span>
          </div>
          <div className="resource-item flex items-center justify-between sm:justify-start gap-2">
            <span className="text-gray-300 text-xs sm:text-sm">Current Planet:</span>
            <span className="text-blue-300 font-semibold text-sm truncate max-w-32 sm:max-w-none">{currentPlanet ? currentPlanet.name : 'None'}</span>
          </div>
          <button 
            className="bg-blue-600 hover:bg-blue-700 text-white px-3 py-2 rounded-lg text-sm font-semibold transition-colors duration-200 flex items-center justify-center gap-2"
            onClick={showPlanetPurchaseModal}
          >
            🛒 <span className="hidden sm:inline">Buy New Planet</span><span className="sm:hidden">Buy Planet</span>
          </button>
        </div>
        {/* Show logged-in user info */}
        <UserInfo />
      </div>
    </header>
  );
};

export default ResourcesPanel;
