<?php

namespace App\Database;

/**
 * Enhanced Database seeder for Planet Trader static data
 */
class SeederEnhanced 
{
    private $pdo;
    private $dataPath;
    
    public function __construct($pdo) 
    {
        $this->pdo = $pdo;
        $this->dataPath = __DIR__ . '/../../../data/';
    }
    
    public function run(): void 
    {
        echo "🌱 Seeding planet types...\n";
        $this->seedPlanetTypes();
        
        echo "👽 Seeding alien species...\n";
        $this->seedSpecies();
        
        echo "🔧 Seeding terraforming tools...\n";
        $this->seedTools();
        
        echo "🪐 Seeding planet names...\n";
        $this->seedPlanetNames();
        
        echo "✅ Database seeding complete!\n";
    }
    
    private function seedPlanetTypes(): void 
    {
        $jsonData = $this->loadJsonFile('planet_types.json');
        
        $stmt = $this->pdo->prepare("
            INSERT IGNORE INTO planet_types 
            (name, description, base_value_multiplier, rarity_weight, color_scheme, atmosphere_types) 
            VALUES (?, ?, ?, ?, ?, ?)
        ");
        
        foreach ($jsonData as $type) {
            // Convert baseTemp and other attributes to description and multiplier
            $temp = $type['baseTemp'] ?? 0;
            $atmo = $type['baseAtmo'] ?? 0.5;
            $water = $type['baseWater'] ?? 0.3;
            $grav = $type['baseGrav'] ?? 1.0;
            $rad = $type['baseRad'] ?? 0.2;
            
            $description = $this->generatePlanetTypeDescription($type['name'], $temp, $atmo, $water, $grav, $rad);
            $multiplier = $this->calculateValueMultiplier($temp, $atmo, $water, $grav, $rad);
            $rarity = $this->calculateRarity($type['name']);
            $colorScheme = $this->generateColorScheme($type['name'], $temp);
            $atmospheres = $this->generateAtmosphereTypes($atmo, $temp);
            
            $stmt->execute([
                $type['name'],
                $description,
                $multiplier,
                $rarity,
                json_encode($colorScheme),
                json_encode($atmospheres)
            ]);
        }
    }
    
    private function seedSpecies(): void 
    {
        $jsonData = $this->loadJsonFile('alien_species.json');
        
        $stmt = $this->pdo->prepare("
            INSERT IGNORE INTO species 
            (name, type, description, preferred_planet_types, appearance, value_multiplier, rarity_weight) 
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ");
        
        foreach ($jsonData as $species) {
            $preferredTypes = $this->determinePlanetPreferences($species);
            $appearance = [
                'color' => $species['color'] ?? '#FFFFFF',
                'texture' => $this->determineTexture($species['name'])
            ];
            
            $valueMultiplier = ($species['basePrice'] ?? 3000) / 3000; // Normalize to 1.0 base
            $rarity = $this->calculateSpeciesRarity($species['basePrice'] ?? 3000);
            
            $stmt->execute([
                $species['name'],
                $this->determineSpeciesType($species['name']),
                $species['description'] ?? 'A unique alien species.',
                json_encode($preferredTypes),
                json_encode($appearance),
                $valueMultiplier,
                $rarity
            ]);
        }
    }
    
    private function seedTools(): void 
    {
        $jsonData = $this->loadJsonFile('terraforming_tools.json');
        
        $stmt = $this->pdo->prepare("
            INSERT IGNORE INTO tools 
            (name, description, effect_type, effect_value, cost, unlock_requirements, applicable_planet_types, research_tier) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ");
        
        foreach ($jsonData as $tool) {
            $effectType = $this->determineEffectType($tool);
            $effectValue = $this->calculateEffectValue($tool);
            $requirements = $this->generateUnlockRequirements($tool);
            $applicableTypes = $this->determineApplicableTypes($tool);
            
            $stmt->execute([
                $tool['name'],
                $tool['description'] ?? 'A terraforming tool.',
                $effectType,
                $effectValue,
                $tool['cost'] ?? 100,
                json_encode($requirements),
                json_encode($applicableTypes),
                $tool['tier'] ?? 1
            ]);
        }
    }
    
    private function seedPlanetNames(): void 
    {
        $jsonData = $this->loadJsonFile('planet_names.json');
        
        $stmt = $this->pdo->prepare("
            INSERT IGNORE INTO planet_names (name, origin, category) 
            VALUES (?, ?, ?)
        ");
        
        foreach ($jsonData as $name) {
            $origin = $this->determineNameOrigin($name);
            $category = $this->determineNameCategory($name);
            $stmt->execute([$name, $origin, $category]);
        }
    }
    
    // Helper methods
    
    private function loadJsonFile(string $filename): array 
    {
        $path = $this->dataPath . $filename;
        if (!file_exists($path)) {
            echo "⚠️  Warning: {$filename} not found, using fallback data\n";
            return $this->getFallbackData($filename);
        }
        
        $content = file_get_contents($path);
        $data = json_decode($content, true);
        
        if ($data === null) {
            throw new \Exception("Failed to parse JSON from {$filename}");
        }
        
        return $data;
    }
    
    private function generatePlanetTypeDescription(string $name, float $temp, float $atmo, float $water, float $grav, float $rad): string 
    {
        $tempDesc = $temp > 60 ? 'scorching' : ($temp < -20 ? 'frozen' : 'temperate');
        $atmoDesc = $atmo > 1.5 ? 'dense' : ($atmo < 0.3 ? 'thin' : 'moderate');
        $waterDesc = $water > 0.7 ? 'water-rich' : ($water < 0.1 ? 'arid' : 'moderate water');
        
        return "{$name} planets feature {$tempDesc} temperatures, {$atmoDesc} atmospheres, and {$waterDesc} content.";
    }
    
    private function calculateValueMultiplier(float $temp, float $atmo, float $water, float $grav, float $rad): float 
    {
        // Base multiplier calculation based on environmental factors
        $base = 1.0;
        
        // Extreme conditions can increase or decrease value
        if ($rad > 0.8) $base += 0.3; // High radiation = dangerous but valuable
        if ($temp > 100 || $temp < -50) $base += 0.2; // Extreme temperatures
        if ($grav > 1.5 || $grav < 0.4) $base += 0.1; // Unusual gravity
        
        // Habitable conditions slightly reduce value (common)
        if ($temp >= 0 && $temp <= 30 && $atmo >= 0.5 && $atmo <= 1.2) {
            $base -= 0.1;
        }
        
        return max(0.5, min(2.0, $base));
    }
    
    private function calculateRarity(string $name): int 
    {
        $rare = ['Crystalline', 'Hollow Planet', 'Bio-Waste Zone', 'Irradiated Core'];
        $uncommon = ['Lava Crust', 'Clouded Giant', 'Tidally Locked'];
        
        if (in_array($name, $rare)) return 1;
        if (in_array($name, $uncommon)) return 2;
        return 3; // Common
    }
    
    private function generateColorScheme(string $name, float $temp): array 
    {
        $schemes = [
            'Ice World' => ['primary' => '#B0E0E6', 'secondary' => '#F0F8FF'],
            'Volcanic' => ['primary' => '#FF4500', 'secondary' => '#FF6347'],
            'Lava Crust' => ['primary' => '#DC143C', 'secondary' => '#FF1493'],
            'Ocean World' => ['primary' => '#006994', 'secondary' => '#4682B4'],
            'Desert' => ['primary' => '#DEB887', 'secondary' => '#F4A460'],
            'Gas Dwarf' => ['primary' => '#9370DB', 'secondary' => '#8A2BE2'],
            'Crystalline' => ['primary' => '#9370DB', 'secondary' => '#DDA0DD']
        ];
        
        if (isset($schemes[$name])) {
            return $schemes[$name];
        }
        
        // Generate based on temperature
        if ($temp > 80) {
            return ['primary' => '#FF4500', 'secondary' => '#FF6347'];
        } elseif ($temp < -20) {
            return ['primary' => '#B0E0E6', 'secondary' => '#F0F8FF'];
        } else {
            return ['primary' => '#8B4513', 'secondary' => '#A0522D'];
        }
    }
    
    private function generateAtmosphereTypes(float $atmo, float $temp): array 
    {
        $types = [];
        
        if ($atmo < 0.3) $types[] = 'thin';
        elseif ($atmo > 1.5) $types[] = 'dense';
        else $types[] = 'moderate';
        
        if ($temp > 80) $types[] = 'toxic';
        elseif ($temp < -20) $types[] = 'frozen';
        
        return $types;
    }
    
    private function determinePlanetPreferences(array $species): array 
    {
        // Analyze species requirements and match to planet types
        $preferences = [];
        $temp = $species['tempRange'] ?? [0, 30];
        $avgTemp = ($temp[0] + $temp[1]) / 2;
        
        if ($avgTemp > 60) $preferences[] = 'Volcanic';
        if ($avgTemp < -10) $preferences[] = 'Ice World';
        if (($species['waterRange'][1] ?? 0.5) > 0.7) $preferences[] = 'Ocean World';
        if (($species['atmoRange'][1] ?? 1.0) < 0.3) $preferences[] = 'Rocky';
        
        return empty($preferences) ? ['Rocky', 'Desert'] : $preferences;
    }
    
    private function determineTexture(string $name): string 
    {
        if (strpos($name, 'Crystal') !== false) return 'crystalline';
        if (strpos($name, 'Aquatic') !== false) return 'smooth';
        if (strpos($name, 'Gas') !== false) return 'ethereal';
        return 'rough';
    }
    
    private function determineSpeciesType(string $name): string 
    {
        if (strpos($name, 'Crystal') !== false) return 'Silicon-based';
        if (strpos($name, 'Aquatic') !== false) return 'Water-dwelling';
        if (strpos($name, 'Gas') !== false) return 'Energy-based';
        return 'Carbon-based';
    }
    
    private function calculateSpeciesRarity(int $basePrice): int 
    {
        if ($basePrice > 6000) return 1; // Rare
        if ($basePrice > 4000) return 2; // Uncommon
        return 3; // Common
    }
    
    private function determineEffectType(array $tool): string 
    {
        $category = $tool['category'] ?? 'general';
        $effect = $tool['effect'] ?? [];
        
        if (isset($effect['temperature'])) return 'temperature_modifier';
        if (isset($effect['atmosphere'])) return 'atmosphere_modifier';
        if (isset($effect['water'])) return 'water_modifier';
        if (isset($effect['gravity'])) return 'gravity_modifier';
        if (isset($effect['radiation'])) return 'radiation_modifier';
        
        return 'value_multiplier';
    }
    
    private function calculateEffectValue(array $tool): float 
    {
        $effect = $tool['effect'] ?? [];
        
        // Get the primary effect value
        foreach ($effect as $value) {
            return (float) $value;
        }
        
        // Default to cost-based value
        $cost = $tool['cost'] ?? 100;
        return 1.0 + ($cost / 1000);
    }
    
    private function generateUnlockRequirements(array $tool): array 
    {
        $tier = $tool['tier'] ?? 1;
        $unlocked = $tool['unlocked'] ?? false;
        
        if ($unlocked || $tier === 1) {
            return [];
        }
        
        $requirements = [];
        if ($tier >= 2) $requirements['planets_visited'] = $tier * 3;
        if ($tier >= 3) $requirements['credits_earned'] = $tier * 10000;
        
        return $requirements;
    }
    
    private function determineApplicableTypes(array $tool): array 
    {
        $category = $tool['category'] ?? 'general';
        
        switch ($category) {
            case 'temperature':
                return ['Rocky', 'Desert', 'Ice World', 'Volcanic'];
            case 'atmosphere':
                return ['Rocky', 'Gas Dwarf', 'Desert'];
            case 'water':
                return ['Desert', 'Rocky', 'Tundra'];
            default:
                return ['all'];
        }
    }
    
    private function determineNameOrigin(string $name): string 
    {
        if (preg_match('/^[A-Z]+[-\d]+[a-z]?$/', $name)) return 'astronomical';
        if (strpos($name, '-') !== false) return 'catalog';
        return 'fictional';
    }
    
    private function determineNameCategory(string $name): string 
    {
        if (strpos($name, 'Kepler') !== false) return 'exoplanet';
        if (strpos($name, 'TRAPPIST') !== false) return 'system';
        return 'general';
    }
    
    private function getFallbackData(string $filename): array 
    {
        // Provide minimal fallback data if JSON files are missing
        switch ($filename) {
            case 'planet_types.json':
                return [
                    ["name" => "Rocky", "baseTemp" => 15, "baseAtmo" => 0.3, "baseWater" => 0.1, "baseGrav" => 1.0, "baseRad" => 0.2],
                    ["name" => "Ice World", "baseTemp" => -40, "baseAtmo" => 0.1, "baseWater" => 0.8, "baseGrav" => 0.8, "baseRad" => 0.1],
                    ["name" => "Desert", "baseTemp" => 50, "baseAtmo" => 0.2, "baseWater" => 0.05, "baseGrav" => 0.9, "baseRad" => 0.3]
                ];
            case 'alien_species.json':
                return [
                    ["name" => "Basic Lifeforms", "description" => "Simple alien species", "basePrice" => 3000, "color" => "#FFFFFF"]
                ];
            case 'terraforming_tools.json':
                return [
                    ["name" => "Basic Scanner", "description" => "Basic planetary analysis tool", "cost" => 100, "tier" => 1, "unlocked" => true]
                ];
            case 'planet_names.json':
                return ["Kepler-442b", "New Earth", "Terra Nova", "Zephyria"];
            default:
                return [];
        }
    }
}
