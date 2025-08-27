import React, { createContext, useContext } from 'react';
import type { ReactNode } from 'react';
import { useGame } from '../game/useGame';
import type { GameContextType } from '../types/GameContextTypes';

const GameContext = createContext<GameContextType | undefined>(undefined);

interface GameProviderProps {
  children: ReactNode;
}

export const GameProvider: React.FC<GameProviderProps> = ({ children }) => {
  const gameState = useGame();

  return (
    <GameContext.Provider value={gameState}>
      {children}
    </GameContext.Provider>
  );
};

export const useGameContext = (): GameContextType => {
  const context = useContext(GameContext);
  if (context === undefined) {
    throw new Error('useGameContext must be used within a GameProvider');
  }
  return context;
};
