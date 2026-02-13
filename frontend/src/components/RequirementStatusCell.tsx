import React from 'react';
import { toolCategories } from '../types/entities';
import type { Alien, Planet } from '../types/entities';

interface RequirementStatusCellProps {
  buyer: Alien;
  currentPlanet: Planet;
  categoryId: 'temperature' | 'atmosphere' | 'water' | 'gravity' | 'radiation';
}

const RequirementStatusCell: React.FC<RequirementStatusCellProps> = ({ buyer, currentPlanet, categoryId }) => {
  const cat = toolCategories.find(c => c.id === categoryId);
  let met = false;
  if (cat) {
    // Map categoryId to the correct buyer range and planet value keys
    const rangeKey = {
      temperature: 'tempRange',
      atmosphere: 'atmoRange',
      water: 'waterRange',
      gravity: 'gravRange',
      radiation: 'radRange',
    }[categoryId];
    const valueKey = categoryId;
    if (rangeKey && buyer[rangeKey] && typeof currentPlanet[valueKey] === 'number') {
      const [min, max] = buyer[rangeKey];
      const value = currentPlanet[valueKey];
      met = value >= min && value <= max;
    }
  }
  return (
    <td className="px-2 py-1 text-center">
      <span className={met ? 'text-green-400' : 'text-red-400'}>
        {met ? '✓' : '✗'}
      </span>
    </td>
  );
};

export default RequirementStatusCell;
