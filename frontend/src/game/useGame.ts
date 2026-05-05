import { useEffect } from 'react';
import { useGameStore } from '../stores/gameStore';

export const randomItem = <T>(arr: readonly T[]): T => arr[Math.floor(Math.random() * arr.length)];

export function useGame() {
  return useGameStore();
}

export function useGameEffects(): void {
  const initializeGameData = useGameStore(state => state.initializeGameData);
  const refreshBuyers = useGameStore(state => state.refreshBuyers);
  const isGameDataLoaded = useGameStore(state => state.isGameDataLoaded);
  const messages = useGameStore(state => state.messages);
  const removeMessage = useGameStore(state => state.removeMessage);

  useEffect(() => {
    initializeGameData().catch(error => {
      console.error('Failed to load game data:', error);
    });
  }, [initializeGameData]);

  useEffect(() => {
    if (!isGameDataLoaded) {
      return undefined;
    }

    const interval = window.setInterval(refreshBuyers, 30000);
    return () => window.clearInterval(interval);
  }, [isGameDataLoaded, refreshBuyers]);

  useEffect(() => {
    const timeoutIds = messages.map(message =>
      window.setTimeout(() => removeMessage(message.id), 3000)
    );

    return () => timeoutIds.forEach(timeoutId => window.clearTimeout(timeoutId));
  }, [messages, removeMessage]);
}
