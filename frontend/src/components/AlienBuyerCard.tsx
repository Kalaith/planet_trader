import React from "react";
import { useGameContext } from "../contexts/useGameContext";
import { toolCategories } from "../types/entities";
import type { Alien, Planet } from "../types/entities";

type PlanetStatCategoryId =
  | "temperature"
  | "atmosphere"
  | "water"
  | "gravity"
  | "radiation";
type AlienRangeKey =
  | "tempRange"
  | "atmoRange"
  | "waterRange"
  | "gravRange"
  | "radRange";

const rangeKeyByCategory: Record<PlanetStatCategoryId, AlienRangeKey> = {
  temperature: "tempRange",
  atmosphere: "atmoRange",
  water: "waterRange",
  gravity: "gravRange",
  radiation: "radRange",
};

const planetStatCategories = toolCategories.filter(
  (c): c is (typeof toolCategories)[number] & { id: PlanetStatCategoryId } =>
    (
      ["temperature", "atmosphere", "water", "gravity", "radiation"] as const
    ).includes(c.id as PlanetStatCategoryId),
);

interface CompactAlienBuyerCardProps {
  buyer: Alien;
  compatibility: number;
  currentPlanet: Planet;
  toggleAlienDetails: (alienId: number) => void;
  sellPlanet: (buyer: Alien) => void;
}

const CompactAlienBuyerCard: React.FC<CompactAlienBuyerCardProps> = ({
  buyer,
  compatibility,
  currentPlanet,
  toggleAlienDetails,
  sellPlanet,
}) => (
  <div
    className="p-3 cursor-pointer hover:bg-gray-600 transition-colors duration-200"
    onClick={() => toggleAlienDetails(buyer.id)}
  >
    <div className="flex items-center justify-between">
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <span className="font-bold text-white text-sm">{buyer.name}</span>
          <span
            className={`text-xs font-medium ${
              compatibility >= 0.8
                ? "text-green-400"
                : compatibility >= 0.6
                  ? "text-yellow-400"
                  : "text-red-400"
            }`}
          >
            {Math.round(compatibility * 100)}%
          </span>
        </div>
        <table className="text-xs">
          <thead>
            <tr>
              {planetStatCategories.map((category) => (
                <th key={category.id} className="px-2 py-1 text-center">
                  {category.icon}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            <tr>
              {planetStatCategories.map((category) => {
                const rangeKey = rangeKeyByCategory[category.id];

                const range = buyer[rangeKey] ?? [0, 0];
                const value = currentPlanet[category.id];
                const met = value >= range[0] && value <= range[1];

                return (
                  <td
                    key={category.id}
                    className={`px-2 py-1 text-center ${met ? "text-green-400" : "text-red-400"}`}
                  >
                    {met ? "✓" : "✗"}
                  </td>
                );
              })}
            </tr>
          </tbody>
        </table>
      </div>
      <div className="flex items-center gap-2 ml-2">
        <button
          className={`py-1 px-2 rounded text-sm font-semibold ${
            compatibility >= 0.6
              ? "bg-green-600 hover:bg-green-700 text-white"
              : "bg-gray-600 text-gray-400 cursor-not-allowed"
          }`}
          onClick={(e) => {
            e.stopPropagation();
            if (compatibility >= 0.6) sellPlanet(buyer);
          }}
          disabled={compatibility < 0.6}
          type="button"
        >
          Sell
        </button>
      </div>
    </div>
  </div>
);

interface ExpandedAlienBuyerCardProps {
  buyer: Alien;
  currentPlanet: Planet;
}

const ExpandedAlienBuyerCard: React.FC<ExpandedAlienBuyerCardProps> = ({
  buyer,
  currentPlanet,
}) => (
  <div className="px-3 pb-3 border-t border-gray-600">
    <div className="pt-3 space-y-3">
      <div className="text-gray-300 text-sm leading-relaxed">
        {buyer.description}
      </div>
      <div className="grid grid-cols-2 gap-2 text-xs">
        {planetStatCategories.map((category) => {
          const rangeKey = rangeKeyByCategory[category.id];

          const range = buyer[rangeKey] ?? [0, 0];
          const value = currentPlanet[category.id];
          const met = value >= range[0] && value <= range[1];

          return (
            <div
              key={category.id}
              className={`rounded p-2 border ${
                met
                  ? "bg-green-900/30 border-green-500"
                  : "bg-red-900/30 border-red-500"
              }`}
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

interface AlienBuyerCardProps {
  buyer: Alien;
  isExpanded: boolean;
  toggleAlienDetails: (alienId: number) => void;
}

const AlienBuyerCard: React.FC<AlienBuyerCardProps> = ({
  buyer,
  isExpanded,
  toggleAlienDetails,
}) => {
  const { currentPlanet, sellPlanet } = useGameContext();

  if (!currentPlanet) return null;

  const calculateCompatibility = (b: Alien) => {
    const matches = {
      temperature:
        currentPlanet.temperature >= (b.tempRange?.[0] ?? 0) &&
        currentPlanet.temperature <= (b.tempRange?.[1] ?? 0),
      atmosphere:
        currentPlanet.atmosphere >= (b.atmoRange?.[0] ?? 0) &&
        currentPlanet.atmosphere <= (b.atmoRange?.[1] ?? 0),
      water:
        currentPlanet.water >= (b.waterRange?.[0] ?? 0) &&
        currentPlanet.water <= (b.waterRange?.[1] ?? 0),
      gravity:
        currentPlanet.gravity >= (b.gravRange?.[0] ?? 0) &&
        currentPlanet.gravity <= (b.gravRange?.[1] ?? 0),
      radiation:
        currentPlanet.radiation >= (b.radRange?.[0] ?? 0) &&
        currentPlanet.radiation <= (b.radRange?.[1] ?? 0),
    };

    const score = Object.values(matches).filter(Boolean).length;
    const compatibility = score / 5;
    return { compatibility };
  };

  const { compatibility } = calculateCompatibility(buyer);

  return (
    <div className="bg-gray-700 border border-gray-600 rounded-lg overflow-hidden">
      <CompactAlienBuyerCard
        buyer={buyer}
        compatibility={compatibility}
        currentPlanet={currentPlanet}
        toggleAlienDetails={toggleAlienDetails}
        sellPlanet={sellPlanet}
      />
      {isExpanded && (
        <ExpandedAlienBuyerCard buyer={buyer} currentPlanet={currentPlanet} />
      )}
    </div>
  );
};

export default AlienBuyerCard;
