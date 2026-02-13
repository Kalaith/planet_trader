import React from "react";
import { useGameContext } from "../contexts/useGameContext";

const Tutorial: React.FC = () => {
  const { startGame } = useGameContext();
  // Fixed handleStartGame reference error

  return (
    <div className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-[9999] p-4">
      <div className="bg-gray-800 border border-gray-700 rounded-lg shadow-xl max-w-md w-full relative z-[10000]">
        <div className="p-6">
          <div className="border-b border-gray-700 pb-4 mb-6">
            <h3 className="text-xl font-bold text-blue-400">
              🎓 Welcome to Terraforming Co!
            </h3>
          </div>
          <div className="space-y-4">
            <p className="text-gray-300">
              Transform barren worlds into thriving planets for alien species!
            </p>
            <ol className="list-decimal list-inside space-y-2 text-gray-300 text-sm">
              <li>Buy a planet using the "Buy New Planet" button</li>
              <li>Check what alien species want (right panel)</li>
              <li>Use terraforming tools (left panel) to modify conditions</li>
              <li>Sell your terraformed planet for profit!</li>
            </ol>
            <button
              className="w-full bg-blue-600 hover:bg-blue-700 text-white py-3 px-4 rounded-lg font-semibold transition-colors duration-200"
              onClick={startGame}
            >
              Start Terraforming!
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Tutorial;
