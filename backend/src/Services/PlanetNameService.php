<?php

namespace App\Services;

use App\Utils\RandomUtils;

class PlanetNameService
{
    private array $usedNames = [];
    private array $planetNames = [];
    
    private const STAR_PREFIXES = [
        "HD", "Kepler", "Gliese", "Epsilon", "Tau", "TYC", 
        "Alpha", "Delta", "Theta", "Zeta", "Xeno", "Vesmir", 
        "PX", "LV", "LX", "Beta", "Gamma", "Sigma", "Omega"
    ];
    
    private const ROMAN_NUMERALS = [
        "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
        "XI", "XII", "XIII", "XIV", "XV"
    ];
    
    private const PLANET_SUFFIXES = [
        "Prime", "Secundus", "Tertius", "Major", "Minor", "Nova", 
        "Proxima", "Ultima", "Centralis", "Australis", "Borealis"
    ];

    public function __construct(array $planetNames = [])
    {
        $this->planetNames = $planetNames;
    }

    /**
     * Get a random planet name, preferring predefined names
     */
    public function getRandomPlanetName(): string
    {
        // Try to use predefined names first
        if (!empty($this->planetNames) && count($this->usedNames) < count($this->planetNames)) {
            do {
                $name = RandomUtils::randomItem($this->planetNames);
            } while (in_array($name, $this->usedNames));
            
            $this->usedNames[] = $name;
            return $name;
        }
        
        // Generate procedural name
        return $this->generateProceduralName();
    }

    /**
     * Generate a procedural planet name
     */
    private function generateProceduralName(): string
    {
        $nameType = RandomUtils::randomInt(1, 4);
        
        switch ($nameType) {
            case 1:
                return $this->generateStarSystemName();
            case 2:
                return $this->generateClassicName();
            case 3:
                return $this->generateCompoundName();
            default:
                return $this->generateNumericName();
        }
    }

    /**
     * Generate star system style name (e.g., "Kepler-442b")
     */
    private function generateStarSystemName(): string
    {
        $prefix = RandomUtils::randomItem(self::STAR_PREFIXES);
        $code = RandomUtils::randomInt(100, 9999);
        $suffix = RandomUtils::randomBool(0.7) ? chr(97 + RandomUtils::randomInt(0, 7)) : ''; // a-h
        
        $name = trim("{$prefix}-{$code}{$suffix}");
        
        // Ensure uniqueness
        $tries = 0;
        while (in_array($name, $this->usedNames) && $tries < 10) {
            $code = RandomUtils::randomInt(100, 9999);
            $name = trim("{$prefix}-{$code}{$suffix}");
            $tries++;
        }
        
        $this->usedNames[] = $name;
        return $name;
    }

    /**
     * Generate classic style name with Roman numerals
     */
    private function generateClassicName(): string
    {
        $prefix = RandomUtils::randomItem(self::STAR_PREFIXES);
        $roman = RandomUtils::randomItem(self::ROMAN_NUMERALS);
        
        $name = "{$prefix} {$roman}";
        
        // Ensure uniqueness
        $tries = 0;
        while (in_array($name, $this->usedNames) && $tries < 10) {
            $prefix = RandomUtils::randomItem(self::STAR_PREFIXES);
            $roman = RandomUtils::randomItem(self::ROMAN_NUMERALS);
            $name = "{$prefix} {$roman}";
            $tries++;
        }
        
        $this->usedNames[] = $name;
        return $name;
    }

    /**
     * Generate compound name with suffix
     */
    private function generateCompoundName(): string
    {
        $prefix = RandomUtils::randomItem(self::STAR_PREFIXES);
        $suffix = RandomUtils::randomItem(self::PLANET_SUFFIXES);
        
        $name = "{$prefix} {$suffix}";
        
        // Ensure uniqueness
        $tries = 0;
        while (in_array($name, $this->usedNames) && $tries < 10) {
            $prefix = RandomUtils::randomItem(self::STAR_PREFIXES);
            $suffix = RandomUtils::randomItem(self::PLANET_SUFFIXES);
            $name = "{$prefix} {$suffix}";
            $tries++;
        }
        
        $this->usedNames[] = $name;
        return $name;
    }

    /**
     * Generate numeric designation
     */
    private function generateNumericName(): string
    {
        $sector = RandomUtils::randomInt(1, 99);
        $grid = RandomUtils::randomInt(1, 999);
        $designation = chr(65 + RandomUtils::randomInt(0, 25)); // A-Z
        
        $name = "Sector {$sector}-{$grid}{$designation}";
        
        // Ensure uniqueness
        $tries = 0;
        while (in_array($name, $this->usedNames) && $tries < 10) {
            $grid = RandomUtils::randomInt(1, 999);
            $designation = chr(65 + RandomUtils::randomInt(0, 25));
            $name = "Sector {$sector}-{$grid}{$designation}";
            $tries++;
        }
        
        $this->usedNames[] = $name;
        return $name;
    }

    /**
     * Reset used names (useful for testing)
     */
    public function resetUsedNames(): void
    {
        $this->usedNames = [];
    }

    /**
     * Get list of used names
     */
    public function getUsedNames(): array
    {
        return $this->usedNames;
    }

    /**
     * Set predefined planet names
     */
    public function setPlanetNames(array $names): void
    {
        $this->planetNames = $names;
    }

    /**
     * Check if a name is available
     */
    public function isNameAvailable(string $name): bool
    {
        return !in_array($name, $this->usedNames);
    }

    /**
     * Reserve a specific name
     */
    public function reserveName(string $name): bool
    {
        if ($this->isNameAvailable($name)) {
            $this->usedNames[] = $name;
            return true;
        }
        return false;
    }
}
