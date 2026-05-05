import type { GameContextType } from '../types/GameContextTypes';
import { useGame } from '../game/useGame';

export const useGameContext = (): GameContextType => {
  return useGame();
};
