import { useGameContext } from './contexts/useGameContext';
import ResourcesPanel from './components/ResourcesPanel';
import TerraformingToolsPanel from './components/TerraformingToolsPanel';
import PlanetWorkshop from './components/PlanetWorkshop';
import AlienMarketPanel from './components/AlienMarketPanel';
import PlanetInventory from './components/PlanetInventory';
import GameMessages from './components/GameMessages';
import PlanetPurchaseModal from './components/PlanetPurchaseModal';
import Tutorial from './components/Tutorial';

const GameContainer = () => {
  const { gameStarted } = useGameContext();

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Mobile-optimized container with safe areas */}
      <div className="container mx-auto px-2 sm:px-4 lg:px-6 py-2 sm:py-4 lg:py-6 pb-safe">
        {/* Mobile-first responsive layout */}
        <div className="flex flex-col space-y-3 sm:space-y-4 lg:grid lg:grid-cols-12 lg:gap-4 xl:gap-6 lg:space-y-0">
          {/* Resources Panel - Full width header */}
          <div className="lg:col-span-12">
            <ResourcesPanel />
          </div>

          {/* Mobile: Stack all panels vertically */}
          {/* Desktop: 3-column layout with terraforming tools on left, workshop/inventory in center, market on right */}

          {/* Left Panel - Terraforming Tools */}
          <div className="lg:col-span-3 xl:col-span-3">
            <TerraformingToolsPanel />
          </div>

          {/* Center Panel - Main workspace */}
          <div className="space-y-3 sm:space-y-4 lg:col-span-6 xl:col-span-6">
            <PlanetWorkshop />
            <PlanetInventory />
          </div>

          {/* Right Panel - Alien Market */}
          <div className="lg:col-span-3 xl:col-span-3">
            <AlienMarketPanel />
          </div>
        </div>

        {/* Fixed position elements */}
        <GameMessages />
        <PlanetPurchaseModal />
        {!gameStarted && <Tutorial />}
      </div>
    </div>
  );
};

export default GameContainer;
