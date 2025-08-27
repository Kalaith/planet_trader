<?php

namespace App\Services;

use App\Repositories\RepositoryManager;
use App\Utils\RandomUtils;

/**
 * Enhanced Game State Service using Repository Pattern
 */
class GameStateServiceEnhanced 
{
    private RepositoryManager $repositories;
    private PlanetGeneratorService $planetGenerator;
    
    public function __construct(RepositoryManager $repositories, PlanetGeneratorService $planetGenerator) 
    {
        $this->repositories = $repositories;
        $this->planetGenerator = $planetGenerator;
    }
    
    /**
     * Create a new game session
     */
    public function createSession(?int $playerId = null, int $startingCredits = 10000): array 
    {
        $sessionId = $this->generateSessionId();
        
        try {
            $this->repositories->beginTransaction();
            
            // Create the session
            $this->repositories->sessions()->createSession($sessionId, $playerId, $startingCredits);
            
            // Generate initial planets
            $initialPlanets = $this->generateInitialPlanets($sessionId);
            
            $this->repositories->commit();
            
            return [
                'success' => true,
                'session_id' => $sessionId,
                'starting_credits' => $startingCredits,
                'initial_planets' => count($initialPlanets),
                'message' => 'Game session created successfully'
            ];
            
        } catch (\Exception $e) {
            $this->repositories->rollback();
            return [
                'success' => false,
                'error' => 'Failed to create session: ' . $e->getMessage()
            ];
        }
    }
    
    /**
     * Load game session with full state
     */
    public function loadSession(string $sessionId): array 
    {
        $session = $this->repositories->sessions()->findWithPlayer($sessionId);
        
        if (!$session) {
            return [
                'success' => false,
                'error' => 'Session not found'
            ];
        }
        
        if (!$session['is_active']) {
            return [
                'success' => false,
                'error' => 'Session is no longer active'
            ];
        }
        
        // Update last activity
        $this->repositories->sessions()->updateActivity($sessionId);
        
        // Get planets with full details
        $planets = $this->repositories->planets()->findSessionPlanetsWithDetails($sessionId);
        
        // Get session statistics
        $stats = $this->repositories->sessions()->getStats($sessionId);
        
        // Get available tools
        $availableTools = $this->repositories->tools()->findUnlockedForSession($stats);
        
        return [
            'success' => true,
            'session' => $session,
            'planets' => $planets,
            'statistics' => $stats,
            'available_tools' => $availableTools
        ];
    }
    
    /**
     * Generate a new planet for the session
     */
    public function generateNewPlanet(string $sessionId): array 
    {
        $session = $this->repositories->sessions()->find($sessionId);
        
        if (!$session || !$session['is_active']) {
            return [
                'success' => false,
                'error' => 'Invalid or inactive session'
            ];
        }
        
        try {
            $planet = $this->planetGenerator->generatePlanet($sessionId);
            
            // Increment planets visited counter
            $this->repositories->sessions()->incrementPlanetsVisited($sessionId);
            
            return [
                'success' => true,
                'planet' => $planet,
                'message' => 'New planet discovered!'
            ];
            
        } catch (\Exception $e) {
            return [
                'success' => false,
                'error' => 'Failed to generate planet: ' . $e->getMessage()
            ];
        }
    }
    
    /**
     * Process a trade transaction
     */
    public function processTrade(string $sessionId, int $planetId, string $action, array $toolsUsed = []): array 
    {
        $session = $this->repositories->sessions()->find($sessionId);
        $planet = $this->repositories->planets()->findWithDetails($planetId);
        
        if (!$session || !$planet) {
            return [
                'success' => false,
                'error' => 'Session or planet not found'
            ];
        }
        
        if ($planet['session_id'] !== $sessionId) {
            return [
                'success' => false,
                'error' => 'Planet does not belong to this session'
            ];
        }
        
        if ($planet['is_sold'] && $action === 'sell') {
            return [
                'success' => false,
                'error' => 'Planet has already been sold'
            ];
        }
        
        try {
            $this->repositories->beginTransaction();
            
            $result = match($action) {
                'buy' => $this->processBuy($session, $planet, $toolsUsed),
                'sell' => $this->processSell($session, $planet, $toolsUsed),
                default => throw new \InvalidArgumentException('Invalid action: ' . $action)
            };
            
            $this->repositories->commit();
            return $result;
            
        } catch (\Exception $e) {
            $this->repositories->rollback();
            return [
                'success' => false,
                'error' => 'Trade failed: ' . $e->getMessage()
            ];
        }
    }
    
    /**
     * Apply terraforming tools to a planet
     */
    public function applyTools(string $sessionId, int $planetId, array $toolIds): array 
    {
        $session = $this->repositories->sessions()->find($sessionId);
        $planet = $this->repositories->planets()->findWithDetails($planetId);
        
        if (!$session || !$planet) {
            return [
                'success' => false,
                'error' => 'Session or planet not found'
            ];
        }
        
        if ($planet['session_id'] !== $sessionId) {
            return [
                'success' => false,
                'error' => 'Planet does not belong to this session'
            ];
        }
        
        try {
            $this->repositories->beginTransaction();
            
            $totalCost = 0;
            $appliedTools = $planet['tools_applied'] ?? [];
            
            foreach ($toolIds as $toolId) {
                $tool = $this->repositories->tools()->find($toolId);
                if (!$tool) {
                    continue;
                }
                
                $totalCost += $tool['cost'];
                $appliedTools[] = [
                    'tool_id' => $toolId,
                    'name' => $tool['name'],
                    'effect_type' => $tool['effect_type'],
                    'effect_value' => $tool['effect_value'],
                    'applied_at' => date('Y-m-d H:i:s')
                ];
            }
            
            if ($session['current_credits'] < $totalCost) {
                throw new \Exception('Insufficient credits for tools');
            }
            
            // Update planet with applied tools
            $this->repositories->planets()->applyTools($planetId, $appliedTools);
            
            // Recalculate planet value based on tools
            $newValue = $this->calculateValueWithTools($planet, $appliedTools);
            $this->repositories->planets()->updateValue($planetId, $newValue);
            
            // Deduct credits
            $newCredits = $session['current_credits'] - $totalCost;
            $this->repositories->sessions()->updateCredits($sessionId, $newCredits);
            
            $this->repositories->commit();
            
            return [
                'success' => true,
                'tools_applied' => count($toolIds),
                'total_cost' => $totalCost,
                'new_planet_value' => $newValue,
                'remaining_credits' => $newCredits,
                'message' => 'Tools applied successfully'
            ];
            
        } catch (\Exception $e) {
            $this->repositories->rollback();
            return [
                'success' => false,
                'error' => 'Failed to apply tools: ' . $e->getMessage()
            ];
        }
    }
    
    /**
     * End a game session
     */
    public function endSession(string $sessionId): array 
    {
        $session = $this->repositories->sessions()->find($sessionId);
        
        if (!$session) {
            return [
                'success' => false,
                'error' => 'Session not found'
            ];
        }
        
        // Get final statistics
        $finalStats = $this->repositories->sessions()->getStats($sessionId);
        
        // Mark session as inactive
        $this->repositories->sessions()->endSession($sessionId);
        
        return [
            'success' => true,
            'final_stats' => $finalStats,
            'message' => 'Session ended successfully'
        ];
    }
    
    /**
     * Get current session status
     */
    public function getSessionStatus(string $sessionId): array 
    {
        $session = $this->repositories->sessions()->find($sessionId);
        
        if (!$session) {
            return [
                'success' => false,
                'error' => 'Session not found'
            ];
        }
        
        $stats = $this->repositories->sessions()->getStats($sessionId);
        $planetCount = $this->repositories->planets()->count(['session_id' => $sessionId]);
        $unsoldPlanets = $this->repositories->planets()->count([
            'session_id' => $sessionId,
            'is_sold' => false
        ]);
        
        return [
            'success' => true,
            'session_id' => $sessionId,
            'is_active' => (bool) $session['is_active'],
            'current_credits' => $session['current_credits'],
            'planets_discovered' => $planetCount,
            'planets_unsold' => $unsoldPlanets,
            'statistics' => $stats
        ];
    }
    
    // Private helper methods
    
    private function generateSessionId(): string 
    {
        return uniqid('session_', true);
    }
    
    private function generateInitialPlanets(string $sessionId, int $count = 3): array 
    {
        $planets = [];
        
        for ($i = 0; $i < $count; $i++) {
            $planets[] = $this->planetGenerator->generatePlanet($sessionId);
        }
        
        return $planets;
    }
    
    private function processBuy(array $session, array $planet, array $toolsUsed): array 
    {
        $cost = $planet['current_value'];
        
        if ($session['current_credits'] < $cost) {
            throw new \Exception('Insufficient credits');
        }
        
        // Update session credits
        $newCredits = $session['current_credits'] - $cost;
        $this->repositories->sessions()->updateCredits($session['id'], $newCredits);
        
        // Record transaction
        $this->repositories->transactions()->recordBuy(
            $session['id'],
            $planet['id'],
            $cost,
            $toolsUsed
        );
        
        return [
            'success' => true,
            'action' => 'buy',
            'cost' => $cost,
            'remaining_credits' => $newCredits,
            'message' => "Purchased {$planet['name']} for {$cost} credits"
        ];
    }
    
    private function processSell(array $session, array $planet, array $toolsUsed): array 
    {
        $salePrice = $this->calculateSalePrice($planet, $toolsUsed);
        $originalCost = $planet['current_value'];
        $profit = $salePrice - $originalCost;
        
        // Update session credits and profit
        $newCredits = $session['current_credits'] + $salePrice;
        $this->repositories->sessions()->updateCredits($session['id'], $newCredits);
        $this->repositories->sessions()->addProfit($session['id'], $profit);
        
        // Mark planet as sold
        $this->repositories->planets()->markAsSold($planet['id'], $salePrice);
        
        // Record transaction
        $this->repositories->transactions()->recordSell(
            $session['id'],
            $planet['id'],
            $salePrice,
            $profit,
            $toolsUsed
        );
        
        return [
            'success' => true,
            'action' => 'sell',
            'sale_price' => $salePrice,
            'profit' => $profit,
            'remaining_credits' => $newCredits,
            'message' => "Sold {$planet['name']} for {$salePrice} credits (profit: {$profit})"
        ];
    }
    
    private function calculateSalePrice(array $planet, array $toolsUsed): int 
    {
        $baseValue = $planet['current_value'];
        $multiplier = 1.0;
        
        // Apply species multiplier if present
        if ($planet['species_multiplier']) {
            $multiplier *= $planet['species_multiplier'];
        }
        
        // Apply tool effects
        foreach ($toolsUsed as $toolId) {
            $tool = $this->repositories->tools()->find($toolId);
            if ($tool && $tool['effect_type'] === 'value_multiplier') {
                $multiplier *= $tool['effect_value'];
            }
        }
        
        // Add some randomness (±10%)
        $randomFactor = RandomUtils::floatBetween(0.9, 1.1);
        
        return (int) ($baseValue * $multiplier * $randomFactor);
    }
    
    private function calculateValueWithTools(array $planet, array $appliedTools): int 
    {
        $baseValue = $planet['base_value'];
        $multiplier = 1.0;
        
        foreach ($appliedTools as $appliedTool) {
            if ($appliedTool['effect_type'] === 'value_multiplier') {
                $multiplier *= $appliedTool['effect_value'];
            }
        }
        
        return (int) ($baseValue * $multiplier);
    }
}
