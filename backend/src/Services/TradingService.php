<?php

namespace App\Services;

use App\Models\Player;
use App\Models\Planet;
use App\Models\Species;
use App\Utils\ValidationUtils;

class TradingService
{
    private GameStateService $gameStateService;
    private PricingService $pricingService;

    public function __construct(
        GameStateService $gameStateService,
        PricingService $pricingService
    ) {
        $this->gameStateService = $gameStateService;
        $this->pricingService = $pricingService;
    }

    /**
     * Purchase a planet for a player
     */
    public function purchasePlanet(string $sessionId, string $planetId): array
    {
        try {
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            if (!$player) {
                return [
                    'success' => false,
                    'message' => 'Player not found'
                ];
            }

            $planet = $this->gameStateService->getPlanetById($planetId);
            if (!$planet) {
                return [
                    'success' => false,
                    'message' => 'Planet not found'
                ];
            }

            if ($planet->ownerId !== null) {
                return [
                    'success' => false,
                    'message' => 'Planet is already owned'
                ];
            }

            if (!$player->canAfford($planet->purchasePrice)) {
                return [
                    'success' => false,
                    'message' => 'Insufficient credits'
                ];
            }

            // Process the purchase
            $player->spendCredits($planet->purchasePrice);
            $planet->ownerId = $player->id;
            
            // Save changes
            $this->gameStateService->updatePlayer($player);
            $this->gameStateService->updatePlanet($planet);

            return [
                'success' => true,
                'message' => "Successfully purchased {$planet->name}",
                'planet' => $planet->toArray(),
                'remainingCredits' => $player->credits
            ];

        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Purchase failed: ' . $e->getMessage()
            ];
        }
    }

    /**
     * Sell a planet to an alien buyer
     */
    public function sellPlanet(string $sessionId, string $planetId, int $buyerId): array
    {
        try {
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            if (!$player) {
                return [
                    'success' => false,
                    'message' => 'Player not found'
                ];
            }

            $planet = $this->gameStateService->getPlanetById($planetId);
            if (!$planet || $planet->ownerId !== $player->id) {
                return [
                    'success' => false,
                    'message' => 'Planet not found or not owned by player'
                ];
            }

            $buyer = $this->gameStateService->getSpeciesById($buyerId);
            if (!$buyer) {
                return [
                    'success' => false,
                    'message' => 'Buyer not found'
                ];
            }

            // Calculate sale price
            $salePrice = $this->pricingService->calculateSalePrice($planet, $buyer);
            
            // Process the sale
            $player->addCredits($salePrice);
            $planet->ownerId = null;
            $planet->soldAt = new \DateTime();
            
            // Save changes
            $this->gameStateService->updatePlayer($player);
            $this->gameStateService->updatePlanet($planet);
            
            // Update game session statistics
            $this->gameStateService->incrementPlanetsTraded($sessionId);

            return [
                'success' => true,
                'message' => "Successfully sold {$planet->name} to {$buyer->name}",
                'salePrice' => $salePrice,
                'newCredits' => $player->credits,
                'buyer' => $buyer->name
            ];

        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Sale failed: ' . $e->getMessage()
            ];
        }
    }

    /**
     * Get owned planets for a player
     */
    public function getOwnedPlanets(string $sessionId): array
    {
        $player = $this->gameStateService->getPlayerBySession($sessionId);
        if (!$player) {
            return [];
        }

        return $this->gameStateService->getPlanetsByOwner($player->id);
    }

    /**
     * Set current planet for a player
     */
    public function setCurrentPlanet(string $sessionId, string $planetId): array
    {
        try {
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            if (!$player) {
                return [
                    'success' => false,
                    'message' => 'Player not found'
                ];
            }

            $planet = $this->gameStateService->getPlanetById($planetId);
            if (!$planet || $planet->ownerId !== $player->id) {
                return [
                    'success' => false,
                    'message' => 'Planet not found or not owned by player'
                ];
            }

            $player->currentPlanetId = $planetId;
            $this->gameStateService->updatePlayer($player);

            return [
                'success' => true,
                'message' => "Selected {$planet->name}",
                'currentPlanet' => $planet->toArray()
            ];

        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Failed to select planet: ' . $e->getMessage()
            ];
        }
    }

    /**
     * Get current planet for a player
     */
    public function getCurrentPlanet(string $sessionId): ?Planet
    {
        $player = $this->gameStateService->getPlayerBySession($sessionId);
        if (!$player || !$player->currentPlanetId) {
            return null;
        }

        $planet = $this->gameStateService->getPlanetById($player->currentPlanetId);
        
        // Verify ownership
        if ($planet && $planet->ownerId === $player->id) {
            return $planet;
        }

        return null;
    }

    /**
     * Calculate profit/loss for a potential sale
     */
    public function calculatePotentialProfit(string $planetId, int $buyerId): array
    {
        $planet = $this->gameStateService->getPlanetById($planetId);
        $buyer = $this->gameStateService->getSpeciesById($buyerId);
        
        if (!$planet || !$buyer) {
            return [
                'success' => false,
                'message' => 'Planet or buyer not found'
            ];
        }

        $salePrice = $this->pricingService->calculateSalePrice($planet, $buyer);
        $profit = $salePrice - $planet->purchasePrice;
        $profitPercentage = $planet->purchasePrice > 0 ? 
            ($profit / $planet->purchasePrice) * 100 : 0;

        return [
            'success' => true,
            'purchasePrice' => $planet->purchasePrice,
            'salePrice' => $salePrice,
            'profit' => $profit,
            'profitPercentage' => round($profitPercentage, 2),
            'compatibility' => $planet->calculateCompatibility($buyer)
        ];
    }

    /**
     * Get trading statistics for a player
     */
    public function getTradingStats(string $sessionId): array
    {
        $player = $this->gameStateService->getPlayerBySession($sessionId);
        if (!$player) {
            return [];
        }

        $session = $this->gameStateService->getCurrentGameSession($sessionId);
        $ownedPlanets = $this->getOwnedPlanets($sessionId);
        
        $totalInvested = array_sum(array_map(fn($p) => $p->purchasePrice, $ownedPlanets));
        
        return [
            'planetsOwned' => count($ownedPlanets),
            'planetsTraded' => $session?->planetsTraded ?? 0,
            'currentCredits' => $player->credits,
            'totalInvested' => $totalInvested,
            'netWorth' => $player->credits + $totalInvested,
            'gameStarted' => $player->gameStarted
        ];
    }
}
