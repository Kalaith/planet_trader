import React, { useState } from 'react';
import { useGameContext } from '../contexts/useGameContext';
import type { Tool } from '../types/entities';

const TerraformingToolsPanel: React.FC = () => {
  const { gameData, applyTool, isToolLocked, credits, currentPlanet } = useGameContext();
  const [expandedTool, setExpandedTool] = useState<string | null>(null);

  const toggleToolDetails = (toolId: string) => {
    setExpandedTool(expandedTool === toolId ? null : toolId);
  };

  const canAffordTool = (tool: Tool): boolean => {
    return credits >= tool.cost;
  };

  const canUseTool = (tool: Tool): boolean => {
    const planetSelected = currentPlanet !== null;
    const canAfford = canAffordTool(tool);
    const notLocked = !isToolLocked(tool);
    
    // Debug logging - remove this after fixing
    console.log(`Tool ${tool.name}:`, {
      planetSelected,
      canAfford,
      notLocked,
      credits,
      toolCost: tool.cost,
      currentPlanet: currentPlanet?.name || 'none',
      upgradeRequired: tool.upgradeRequired
    });
    
    return planetSelected && canAfford && notLocked;
  };

  const getToolIcon = (category: string): string => {
    const icons: Record<string, string> = {
      'temperature': '🌡️',
      'atmosphere': '🌫️', 
      'water': '💧',
      'gravity': '⚖️',
      'radiation': '☢️',
      'biological': '🧬',
    };
    return icons[category] || '🔧';
  };

  const getEffectText = (tool: Tool): string => {
    const effects: string[] = [];
    if (tool.effect) {
      Object.entries(tool.effect).forEach(([stat, value]) => {
        const sign = value > 0 ? '+' : '';
        effects.push(`${stat} ${sign}${value}`);
      });
    }
    return effects.join(', ');
  };

  return (
    <aside className="bg-gray-800 border border-gray-700 rounded-lg shadow-lg">
      <div className="p-3 sm:p-6">
        <div className="border-b border-gray-700 pb-3 sm:pb-4 mb-3 sm:mb-6">
          <h3 className="text-lg sm:text-xl font-bold text-blue-400">🔧 Terraforming Tools</h3>
          <p className="text-xs text-gray-400 mt-1">Tap on a tool to see details</p>
        </div>
        <div className="space-y-2 max-h-64 sm:max-h-96 overflow-y-auto">
          {gameData.terraformingTools.map(tool => {
            const isExpanded = expandedTool === tool.id;
            const locked = isToolLocked(tool);
            const canAfford = canAffordTool(tool);
            const canUse = canUseTool(tool);
            
            return (
              <div key={tool.id} className="bg-gray-700 border border-gray-600 rounded-lg overflow-hidden">
                {/* Compact View - Always Visible */}
                <div 
                  className="p-3 cursor-pointer hover:bg-gray-600 transition-colors duration-200"
                  onClick={() => toggleToolDetails(tool.id)}
                >
                  <div className="flex items-center justify-between">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1">
                        <span className="text-lg">{getToolIcon(tool.category)}</span>
                        <span className="font-bold text-white text-sm">{tool.name}</span>
                        {locked && <span className="text-red-400 text-xs">🔒</span>}
                      </div>
                      <div className="flex items-center gap-2 text-xs">
                        <span className={canAfford ? 'text-green-400' : 'text-red-400'}>
                          {tool.cost}₵
                        </span>
                        <span className="text-gray-400">
                          {getEffectText(tool)}
                        </span>
                      </div>
                    </div>
                    <div className="flex items-center gap-2 ml-2">
                      <button
                        className={`px-3 py-1 rounded text-xs font-semibold transition-colors duration-200 ${
                          canUse
                            ? 'bg-green-600 hover:bg-green-700 text-white'
                            : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                        }`}
                        onClick={(e) => {
                          e.stopPropagation();
                          if (canUse) applyTool(tool);
                        }}
                        disabled={!canUse}
                      >
                        Use
                      </button>
                      <span className={`transform transition-transform duration-200 text-gray-400 ${isExpanded ? 'rotate-180' : ''}`}>
                        ▼
                      </span>
                    </div>
                  </div>
                </div>

                {/* Expanded Details - Only when expanded */}
                {isExpanded && (
                  <div className="px-3 pb-3 border-t border-gray-600">
                    <div className="pt-3 space-y-3">
                      {/* Tool Description */}
                      <div className="text-gray-300 text-sm leading-relaxed">
                        {tool.description}
                      </div>

                      {/* Effects Grid */}
                      <div className="grid grid-cols-1 gap-2 text-xs">
                        {tool.effect && (
                          <div className="bg-green-900/30 border border-green-500 rounded p-2">
                            <div className="text-gray-400 mb-1">Primary Effects</div>
                            <div className="text-green-300">
                              {Object.entries(tool.effect).map(([stat, value]) => (
                                <div key={stat} className="flex items-center justify-between">
                                  <span className="capitalize">{stat}</span>
                                  <span className={value > 0 ? 'text-green-400' : 'text-red-400'}>
                                    {value > 0 ? '+' : ''}{value}
                                  </span>
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                        
                        {tool.sideEffects && Object.keys(tool.sideEffects).length > 0 && (
                          <div className="bg-orange-900/30 border border-orange-500 rounded p-2">
                            <div className="text-gray-400 mb-1">Side Effects</div>
                            <div className="text-orange-300">
                              {Object.entries(tool.sideEffects).map(([stat, value]) => (
                                <div key={stat} className="flex items-center justify-between">
                                  <span className="capitalize">{stat}</span>
                                  <span className={value > 0 ? 'text-green-400' : 'text-red-400'}>
                                    {value > 0 ? '+' : ''}{value}
                                  </span>
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>

                      {/* Requirements and Status */}
                      <div className="bg-gray-600 rounded p-3">
                        <div className="grid grid-cols-2 gap-2 text-xs">
                          <div>
                            <span className="text-gray-400">Cost: </span>
                            <span className={canAfford ? 'text-green-400' : 'text-red-400'}>
                              {tool.cost}₵
                            </span>
                          </div>
                          <div>
                            <span className="text-gray-400">Status: </span>
                            <span className={canUse ? 'text-green-400' : 'text-red-400'}>
                              {!currentPlanet ? 'No Planet' : 
                               locked ? 'Locked' : 
                               !canAfford ? 'No Credits' : 'Ready'}
                            </span>
                          </div>
                        </div>
                        {locked && tool.upgradeRequired && (
                          <div className="mt-2 text-red-400 text-xs">
                            🔒 Requires: {tool.upgradeRequired}
                          </div>
                        )}
                      </div>

                      {/* Use Button */}
                      <button
                        className={`w-full py-3 px-4 rounded-lg text-sm font-semibold transition-all duration-200 ${
                          canUse 
                            ? 'bg-green-600 hover:bg-green-700 text-white shadow-lg hover:shadow-green-500/25' 
                            : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                        }`}
                        onClick={(e) => {
                          e.stopPropagation();
                          if (canUse) applyTool(tool);
                        }}
                        disabled={!canUse}
                      >
                        {!currentPlanet ? '🪐 Select a Planet First' : 
                         locked ? '🔒 Tool Locked' : 
                         !canAfford ? `💰 Need ${tool.cost - credits} More Credits` : 
                         `🔧 Use ${tool.name}`}
                      </button>
                    </div>
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </aside>
  );
};

export default TerraformingToolsPanel;
