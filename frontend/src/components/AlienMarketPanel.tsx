import React, { useState } from 'react';
import { useGameContext } from '../contexts/GameContext';
import AlienBuyerCard from './AlienBuyerCard';

const AlienMarketPanel: React.FC = () => {
  const { alienBuyers } = useGameContext();
  const [expandedAlien, setExpandedAlien] = useState<number | null>(null);

  const toggleAlienDetails = (alienId: number) => {
    setExpandedAlien(expandedAlien === alienId ? null : alienId);
  };

  return (
    <aside className="bg-gray-800 border border-gray-700 rounded-lg shadow-lg">
      <div className="p-3 sm:p-6">
        <div className="border-b border-gray-700 pb-3 sm:pb-4 mb-3 sm:mb-6">
          <h3 className="text-lg sm:text-xl font-bold text-blue-400">👽 Alien Market</h3>
          <p className="text-xs text-gray-400 mt-1">Tap on a buyer to see details</p>
        </div>
        <div className="space-y-2 max-h-64 sm:max-h-96 overflow-y-auto">
          {alienBuyers.map(buyer => (
            <AlienBuyerCard 
              key={buyer.id} 
              buyer={buyer} 
              isExpanded={expandedAlien === buyer.id} 
              toggleAlienDetails={toggleAlienDetails} 
            />
          ))}
        </div>
      </div>
    </aside>
  );
};

export default AlienMarketPanel;
