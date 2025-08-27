<?php

namespace App\Services;

use App\Models\Planet;
use App\Models\PlanetType;
use App\Utils\RandomUtils;
use App\Utils\ColorUtils;

class PlanetGeneratorService
{
    private PlanetNameService $nameService;
    private array $usedPlanetIds = [];

    public function __construct(PlanetNameService $nameService)
    {
        $this->nameService = $nameService;
    }

    /**
     * Create a new planet with the given type and name
     */
    public function createPlanet(PlanetType $type, string $name): Planet
    {
        $planet = new Planet();
        $planet->id = $this->generatePlanetId();
        $planet->type = $type;
        $planet->name = $name;
        
        // Generate planet properties with variance from base type
        $planet->temperature = RandomUtils::randomRange($type->baseTemp, 20);
        $planet->atmosphere = max(0, RandomUtils::randomRange($type->baseAtmo, 0.4));
        $planet->water = max(0, min(1, RandomUtils::randomRange($type->baseWater, 0.3)));
        $planet->gravity = max(0.1, RandomUtils::randomRange($type->baseGrav, 0.4));
        $planet->radiation = max(0, RandomUtils::randomRange($type->baseRad, 0.3));
        
        // Generate purchase price
        $planet->purchasePrice = (int) floor(1000 + RandomUtils::randomFloat() * 2000);
        
        // Set color
        $planet->color = $type->color;
        
        return $planet;
    }

    /**
     * Generate multiple planet options for purchase
     */
    public function generatePlanetOptions(int $count = 3, array $planetTypes = []): array
    {
        if (empty($planetTypes)) {
            throw new \InvalidArgumentException('Planet types array cannot be empty');
        }

        $planets = [];
        
        for ($i = 0; $i < $count; $i++) {
            $planetType = RandomUtils::randomItem($planetTypes);
            $name = $this->nameService->getRandomPlanetName();
            $planet = $this->createPlanet($planetType, $name);
            $planets[] = $planet;
        }
        
        return $planets;
    }

    /**
     * Generate a unique planet ID
     */
    private function generatePlanetId(): string
    {
        do {
            $id = RandomUtils::generateId('planet');
        } while (in_array($id, $this->usedPlanetIds));
        
        $this->usedPlanetIds[] = $id;
        return $id;
    }

    /**
     * Calculate planet value based on properties
     */
    public function calculatePlanetValue(Planet $planet): int
    {
        $baseValue = $planet->purchasePrice;
        
        // Factors that affect value
        $temperatureBonus = $this->getTemperatureBonus($planet->temperature);
        $atmosphereBonus = $planet->atmosphere * 100;
        $waterBonus = $planet->water * 200;
        $gravityBonus = $this->getGravityBonus($planet->gravity);
        $radiationPenalty = $planet->radiation * -50;
        
        $totalValue = $baseValue + $temperatureBonus + $atmosphereBonus + 
                     $waterBonus + $gravityBonus + $radiationPenalty;
        
        return max(500, (int) $totalValue); // Minimum value of 500
    }

    /**
     * Get temperature bonus/penalty
     */
    private function getTemperatureBonus(float $temperature): float
    {
        // Ideal temperature range is 0-30°C
        if ($temperature >= 0 && $temperature <= 30) {
            return 150; // Bonus for habitable temperature
        }
        
        // Penalty for extreme temperatures
        $deviation = min(abs($temperature), abs($temperature - 30));
        return -$deviation * 2;
    }

    /**
     * Get gravity bonus/penalty
     */
    private function getGravityBonus(float $gravity): float
    {
        // Ideal gravity is around 1.0 (Earth-like)
        $deviation = abs($gravity - 1.0);
        
        if ($deviation <= 0.2) {
            return 100; // Bonus for Earth-like gravity
        }
        
        return -$deviation * 50; // Penalty for extreme gravity
    }

    /**
     * Generate planet with specific characteristics
     */
    public function generatePlanetWithCharacteristics(
        PlanetType $type, 
        array $characteristics = []
    ): Planet {
        $planet = $this->createPlanet($type, $this->nameService->getRandomPlanetName());
        
        // Apply specific characteristics
        foreach ($characteristics as $property => $value) {
            if (property_exists($planet, $property)) {
                $planet->$property = $value;
            }
        }
        
        return $planet;
    }

    /**
     * Get planet rarity based on properties
     */
    public function getPlanetRarity(Planet $planet): string
    {
        $score = 0;
        
        // Score based on ideal conditions
        if ($planet->temperature >= 0 && $planet->temperature <= 30) $score++;
        if ($planet->atmosphere >= 0.8 && $planet->atmosphere <= 1.2) $score++;
        if ($planet->water >= 0.3 && $planet->water <= 0.7) $score++;
        if ($planet->gravity >= 0.8 && $planet->gravity <= 1.2) $score++;
        if ($planet->radiation <= 0.1) $score++;
        
        return match($score) {
            5 => 'Legendary',
            4 => 'Epic',
            3 => 'Rare',
            2 => 'Uncommon',
            default => 'Common'
        };
    }
}
