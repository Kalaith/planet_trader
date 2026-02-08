import React, { type ReactNode } from 'react';
import { useGame } from '../game/useGame';
import { GameContext } from './gameContext';

interface GameProviderProps {
  children: ReactNode;
}

export const GameProvider: React.FC<GameProviderProps> = ({ children }) => {
  const gameState = useGame();

  return <GameContext.Provider value={gameState}>{children}</GameContext.Provider>;
};

