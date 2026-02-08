import React from 'react';
import { useGameContext } from '../contexts/useGameContext';
import type { Planet } from '../types/entities';
import statList from '../mocks/statList.json';

// Ensure stat.id is properly typed as keyof Planet
const parsedStatList = statList.map(stat => ({
  ...stat,
  id: stat.id as keyof Planet,
  bar: new Function('p', `return ${stat.bar}`) as (p: Planet) => number
}));

// Update PlanetWorkshop to handle null planet internally
const PlanetWorkshop: React.FC = () => {
  const { currentPlanet } = useGameContext();

  if (!currentPlanet) {
    return (
      <div className="bg-gray-800 border border-gray-700 rounded-lg p-8 text-center">
        <div className="text-gray-400 mb-4">
          <div className="text-6xl mb-4">🪐</div>
          <p className="text-lg">No active planet selected</p>
          <p className="text-sm text-gray-500">Please purchase a planet to get started</p>
        </div>
      </div>
    );
  }

  if (!currentPlanet.type || !currentPlanet.type.color) {
    console.error('Planet type or color is undefined:', currentPlanet);
    return null; // Render nothing if planet is invalid
  }

  return (
    <section className="bg-gray-800 border border-gray-700 rounded-lg shadow-lg">
      <div className="p-3 sm:p-6">
        <div className="border-b border-gray-700 pb-3 sm:pb-4 mb-3 sm:mb-6">
          <h3 className="text-lg sm:text-xl font-bold text-blue-400 flex items-center gap-2">
            🪐 {currentPlanet.name}
          </h3>
        </div>
        
        <div className="flex flex-col lg:flex-row gap-4 sm:gap-6">
          {/* Planet Visual */}
          <div className="flex-shrink-0 flex justify-center">
            <div className="relative">
              <div 
                className="w-24 h-24 sm:w-32 sm:h-32 rounded-full shadow-lg relative overflow-hidden"
                style={{ backgroundColor: '#222' }}
              >
                <div 
                  className="absolute inset-0 rounded-full"
                  style={{ backgroundColor: currentPlanet.color }}
                ></div>
                <div 
                  className="absolute inset-0 rounded-full"
                  style={{ 
                    backgroundColor: '#aaf6', 
                    opacity: Math.min(currentPlanet.atmosphere || 0, 1) 
                  }}
                ></div>
                <div 
                  className="absolute inset-0 rounded-full"
                  style={{ 
                    backgroundColor: '#4fd1c5cc', 
                    opacity: currentPlanet.water || 0 
                  }}
                ></div>
              </div>
            </div>
          </div>
          
          {/* Planet Stats */}
          <div className="flex-1">
            <div className="grid grid-cols-1 gap-2 sm:gap-3">
              {parsedStatList.map(stat => (
                <div key={stat.id} className="flex items-center justify-between p-2 sm:p-3 bg-gray-700 rounded-lg">
                  <span className="text-gray-300 text-xs sm:text-sm font-medium">{stat.label}:</span>
                  <span className="text-white font-bold text-sm">
                    {typeof currentPlanet[stat.id] === 'number' 
                      ? (stat.id === 'water' 
                          ? Math.round((currentPlanet[stat.id] as number) * 100) 
                          : Math.round(currentPlanet[stat.id] as number)
                        ) 
                      : 0
                    }{stat.unit}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default PlanetWorkshop;
