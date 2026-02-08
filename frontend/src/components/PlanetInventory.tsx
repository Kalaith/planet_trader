import React from 'react';
import { useGameContext } from '../contexts/useGameContext';

const PlanetInventory: React.FC = () => {
  const { planets, currentPlanet, showPlanetPurchaseModal, selectPlanet } = useGameContext();

  return (
    <section className="bg-gray-800 border border-gray-700 rounded-lg shadow-lg">
      <div className="p-3 sm:p-6">
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center border-b border-gray-700 pb-3 sm:pb-4 mb-3 sm:mb-6 gap-2 sm:gap-0">
          <h3 className="text-lg sm:text-xl font-bold text-blue-400">🌍 Planet Inventory</h3>
          <button 
            className="bg-gray-600 hover:bg-gray-500 text-white px-3 py-1 rounded-lg text-xs sm:text-sm font-semibold transition-colors duration-200 w-full sm:w-auto"
            onClick={showPlanetPurchaseModal}
          >
            🛒 Buy Planet
          </button>
        </div>
        <div className="max-h-48 sm:max-h-64 overflow-y-auto">
          {planets.length === 0 ? (
            <div className="text-center text-gray-400 py-6 sm:py-8">
              <div className="text-3xl sm:text-4xl mb-2">🌌</div>
              <p className="text-sm sm:text-base">No planets owned yet</p>
              <p className="text-xs sm:text-sm text-gray-500">Buy your first planet to get started!</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 gap-2 sm:gap-3">
              {planets.map(planet => (
                <div
                  key={planet.id || planet.name}
                  className={`p-2 sm:p-3 rounded-lg border cursor-pointer transition-all duration-200 ${
                    planet === currentPlanet 
                      ? 'bg-blue-900 border-blue-500 shadow-lg' 
                      : 'bg-gray-700 border-gray-600 hover:bg-gray-600 hover:border-gray-500'
                  }`}
                  onClick={() => selectPlanet(planet)}
                >
                  <div className="flex items-center gap-3">
                    <div 
                      className="w-6 h-6 sm:w-8 sm:h-8 rounded-full border-2 border-gray-500 flex-shrink-0"
                      style={{ backgroundColor: planet.color }}
                    ></div>
                    <div className="flex-1 min-w-0">
                      <div className="font-semibold text-white text-sm truncate">{planet.name}</div>
                      <div className="text-xs text-gray-400 truncate">{planet.type?.name}</div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </section>
  );
};

export default PlanetInventory;
