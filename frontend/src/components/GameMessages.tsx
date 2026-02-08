import React, { useEffect } from 'react';
import { useGameContext } from '../contexts/useGameContext';

const GameMessages: React.FC = () => {
  const { messages } = useGameContext();

  useEffect(() => {
    console.log('Updated messages:', messages);
  }, [messages]);

  return (
    <div className="fixed top-2 sm:top-4 right-2 sm:right-4 space-y-2 z-40 max-w-xs sm:max-w-sm">
      {messages.map((msg) => (
        <div 
          key={msg.id} 
          className={`p-2 sm:p-3 rounded-lg shadow-lg border transition-all duration-300 transform animate-in slide-in-from-right text-sm ${
            msg.type === 'success' 
              ? 'bg-green-800 border-green-600 text-green-100' 
              : msg.type === 'error'
              ? 'bg-red-800 border-red-600 text-red-100'
              : 'bg-blue-800 border-blue-600 text-blue-100'
          }`}
        >
          {msg.msg}
        </div>
      ))}
    </div>
  );
};

export default GameMessages;
