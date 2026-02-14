import type { Alien, Planet, PlanetType, Species, Tool } from '../types/entities';

// Define StatusMessage type
export type StatusMessage = {
  id: string;
  msg: string;
  type: 'info' | 'success' | 'error';
};

// Create the context type based on what useGame returns
export interface GameContextType {
  messages: StatusMessage[];
  gameStarted: boolean;
  alienBuyers: Alien[];
  planets: Planet[];
  currentPlanet: Planet | null;
  planetModalOpen: boolean;
  planetOptions: Planet[];
  credits: number;
  gameData: {
    planetTypes: PlanetType[];
    alienSpecies: Species[];
    terraformingTools: Tool[];
    planetNames: string[];
  };
  applyTool: (tool: Tool) => void;
  isToolLocked: (tool: Tool) => boolean;
  showPlanetPurchaseModal: () => void;
  closePlanetModal: () => void;
  purchasePlanet: (planet: Planet) => void;
  selectPlanet: (planet: Planet) => void;
  sellPlanet: (buyer: Alien) => void;
  startGame: () => void;
}
