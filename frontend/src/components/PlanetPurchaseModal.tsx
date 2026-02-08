import React, { useRef } from 'react';
import { useGameContext } from '../contexts/useGameContext';

const PlanetPurchaseModal: React.FC = () => {
  const { planetModalOpen, planetOptions, closePlanetModal, purchasePlanet } = useGameContext();
  const modalRef = useRef<HTMLDivElement>(null);

  const handleBackdropClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.target === modalRef.current) {
      closePlanetModal();
    }
  };

  return planetModalOpen ? (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-2 sm:p-4" ref={modalRef} onClick={handleBackdropClick}>
      <div className="bg-gray-800 border border-gray-700 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] sm:max-h-[80vh] overflow-y-auto">
        <div className="p-3 sm:p-6">
          <div className="flex justify-between items-center border-b border-gray-700 pb-3 sm:pb-4 mb-3 sm:mb-6">
            <h3 className="text-lg sm:text-xl font-bold text-blue-400">🛒 Buy New Planet</h3>
            <button 
              className="bg-gray-600 hover:bg-gray-500 text-white px-2 sm:px-3 py-1 rounded-lg text-sm transition-colors duration-200"
              onClick={closePlanetModal}
            >
              ✕
            </button>
          </div>
          <div className="space-y-3 sm:space-y-4">
            {planetOptions.map(opt => (
              <div key={opt.id || opt.name} className="bg-gray-700 border border-gray-600 rounded-lg p-3 sm:p-4">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
                  <div className="flex-1 w-full sm:w-auto">
                    <div className="font-bold text-white text-base sm:text-lg mb-2">{opt.name}</div>
                    <div className="text-sm text-gray-300 mb-2">Type: {opt.type?.name}</div>
                    <div className="grid grid-cols-2 sm:grid-cols-3 gap-1 sm:gap-2 text-xs text-gray-400">
                      <div>🌡️ {Math.round(opt.temperature)}°C</div>
                      <div>🌫️ {opt.atmosphere.toFixed(1)}x</div>
                      <div>💧 {Math.round(opt.water * 100)}%</div>
                      <div>⚖️ {opt.gravity.toFixed(1)}x</div>
                      <div>☢️ {opt.radiation.toFixed(1)}x</div>
                    </div>
                  </div>
                  <div className="text-center w-full sm:w-auto sm:ml-4">
                    <div className="text-green-400 font-bold text-lg mb-2">{opt.purchasePrice.toLocaleString()}₵</div>
                    <button 
                      className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-semibold transition-colors duration-200 w-full sm:w-auto"
                      onClick={() => purchasePlanet(opt)}
                    >
                      Buy
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  ) : null;
};

export default PlanetPurchaseModal;
