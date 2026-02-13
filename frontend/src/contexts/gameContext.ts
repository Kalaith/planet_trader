import { createContext } from "react";
import type { GameContextType } from "../types/GameContextTypes";

export const GameContext = createContext<GameContextType | undefined>(
  undefined,
);
