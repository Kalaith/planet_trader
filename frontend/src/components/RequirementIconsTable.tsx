import React from 'react';
import { toolCategories } from '../types/entities';
import type { Alien, Planet } from '../types/entities';

interface RequirementIconsTableProps {
  buyer: Alien;
  currentPlanet: Planet;
}

const RequirementIconsTable: React.FC<RequirementIconsTableProps> = ({ buyer, currentPlanet }) => (
  <div className="inline-block border border-gray-600 rounded bg-gray-700/50">
    <table className="text-sm">
      <tbody>
        <tr>
          {toolCategories.map(cat => (
            <td key={cat.id} className={`px-2 py-1 text-center ${cat.colorClass || ''}`}>
              {cat.icon}
            </td>
          ))}
        </tr>
        <tr>
          <td className="px-2 py-1 text-center">
            <span
              className={
                buyer.tempRange &&
                currentPlanet.temperature >= buyer.tempRange[0] &&
                currentPlanet.temperature <= buyer.tempRange[1]
                  ? 'text-green-400'
                  : 'text-red-400'
              }
            >
              {buyer.tempRange &&
              currentPlanet.temperature >= buyer.tempRange[0] &&
              currentPlanet.temperature <= buyer.tempRange[1]
                ? '✓'
                : '✗'}
            </span>
          </td>
          <td className="px-2 py-1 text-center">
            <span
              className={
                buyer.atmoRange &&
                currentPlanet.atmosphere >= buyer.atmoRange[0] &&
                currentPlanet.atmosphere <= buyer.atmoRange[1]
                  ? 'text-green-400'
                  : 'text-red-400'
              }
            >
              {buyer.atmoRange &&
              currentPlanet.atmosphere >= buyer.atmoRange[0] &&
              currentPlanet.atmosphere <= buyer.atmoRange[1]
                ? '✓'
                : '✗'}
            </span>
          </td>
          <td className="px-2 py-1 text-center">
            <span
              className={
                buyer.waterRange &&
                currentPlanet.water >= buyer.waterRange[0] &&
                currentPlanet.water <= buyer.waterRange[1]
                  ? 'text-green-400'
                  : 'text-red-400'
              }
            >
              {buyer.waterRange &&
              currentPlanet.water >= buyer.waterRange[0] &&
              currentPlanet.water <= buyer.waterRange[1]
                ? '✓'
                : '✗'}
            </span>
          </td>
          <td className="px-2 py-1 text-center">
            <span
              className={
                buyer.gravRange &&
                currentPlanet.gravity >= buyer.gravRange[0] &&
                currentPlanet.gravity <= buyer.gravRange[1]
                  ? 'text-green-400'
                  : 'text-red-400'
              }
            >
              {buyer.gravRange &&
              currentPlanet.gravity >= buyer.gravRange[0] &&
              currentPlanet.gravity <= buyer.gravRange[1]
                ? '✓'
                : '✗'}
            </span>
          </td>
          <td className="px-2 py-1 text-center">
            <span
              className={
                buyer.radRange &&
                currentPlanet.radiation >= buyer.radRange[0] &&
                currentPlanet.radiation <= buyer.radRange[1]
                  ? 'text-green-400'
                  : 'text-red-400'
              }
            >
              {buyer.radRange &&
              currentPlanet.radiation >= buyer.radRange[0] &&
              currentPlanet.radiation <= buyer.radRange[1]
                ? '✓'
                : '✗'}
            </span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
);

export default RequirementIconsTable;
