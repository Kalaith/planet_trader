import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import { fetchGameData } from '../api/fetchGameData';
import type { GameContextType, StatusMessage } from '../types/GameContextTypes';
import type { Planet, PlanetType, Species, Tool } from '../types/entities';

interface GameDataState {
  planetTypes: PlanetType[];
  alienSpecies: Species[];
  terraformingTools: Tool[];
  planetNames: string[];
}

interface GameStoreState extends GameContextType {
  alienSpeciesTypes: Species[];
  isGameDataLoaded: boolean;
  initializeGameData: () => Promise<void>;
  refreshBuyers: () => void;
  removeMessage: (id: string) => void;
}

const unlockedResearch: string[] = [];

const randomItem = <T>(arr: readonly T[]): T => arr[Math.floor(Math.random() * arr.length)];

const defaultGameData: GameDataState = {
  planetTypes: [],
  alienSpecies: [],
  terraformingTools: [],
  planetNames: [],
};

const createMessage = (msg: string, type: StatusMessage['type'] = 'info'): StatusMessage => ({
  id: `${type}-${Date.now()}-${Math.random().toString(36).slice(2)}`,
  msg,
  type,
});

const generateRandomSpecies = (types: Species[], count: number): Species[] => {
  if (types.length === 0) {
    return [];
  }

  if (count >= types.length) {
    return [...types];
  }

  const shuffled = [...types];
  for (let i = shuffled.length - 1; i > 0; i -= 1) {
    const j = Math.floor(Math.random() * (i + 1));
    [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
  }

  return shuffled.slice(0, count);
};

const getRandomPlanetName = (names: string[]): string => {
  const fallbackNames = ['Terra', 'Nova', 'Aether', 'Zion', 'Eden'];
  return randomItem(names.length > 0 ? names : fallbackNames);
};

const createPlanet = (type: PlanetType, name: string): Planet => ({
  id: `${type.name}-${Date.now()}`,
  type,
  name,
  temperature: type.baseTemp + (Math.random() - 0.5) * 20,
  atmosphere: Math.max(0, type.baseAtmo + (Math.random() - 0.5) * 0.4),
  water: Math.max(0, Math.min(1, type.baseWater + (Math.random() - 0.5) * 0.3)),
  gravity: Math.max(0.1, type.baseGrav + (Math.random() - 0.5) * 0.4),
  radiation: Math.max(0, type.baseRad + (Math.random() - 0.5) * 0.3),
  purchasePrice: Math.floor(1000 + Math.random() * 2000),
  color: type.color,
});

const applyPlanetEffect = (planet: Planet, stat: string, delta: number): Planet => {
  const updated = { ...planet };
  if (stat === 'temperature') {
    updated.temperature = Math.max(-100, Math.min(200, updated.temperature + delta));
  }
  if (stat === 'atmosphere') {
    updated.atmosphere = Math.max(0, Math.min(3, updated.atmosphere + delta));
  }
  if (stat === 'water') {
    updated.water = Math.max(0, Math.min(1, updated.water + delta));
  }
  if (stat === 'gravity') {
    updated.gravity = Math.max(0.1, Math.min(5, updated.gravity + delta));
  }
  if (stat === 'radiation') {
    updated.radiation = Math.max(0, Math.min(2, updated.radiation + delta));
  }

  return updated;
};

export const useGameStore = create<GameStoreState>()(
  persist(
    (set, get) => ({
      credits: 10000,
      alienBuyers: [],
      messages: [],
      planetOptions: [],
      planets: [],
      currentPlanet: null,
      gameStarted: false,
      gameData: defaultGameData,
      alienSpeciesTypes: [],
      planetModalOpen: false,
      isGameDataLoaded: false,

      initializeGameData: async () => {
        if (get().isGameDataLoaded) {
          return;
        }

        const gameDataResponse = await fetchGameData();
        const data = gameDataResponse.data;
        if (!data) {
          throw new Error('No game data returned');
        }

        set({
          gameData: {
            planetTypes: data.planetTypes,
            alienSpecies: data.alienSpecies,
            terraformingTools: data.terraformingTools,
            planetNames: data.planetNames,
          },
          alienSpeciesTypes: data.alienSpeciesTypes,
          isGameDataLoaded: true,
        });
        get().refreshBuyers();
      },

      refreshBuyers: () => {
        const uniqueSpecies = generateRandomSpecies(get().alienSpeciesTypes, 4);
        set({
          alienBuyers: uniqueSpecies.map((species, index) => ({
            ...species,
            id: Date.now() + index,
            currentPrice: species.basePrice + Math.floor(Math.random() * 500) - 250,
          })),
        });
      },

      removeMessage: id => {
        set(state => ({
          messages: state.messages.filter(message => message.id !== id),
        }));
      },

      showPlanetPurchaseModal: () => {
        const { gameStarted, gameData } = get();
        if (!gameStarted) {
          set(state => ({ messages: [...state.messages, createMessage('Complete the tutorial first!', 'error')] }));
          return;
        }

        if (gameData.planetTypes.length === 0) {
          set(state => ({ messages: [...state.messages, createMessage('Planet catalogue is still loading.', 'error')] }));
          return;
        }

        const options = Array.from({ length: 3 + Math.floor(Math.random() * 2) }, () => {
          const planetType = randomItem(gameData.planetTypes);
          return createPlanet(planetType, getRandomPlanetName(gameData.planetNames));
        });
        set({ planetOptions: options, planetModalOpen: true });
      },

      closePlanetModal: () => {
        set({ planetModalOpen: false });
      },

      purchasePlanet: planet => {
        set(state => ({
          credits: state.credits - planet.purchasePrice,
          planets: [...state.planets, planet],
          messages: [
            ...state.messages,
            createMessage(`Purchased ${planet.name} for ${planet.purchasePrice} credits`, 'success'),
          ],
          planetModalOpen: false,
        }));
      },

      selectPlanet: planet => {
        set(state => ({
          currentPlanet: planet,
          messages: [...state.messages, createMessage(`Selected ${planet.name}`, 'info')],
        }));
      },

      sellPlanet: buyer => {
        const currentPlanet = get().currentPlanet;
        if (!currentPlanet) {
          set(state => ({
            messages: [...state.messages, createMessage('No planet selected to sell.', 'error')],
          }));
          return;
        }

        const price = buyer.currentPrice;
        set(state => ({
          credits: state.credits + price,
          planets: state.planets.filter(planet => planet.id !== currentPlanet.id),
          currentPlanet: null,
          messages: [
            ...state.messages,
            createMessage(`Sold ${currentPlanet.name} to ${buyer.name} for ${price} credits`, 'success'),
          ],
        }));
      },

      startGame: () => {
        set(state => ({
          gameStarted: true,
          messages: [...state.messages, createMessage('Game started! Begin terraforming planets.', 'info')],
        }));
      },

      applyTool: tool => {
        const { currentPlanet, credits } = get();
        if (!currentPlanet) {
          set(state => ({ messages: [...state.messages, createMessage('Select a planet first!', 'error')] }));
          return;
        }

        if (credits < tool.cost) {
          set(state => ({ messages: [...state.messages, createMessage('Not enough credits!', 'error')] }));
          return;
        }

        let updated = { ...currentPlanet };
        Object.entries(tool.effect ?? {}).forEach(([stat, delta]) => {
          updated = applyPlanetEffect(updated, stat, delta);
        });
        Object.entries(tool.sideEffects ?? {}).forEach(([stat, delta]) => {
          updated = applyPlanetEffect(updated, stat, delta);
        });

        set(state => ({
          credits: state.credits - tool.cost,
          currentPlanet: updated,
          planets: state.planets.map(planet => (planet.id === updated.id ? updated : planet)),
          messages: [...state.messages, createMessage(`Used ${tool.name}`, 'success')],
        }));
      },

      isToolLocked: tool => Boolean(tool.upgradeRequired && !unlockedResearch.includes(tool.upgradeRequired)),
    }),
    {
      name: 'planet-trader-game',
      storage: createJSONStorage(() => localStorage),
      partialize: state => ({
        credits: state.credits,
        planets: state.planets,
        currentPlanet: state.currentPlanet,
        gameStarted: state.gameStarted,
      }),
    }
  )
);
