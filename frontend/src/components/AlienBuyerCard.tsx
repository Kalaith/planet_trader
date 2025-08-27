import React from 'react';
import { useGameContext } from '../contexts/GameContext';
import { tool_categories } from '../types/entities';
import type { Planet } from '../types/entities';

const CompactAlienBuyerCard: React.FC<{ buyer: any; compatibility: number; currentPlanet: Planet; toggleAlienDetails: (alienId: number) => void; sellPlanet: (buyer: any) => void }> = ({ buyer, compatibility, currentPlanet, toggleAlienDetails, sellPlanet }) => (
  <div 
    className="p-3 cursor-pointer hover:bg-gray-600 transition-colors duration-200"
    onClick={() => toggleAlienDetails(buyer.id)}
  >
    <div className="flex items-center justify-between">
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <span className="font-bold text-white text-sm">{buyer.name}</span>
          <span className={`text-xs font-medium ${compatibility >= 0.8 ? 'text-green-400' : compatibility >= 0.6 ? 'text-yellow-400' : 'text-red-400'}`}>
            {Math.round(compatibility * 100)}%
          </span>
        </div>
        <table className="text-xs">
          <thead>
            <tr>
              {tool_categories.map(category => (
                <th key={category.id} className="px-2 py-1 text-center">{category.icon}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            <tr>
              {tool_categories.map(category => {
                const rangeKey = {
                  temperature: 'tempRange',
                  atmosphere: 'atmoRange',
                  water: 'waterRange',
                  gravity: 'gravRange',
                  radiation: 'radRange',
                  infrastructure: 'infraRange',
                }[category.id];

                if (!rangeKey) return null;

                const valueKey = category.id as keyof Planet;
                const range = buyer[rangeKey] || [0, 0];
                const value = currentPlanet[valueKey] || 0;
                const met = value >= range[0] && value <= range[1];

                return (
                  <td key={category.id} className={`px-2 py-1 text-center ${met ? 'text-green-400' : 'text-red-400'}`}>{met ? '✓' : '✗'}</td>
                );
              })}
            </tr>
          </tbody>
        </table>
      </div>
      <div className="flex items-center gap-2 ml-2">
        <button
          className={`py-1 px-2 rounded text-sm font-semibold ${compatibility >= 0.6 ? 'bg-green-600 hover:bg-green-700 text-white' : 'bg-gray-600 text-gray-400 cursor-not-allowed'}`}
          onClick={(e) => {
            e.stopPropagation();
            if (compatibility >= 0.6) sellPlanet(buyer);
          }}
          disabled={compatibility < 0.6}
        >
          Sell
        </button>
      </div>
    </div>
  </div>
);

const ExpandedAlienBuyerCard: React.FC<{ buyer: any; currentPlanet: Planet }> = ({ buyer, currentPlanet }) => (
  <div className="px-3 pb-3 border-t border-gray-600">
    <div className="pt-3 space-y-3">
      <div className="text-gray-300 text-sm leading-relaxed">{buyer.description}</div>
      <div className="grid grid-cols-2 gap-2 text-xs">
        {tool_categories.map(category => {
          const rangeKey = {
            temperature: 'tempRange',
            atmosphere: 'atmoRange',
            water: 'waterRange',
            gravity: 'gravRange',
            radiation: 'radRange',
            infrastructure: 'infraRange',
          }[category.id];

          if (!rangeKey) return null;

          const range = buyer[rangeKey] || [0, 0];
          const value = currentPlanet[category.id as keyof Planet] || 0;
          const met = value >= range[0] && value <= range[1];

          return (
            <div
              key={category.id}
              className={`rounded p-2 border ${met ? 'bg-green-900/30 border-green-500' : 'bg-red-900/30 border-red-500'}`}
            >
              <div className="text-gray-400">{category.label}</div>
              <div className="text-xs text-white">
                {range[0]} to {range[1]}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  </div>
);

const AlienBuyerCard: React.FC<{ buyer: any; isExpanded: boolean; toggleAlienDetails: (alienId: number) => void }> = ({ buyer, isExpanded, toggleAlienDetails }) => {
  const { currentPlanet, sellPlanet } = useGameContext();

  if (!currentPlanet) {
    return null; // Do not render if currentPlanet is null
  }

  const calculateCompatibility = (buyer: any) => {
    const defaultMatches = {
      temperature: false,
      atmosphere: false,
      water: false,
      gravity: false,
      radiation: false
    };

    if (!currentPlanet) return { compatibility: 0, matches: defaultMatches };

    let score = 0;
    const matches = {
      temperature: currentPlanet.temperature >= (buyer.tempRange?.[0] || 0) && currentPlanet.temperature <= (buyer.tempRange?.[1] || 0),
      atmosphere: currentPlanet.atmosphere >= (buyer.atmoRange?.[0] || 0) && currentPlanet.atmosphere <= (buyer.atmoRange?.[1] || 0),
      water: currentPlanet.water >= (buyer.waterRange?.[0] || 0) && currentPlanet.water <= (buyer.waterRange?.[1] || 0),
      gravity: currentPlanet.gravity >= (buyer.gravRange?.[0] || 0) && currentPlanet.gravity <= (buyer.gravRange?.[1] || 0),
      radiation: currentPlanet.radiation >= (buyer.radRange?.[0] || 0) && currentPlanet.radiation <= (buyer.radRange?.[1] || 0)
    };

    score = Object.values(matches).filter(Boolean).length;

    const compatibility = score / 5;
    let price = buyer.currentPrice;
    if (compatibility >= 0.8) price = Math.floor(price * 1.2);
    const canSell = compatibility >= 0.6;

    return { compatibility, canSell, price, matches };
  };

  const { compatibility } = calculateCompatibility(buyer);

  return (
    <div key={buyer.id} className="bg-gray-700 border border-gray-600 rounded-lg overflow-hidden">
      <CompactAlienBuyerCard buyer={buyer} compatibility={compatibility} currentPlanet={currentPlanet} toggleAlienDetails={toggleAlienDetails} sellPlanet={sellPlanet} />
      {isExpanded && <ExpandedAlienBuyerCard buyer={buyer} currentPlanet={currentPlanet} />}
    </div>
  );
};

export default AlienBuyerCard;
