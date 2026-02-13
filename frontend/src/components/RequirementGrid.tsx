import React from "react";
import { toolCategories } from "../types/entities";
import type { Alien, Planet } from "../types/entities";

interface RequirementGridProps {
  buyer: Alien;
  currentPlanet: Planet | null;
}

const RequirementGrid: React.FC<RequirementGridProps> = ({
  buyer,
  currentPlanet,
}) => (
  <div className="grid grid-cols-2 gap-2 text-xs">
    {currentPlanet ? (
      <>
        {toolCategories.map((cat) => {
          let stat = "";
          let range: [number, number] = [0, 0];
          let value = 0;
          let display = "";
          let yourValue = "";
          let met = false;
          switch (cat.id) {
            case "temperature":
              stat = "Temperature";
              range = buyer.tempRange || [0, 0];
              value = currentPlanet.temperature;
              display = `${cat.icon} ${range[0] || 0}°C to ${range[1] || 0}°C`;
              yourValue = `${currentPlanet.temperature.toFixed(1)}°C`;
              met = value >= range[0] && value <= range[1];
              break;
            case "atmosphere":
              stat = "Atmosphere";
              range = buyer.atmoRange || [0, 0];
              value = currentPlanet.atmosphere;
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              yourValue = `${currentPlanet.atmosphere.toFixed(2)}x`;
              met = value >= range[0] && value <= range[1];
              break;
            case "water":
              stat = "Water";
              range = buyer.waterRange || [0, 0];
              value = currentPlanet.water;
              display = `${cat.icon} ${Math.round((range[0] || 0) * 100)}% to ${Math.round((range[1] || 0) * 100)}%`;
              yourValue = `${Math.round(currentPlanet.water * 100)}%`;
              met = value >= range[0] && value <= range[1];
              break;
            case "gravity":
              stat = "Gravity";
              range = buyer.gravRange || [0, 0];
              value = currentPlanet.gravity;
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              yourValue = `${currentPlanet.gravity.toFixed(2)}x`;
              met = value >= range[0] && value <= range[1];
              break;
            case "radiation":
              stat = "Radiation";
              range = buyer.radRange || [0, 0];
              value = currentPlanet.radiation;
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              yourValue = `${currentPlanet.radiation.toFixed(2)}x`;
              met = value >= range[0] && value <= range[1];
              break;
            default:
              return null;
          }
          return (
            <div
              key={cat.id}
              className={`rounded p-2 ${
                met
                  ? "bg-green-900/30 border border-green-600/50"
                  : "bg-red-900/30 border border-red-600/50"
              }`}
            >
              <div className="flex items-center gap-1">
                <span>{met ? "✅" : "❌"}</span>
                <span className="text-gray-400">{stat}</span>
              </div>
              <div className="text-white">{display}</div>
              <div className="text-xs text-blue-400">
                Your planet: {yourValue}
              </div>
            </div>
          );
        })}
      </>
    ) : (
      <>
        {toolCategories.map((cat) => {
          let stat = "";
          let range: [number, number] = [0, 0];
          let display = "";
          switch (cat.id) {
            case "temperature":
              stat = "Temperature";
              range = buyer.tempRange || [0, 0];
              display = `${cat.icon} ${range[0] || 0}°C to ${range[1] || 0}°C`;
              break;
            case "atmosphere":
              stat = "Atmosphere";
              range = buyer.atmoRange || [0, 0];
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              break;
            case "water":
              stat = "Water";
              range = buyer.waterRange || [0, 0];
              display = `${cat.icon} ${Math.round((range[0] || 0) * 100)}% to ${Math.round((range[1] || 0) * 100)}%`;
              break;
            case "gravity":
              stat = "Gravity";
              range = buyer.gravRange || [0, 0];
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              break;
            case "radiation":
              stat = "Radiation";
              range = buyer.radRange || [0, 0];
              display = `${cat.icon} ${range[0] || 0}x to ${range[1] || 0}x`;
              break;
            default:
              return null;
          }
          return (
            <div key={cat.id} className="bg-gray-600 rounded p-2">
              <div className="text-gray-400">{stat}</div>
              <div className="text-white">{display}</div>
            </div>
          );
        })}
      </>
    )}
  </div>
);

export default RequirementGrid;
