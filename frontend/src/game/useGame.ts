import { useState, useEffect, useCallback } from "react";
import type {
  Planet,
  PlanetType,
  Tool,
  Species,
  Alien,
} from "../types/entities";
// Removed unused imports
// import { hexToRgb, rgbToHsl, hslToRgb, rgbToHex } from '../utils/colorUtils';
import { fetchGameData } from "../api/fetchGameData";

// Utility functions
export const randomItem = <T>(arr: readonly T[]): T =>
  arr[Math.floor(Math.random() * arr.length)];

// Planet class logic as a factory function
function createPlanet(type: PlanetType, name: string): Planet {
  return {
    id: `${type.name}-${Date.now()}`, // Added id property
    type,
    name,
    temperature: type.baseTemp + (Math.random() - 0.5) * 20,
    atmosphere: Math.max(0, type.baseAtmo + (Math.random() - 0.5) * 0.4),
    water: Math.max(
      0,
      Math.min(1, type.baseWater + (Math.random() - 0.5) * 0.3),
    ),
    gravity: Math.max(0.1, type.baseGrav + (Math.random() - 0.5) * 0.4),
    radiation: Math.max(0, type.baseRad + (Math.random() - 0.5) * 0.3),
    purchasePrice: Math.floor(1000 + Math.random() * 2000),
    color: type.color, // Ensure color is assigned
  };
}

// Alien species generator - ensures unique species
function generateRandomSpecies(types: Species[], count: number): Species[] {
  if (types.length === 0) return [];

  // If we need more species than available, just return all available species
  if (count >= types.length) {
    return [...types];
  }

  // Create a copy of the array and shuffle it to get random unique selections
  const shuffled = [...types];
  for (let i = shuffled.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
  }

  // Return the first 'count' items from the shuffled array
  return shuffled.slice(0, count);
}

// Reserved for future: track unique planet names across the session.

export function useGame() {
  // Game state
  const [credits, setCredits] = useState(10000);
  const [alienBuyers, setAlienBuyers] = useState<Alien[]>([]);
  const [messages, setMessages] = useState<
    { id: string; msg: string; type: "info" | "success" | "error" }[]
  >([]);
  const [planetOptions, setPlanetOptions] = useState<Planet[]>([]);
  const [planets, setPlanets] = useState<Planet[]>([]);
  const [currentPlanet, setCurrentPlanet] = useState<Planet | null>(null);
  const [gameStarted, setGameStarted] = useState(false);
  const [gameData, setGameData] = useState({
    planetTypes: [],
    alienSpecies: [],
    terraformingTools: [] as Tool[],
    planetNames: [],
  });
  const [alienSpeciesTypes, setAlienSpeciesTypes] = useState<Species[]>([]);
  const [planetModalOpen, setPlanetModalOpen] = useState(false);

  // Load game data from /api endpoints
  useEffect(() => {
    fetchGameData()
      .then(
        ({
          planetTypes,
          alienSpecies,
          terraformingTools,
          planetNames,
          alienSpeciesTypes,
        }) => {
          setGameData({
            planetTypes,
            alienSpecies,
            terraformingTools,
            planetNames,
          }); // Removed researchList
          setAlienSpeciesTypes(alienSpeciesTypes);
        },
      )
      .catch((err) => console.error("Failed to load game data:", err));
  }, []);

  // Alien buyers refresh
  useEffect(() => {
    if (!alienSpeciesTypes.length) return;
    function refreshBuyers() {
      const uniqueSpecies = generateRandomSpecies(alienSpeciesTypes, 4);
      setAlienBuyers(
        uniqueSpecies.map((s, i) => ({
          ...s,
          id: Date.now() + i,
          currentPrice: s.basePrice + Math.floor(Math.random() * 500) - 250, // Add some price variation
        })) as Alien[],
      );
    }
    refreshBuyers();
    const interval = setInterval(refreshBuyers, 30000);
    return () => clearInterval(interval);
  }, [alienSpeciesTypes]);

  // Game actions
  const spendCredits = useCallback(
    (amount: number): boolean => {
      if (credits < amount) return false;
      setCredits((c) => c - amount);
      return true;
    },
    [credits],
  );
  // Show message with auto-remove
  const showMessage = useCallback(
    (msg: string, type: "info" | "success" | "error" = "info"): void => {
      const id = `${type}-${Date.now()}`;
      console.log(`Adding message with id: ${id}`); // Log when message is added
      setMessages((prevMessages) => [...prevMessages, { id, msg, type }]);
    },
    [],
  );

  useEffect(() => {
    const timeoutIds: number[] = []; // Changed type to number for browser compatibility

    messages.forEach((message) => {
      const timeoutId = setTimeout(() => {
        console.log(`Attempting to remove message with id: ${message.id}`); // Log before attempting removal
        setMessages((prevMessages) => {
          const updatedMessages = prevMessages.filter(
            (msg) => msg.id !== message.id,
          );
          console.log("Updated messages after removal:", updatedMessages); // Log updated state
          return updatedMessages;
        });
      }, 3000);

      timeoutIds.push(timeoutId);
    });

    return () => {
      timeoutIds.forEach((timeoutId) => clearTimeout(timeoutId));
    };
  }, [messages]);

  // Planet purchase modal logic
  const showPlanetPurchaseModal = useCallback(() => {
    if (!gameStarted) {
      showMessage("Complete the tutorial first!", "error");
      return;
    }
    // Generate planet options
    const options = [];
    for (let i = 0, n = 3 + Math.floor(Math.random() * 2); i < n; i++) {
      const planetType = randomItem(gameData.planetTypes);
      const name = getRandomPlanetName(); // Removed arguments
      const planet = createPlanet(planetType, name);
      options.push(planet);
    }
    setPlanetOptions(options);
    setPlanetModalOpen(true);
  }, [gameStarted, gameData, showMessage]);

  const closePlanetModal = useCallback(() => {
    setPlanetModalOpen(false);
  }, []);

  // Purchase a planet
  const purchasePlanet = useCallback(
    (planet: Planet): void => {
      setCredits((prevCredits) => prevCredits - planet.purchasePrice);
      setPlanets((prevPlanets) => [...prevPlanets, planet]);
      setMessages((prevMessages) => [
        ...prevMessages,
        {
          id: `success-${Date.now()}`,
          msg: `Purchased ${planet.name} for ${planet.purchasePrice}₵`,
          type: "success",
        },
      ]);
      closePlanetModal();
    },
    [closePlanetModal],
  );

  // Select a planet
  const selectPlanet = useCallback((planet: Planet): void => {
    setCurrentPlanet(planet);
    setMessages((prevMessages) => [
      ...prevMessages,
      {
        id: `info-${Date.now()}`,
        msg: `Selected ${planet.name}`,
        type: "info",
      },
    ]);
  }, []);

  // Sell planet to buyer
  const sellPlanet = useCallback(
    (buyer: Alien): void => {
      if (currentPlanet) {
        const price = buyer.currentPrice;
        setCredits((prevCredits) => prevCredits + price);
        setPlanets((prevPlanets) =>
          prevPlanets.filter((planet) => planet !== currentPlanet),
        );
        setCurrentPlanet(null);
        setMessages((prevMessages) => [
          ...prevMessages,
          {
            id: `success-${Date.now()}`,
            msg: `Sold ${currentPlanet.name} to ${buyer.name} for ${price}₵`,
            type: "success",
          },
        ]);
      } else {
        setMessages((prevMessages) => [
          ...prevMessages,
          {
            id: `error-${Date.now()}`,
            msg: "No planet selected to sell.",
            type: "error",
          },
        ]);
      }
    },
    [currentPlanet],
  );

  // Apply a terraforming tool to the current planet
  const applyTool = useCallback(
    (tool: Tool): void => {
      if (!currentPlanet) {
        showMessage("Select a planet first!", "error");
        return;
      }
      if (!spendCredits(tool.cost)) {
        showMessage("Not enough credits!", "error");
        return;
      }
      // Apply tool effects
      const apply = (planet: Planet, stat: string, delta: number): void => {
        if (stat === "temperature")
          planet.temperature = Math.max(
            -100,
            Math.min(200, (planet.temperature || 0) + delta),
          );
        if (stat === "atmosphere")
          planet.atmosphere = Math.max(
            0,
            Math.min(3, (planet.atmosphere || 0) + delta),
          );
        if (stat === "water")
          planet.water = Math.max(0, Math.min(1, (planet.water || 0) + delta));
        if (stat === "gravity")
          planet.gravity = Math.max(
            0.1,
            Math.min(5, (planet.gravity || 0) + delta),
          );
        if (stat === "radiation")
          planet.radiation = Math.max(
            0,
            Math.min(2, (planet.radiation || 0) + delta),
          );
      };
      setCurrentPlanet((planet: Planet | null): Planet | null => {
        if (!planet) return planet;
        const updated = { ...planet };
        if (tool.effect) {
          Object.entries(tool.effect).forEach(([stat, delta]) =>
            apply(updated, stat, delta),
          );
        }
        if (tool.sideEffects) {
          Object.entries(tool.sideEffects).forEach(([stat, delta]) =>
            apply(updated, stat, delta),
          );
        }
        return updated;
      });
      showMessage(`Used ${tool.name}`, "success");
    },
    [currentPlanet, spendCredits, showMessage],
  );

  // Tool locking: a tool is locked if it has upgradeRequired and it's not in unlockedResearch
  const isToolLocked = (tool: Tool): boolean => {
    if (
      tool.upgradeRequired &&
      !unlockedResearch.includes(tool.upgradeRequired)
    )
      return true;
    return false;
  };

  const startGame = useCallback((): void => {
    setGameStarted(true);
    setMessages((prevMessages) => [
      ...prevMessages,
      {
        id: `info-${Date.now()}`,
        msg: "Game started! Begin terraforming planets.",
        type: "info",
      },
    ]);
  }, []);

  return {
    credits,
    alienBuyers,
    messages,
    planetOptions,
    planets,
    currentPlanet,
    gameStarted,
    gameData,
    planetModalOpen,
    // ...other actions...
    sellPlanet,
    showPlanetPurchaseModal,
    selectPlanet,
    closePlanetModal,
    purchasePlanet,
    startGame,
    applyTool,
    isToolLocked,
  };
}

// Added missing function definition
function getRandomPlanetName(): string {
  const names = ["Terra", "Nova", "Aether", "Zion", "Eden"];
  return names[Math.floor(Math.random() * names.length)];
}

// Defined missing variable
const unlockedResearch: string[] = []; // Placeholder array for unlocked research
