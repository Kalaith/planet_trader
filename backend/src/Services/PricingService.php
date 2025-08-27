<?php

namespace App\Services;

use App\Models\Player;
use App\Models\Planet;
use App\Models\Species;
use App\Utils\RandomUtils;

class PricingService
{
    /**
     * Calculate the sale price of a planet to a specific buyer
     */
    public function calculateSalePrice(Planet $planet, Species $buyer): int
    {
        $basePrice = $buyer->currentPrice ?? $buyer->basePrice;
        $compatibility = $planet->calculateCompatibility($buyer);
        
        // Apply compatibility multiplier
        $compatibilityMultiplier = 0.5 + ($compatibility * 1.5); // 0.5x to 2.0x multiplier
        
        // Apply rarity bonus
        $rarityMultiplier = $this->getRarityMultiplier($planet);
        
        // Apply market volatility (±10%)
        $volatility = RandomUtils::randomFloat(0.9, 1.1);
        
        $finalPrice = $basePrice * $compatibilityMultiplier * $rarityMultiplier * $volatility;
        
        return max(100, (int) round($finalPrice)); // Minimum price of 100
    }

    /**
     * Generate current market price for a species
     */
    public function generateCurrentPrice(Species $species): int
    {
        // Base price with market fluctuation (±25%)
        $fluctuation = RandomUtils::randomFloat(0.75, 1.25);
        return (int) round($species->basePrice * $fluctuation);
    }

    /**
     * Calculate planet purchase price based on properties
     */
    public function calculatePurchasePrice(Planet $planet): int
    {
        $basePrice = 1000;
        
        // Factors that affect price
        $temperatureValue = $this->getTemperatureValue($planet->temperature);
        $atmosphereValue = $planet->atmosphere * 200;
        $waterValue = $planet->water * 400;
        $gravityValue = $this->getGravityValue($planet->gravity);
        $radiationPenalty = $planet->radiation * -100;
        
        // Size/rarity bonus
        $rarityBonus = $this->getRarityBonus($planet);
        
        $totalPrice = $basePrice + $temperatureValue + $atmosphereValue + 
                     $waterValue + $gravityValue + $radiationPenalty + $rarityBonus;
        
        // Add some randomness (±15%)
        $randomVariation = RandomUtils::randomFloat(0.85, 1.15);
        
        return max(500, (int) round($totalPrice * $randomVariation));
    }

    /**
     * Get temperature value contribution
     */
    private function getTemperatureValue(float $temperature): float
    {
        // Ideal temperature range (habitable zone)
        if ($temperature >= -10 && $temperature <= 40) {
            return 300; // Premium for habitable temperature
        }
        
        // Moderate temperatures
        if ($temperature >= -50 && $temperature <= 80) {
            return 100;
        }
        
        // Extreme temperatures reduce value
        return -200;
    }

    /**
     * Get gravity value contribution
     */
    private function getGravityValue(float $gravity): float
    {
        // Earth-like gravity is most valuable
        $deviation = abs($gravity - 1.0);
        
        if ($deviation <= 0.3) {
            return 200; // Premium for Earth-like gravity
        }
        
        if ($deviation <= 0.7) {
            return 50; // Moderate gravity
        }
        
        return -100; // Extreme gravity reduces value
    }

    /**
     * Get rarity multiplier based on planet characteristics
     */
    private function getRarityMultiplier(Planet $planet): float
    {
        $idealConditions = 0;
        
        // Count how many conditions are "ideal"
        if ($planet->temperature >= 0 && $planet->temperature <= 30) $idealConditions++;
        if ($planet->atmosphere >= 0.8 && $planet->atmosphere <= 1.2) $idealConditions++;
        if ($planet->water >= 0.3 && $planet->water <= 0.7) $idealConditions++;
        if ($planet->gravity >= 0.8 && $planet->gravity <= 1.2) $idealConditions++;
        if ($planet->radiation <= 0.1) $idealConditions++;
        
        return match($idealConditions) {
            5 => 2.5,  // Legendary - perfect conditions
            4 => 2.0,  // Epic - nearly perfect
            3 => 1.5,  // Rare - good conditions
            2 => 1.2,  // Uncommon - decent conditions
            1 => 1.0,  // Common - some good conditions
            default => 0.8  // Poor conditions
        };
    }

    /**
     * Get rarity bonus for purchase price calculation
     */
    private function getRarityBonus(Planet $planet): int
    {
        $multiplier = $this->getRarityMultiplier($planet);
        
        return (int) (($multiplier - 1.0) * 500); // Convert multiplier to bonus amount
    }

    /**
     * Calculate dynamic market prices for all species
     */
    public function updateMarketPrices(array $species): array
    {
        $updatedSpecies = [];
        
        foreach ($species as $speciesItem) {
            $speciesItem->currentPrice = $this->generateCurrentPrice($speciesItem);
            $updatedSpecies[] = $speciesItem;
        }
        
        return $updatedSpecies;
    }

    /**
     * Get market trend information
     */
    public function getMarketTrend(Species $species): array
    {
        $currentPrice = $species->currentPrice ?? $species->basePrice;
        $basePrice = $species->basePrice;
        
        $percentChange = (($currentPrice - $basePrice) / $basePrice) * 100;
        
        $trend = 'stable';
        if ($percentChange > 10) {
            $trend = 'rising';
        } elseif ($percentChange < -10) {
            $trend = 'falling';
        }
        
        return [
            'species' => $species->name,
            'currentPrice' => $currentPrice,
            'basePrice' => $basePrice,
            'percentChange' => round($percentChange, 2),
            'trend' => $trend
        ];
    }

    /**
     * Calculate bulk discount for multiple planet purchases
     */
    public function calculateBulkDiscount(array $planets): float
    {
        $count = count($planets);
        
        if ($count >= 5) {
            return 0.15; // 15% discount for 5+ planets
        } elseif ($count >= 3) {
            return 0.10; // 10% discount for 3+ planets
        }
        
        return 0.0; // No discount for fewer than 3 planets
    }

    /**
     * Apply seasonal price modifiers
     */
    public function applySeasonalModifier(int $price, string $season = null): int
    {
        if (!$season) {
            $season = $this->getCurrentSeason();
        }
        
        $modifier = match($season) {
            'spring' => 1.05,   // 5% increase - exploration season
            'summer' => 1.10,   // 10% increase - peak trading
            'autumn' => 0.95,   // 5% decrease - harvest time
            'winter' => 0.90,   // 10% decrease - slow period
            default => 1.0
        };
        
        return (int) round($price * $modifier);
    }

    /**
     * Get current season (for demo purposes, cycles every 30 seconds)
     */
    private function getCurrentSeason(): string
    {
        $seasons = ['spring', 'summer', 'autumn', 'winter'];
        $index = (time() / 30) % 4; // Change season every 30 seconds
        return $seasons[(int) $index];
    }
}
