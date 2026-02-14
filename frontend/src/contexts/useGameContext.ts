import { useContext } from 'react';
import { GameContext } from './gameContext';
import type { GameContextType } from '../types/GameContextTypes';

export const useGameContext = (): GameContextType => {
  const context = useContext(GameContext);
  if (context === undefined) {
    throw new Error('useGameContext must be used within a GameProvider');
  }
  return context;
};
