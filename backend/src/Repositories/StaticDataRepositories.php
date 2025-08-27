<?php

namespace App\Repositories;

/**
 * Repository for PlanetType model
 */
class PlanetTypeRepository extends BaseRepository 
{
    protected $table = 'planet_types';
    protected $fillable = [
        'name', 'description', 'base_value_multiplier', 'rarity_weight', 
        'color_scheme', 'atmosphere_types'
    ];
    
    /**
     * Get all planet types with decoded JSON fields
     */
    public function findAllWithDetails(): array 
    {
        $types = $this->findAll(['name' => 'ASC']);
        
        foreach ($types as &$type) {
            $type['color_scheme'] = $this->decodeJson($type['color_scheme']);
            $type['atmosphere_types'] = $this->decodeJson($type['atmosphere_types']);
        }
        
        return $types;
    }
    
    /**
     * Get weighted random planet type
     */
    public function getRandomWeighted(): ?array 
    {
        $sql = "
            SELECT *, (rarity_weight * RAND()) as weighted_random
            FROM planet_types 
            ORDER BY weighted_random DESC 
            LIMIT 1
        ";
        
        $stmt = $this->query($sql);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$result) {
            return null;
        }
        
        $result['color_scheme'] = $this->decodeJson($result['color_scheme']);
        $result['atmosphere_types'] = $this->decodeJson($result['atmosphere_types']);
        
        return $result;
    }
    
    /**
     * Find planet types by rarity
     */
    public function findByRarity(int $rarity): array 
    {
        $types = $this->findBy(['rarity_weight' => $rarity]);
        
        foreach ($types as &$type) {
            $type['color_scheme'] = $this->decodeJson($type['color_scheme']);
            $type['atmosphere_types'] = $this->decodeJson($type['atmosphere_types']);
        }
        
        return $types;
    }
    
    /**
     * Get planet type by name
     */
    public function findByName(string $name): ?array 
    {
        $type = $this->findOneBy(['name' => $name]);
        
        if ($type) {
            $type['color_scheme'] = $this->decodeJson($type['color_scheme']);
            $type['atmosphere_types'] = $this->decodeJson($type['atmosphere_types']);
        }
        
        return $type;
    }
    
    /**
     * Get rare planet types (rarity 1)
     */
    public function findRare(): array 
    {
        return $this->findByRarity(1);
    }
    
    /**
     * Get common planet types (rarity 3)
     */
    public function findCommon(): array 
    {
        return $this->findByRarity(3);
    }
    
    /**
     * Update color scheme for a planet type
     */
    public function updateColorScheme(int $id, array $colorScheme): bool 
    {
        return $this->update($id, [
            'color_scheme' => $this->encodeJson($colorScheme)
        ]);
    }
    
    /**
     * Update atmosphere types for a planet type
     */
    public function updateAtmosphereTypes(int $id, array $atmosphereTypes): bool 
    {
        return $this->update($id, [
            'atmosphere_types' => $this->encodeJson($atmosphereTypes)
        ]);
    }
}

/**
 * Repository for Species model
 */
class SpeciesRepository extends BaseRepository 
{
    protected $table = 'species';
    protected $fillable = [
        'name', 'type', 'description', 'preferred_planet_types', 
        'appearance', 'value_multiplier', 'rarity_weight'
    ];
    
    /**
     * Get all species with decoded JSON fields
     */
    public function findAllWithDetails(): array 
    {
        $species = $this->findAll(['name' => 'ASC']);
        
        foreach ($species as &$spec) {
            $spec['preferred_planet_types'] = $this->decodeJson($spec['preferred_planet_types']);
            $spec['appearance'] = $this->decodeJson($spec['appearance']);
        }
        
        return $species;
    }
    
    /**
     * Get weighted random species
     */
    public function getRandomWeighted(): ?array 
    {
        $sql = "
            SELECT *, (rarity_weight * RAND()) as weighted_random
            FROM species 
            ORDER BY weighted_random DESC 
            LIMIT 1
        ";
        
        $stmt = $this->query($sql);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$result) {
            return null;
        }
        
        $result['preferred_planet_types'] = $this->decodeJson($result['preferred_planet_types']);
        $result['appearance'] = $this->decodeJson($result['appearance']);
        
        return $result;
    }
    
    /**
     * Find species compatible with a planet type
     */
    public function findCompatibleWithPlanetType(string $planetTypeName): array 
    {
        $sql = "
            SELECT * FROM species 
            WHERE JSON_CONTAINS(preferred_planet_types, ?) 
            OR JSON_CONTAINS(preferred_planet_types, '\"all\"')
        ";
        
        $stmt = $this->query($sql, [json_encode($planetTypeName)]);
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        foreach ($results as &$species) {
            $species['preferred_planet_types'] = $this->decodeJson($species['preferred_planet_types']);
            $species['appearance'] = $this->decodeJson($species['appearance']);
        }
        
        return $results;
    }
    
    /**
     * Find species by type
     */
    public function findByType(string $type): array 
    {
        $species = $this->findBy(['type' => $type]);
        
        foreach ($species as &$spec) {
            $spec['preferred_planet_types'] = $this->decodeJson($spec['preferred_planet_types']);
            $spec['appearance'] = $this->decodeJson($spec['appearance']);
        }
        
        return $species;
    }
    
    /**
     * Get species by name
     */
    public function findByName(string $name): ?array 
    {
        $species = $this->findOneBy(['name' => $name]);
        
        if ($species) {
            $species['preferred_planet_types'] = $this->decodeJson($species['preferred_planet_types']);
            $species['appearance'] = $this->decodeJson($species['appearance']);
        }
        
        return $species;
    }
}

/**
 * Repository for Tool model
 */
class ToolRepository extends BaseRepository 
{
    protected $table = 'tools';
    protected $fillable = [
        'name', 'description', 'effect_type', 'effect_value', 'cost',
        'unlock_requirements', 'applicable_planet_types', 'research_tier'
    ];
    
    /**
     * Get all tools with decoded JSON fields
     */
    public function findAllWithDetails(): array 
    {
        $tools = $this->findAll(['research_tier' => 'ASC', 'cost' => 'ASC']);
        
        foreach ($tools as &$tool) {
            $tool['unlock_requirements'] = $this->decodeJson($tool['unlock_requirements']);
            $tool['applicable_planet_types'] = $this->decodeJson($tool['applicable_planet_types']);
        }
        
        return $tools;
    }
    
    /**
     * Find tools by research tier
     */
    public function findByTier(int $tier): array 
    {
        $tools = $this->findBy(['research_tier' => $tier], ['cost' => 'ASC']);
        
        foreach ($tools as &$tool) {
            $tool['unlock_requirements'] = $this->decodeJson($tool['unlock_requirements']);
            $tool['applicable_planet_types'] = $this->decodeJson($tool['applicable_planet_types']);
        }
        
        return $tools;
    }
    
    /**
     * Find unlocked tools for a player (based on session stats)
     */
    public function findUnlockedForSession(array $sessionStats): array 
    {
        $allTools = $this->findAllWithDetails();
        $unlockedTools = [];
        
        foreach ($allTools as $tool) {
            if ($this->isToolUnlocked($tool, $sessionStats)) {
                $unlockedTools[] = $tool;
            }
        }
        
        return $unlockedTools;
    }
    
    /**
     * Find tools applicable to a planet type
     */
    public function findApplicableToPlanetType(string $planetTypeName): array 
    {
        $sql = "
            SELECT * FROM tools 
            WHERE JSON_CONTAINS(applicable_planet_types, ?) 
            OR JSON_CONTAINS(applicable_planet_types, '\"all\"')
        ";
        
        $stmt = $this->query($sql, [json_encode($planetTypeName)]);
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        foreach ($results as &$tool) {
            $tool['unlock_requirements'] = $this->decodeJson($tool['unlock_requirements']);
            $tool['applicable_planet_types'] = $this->decodeJson($tool['applicable_planet_types']);
        }
        
        return $results;
    }
    
    /**
     * Find tools by effect type
     */
    public function findByEffectType(string $effectType): array 
    {
        $tools = $this->findBy(['effect_type' => $effectType]);
        
        foreach ($tools as &$tool) {
            $tool['unlock_requirements'] = $this->decodeJson($tool['unlock_requirements']);
            $tool['applicable_planet_types'] = $this->decodeJson($tool['applicable_planet_types']);
        }
        
        return $tools;
    }
    
    /**
     * Check if a tool is unlocked for a player
     */
    private function isToolUnlocked(array $tool, array $sessionStats): bool 
    {
        $requirements = $tool['unlock_requirements'];
        
        if (empty($requirements)) {
            return true; // No requirements means always unlocked
        }
        
        foreach ($requirements as $requirement => $value) {
            switch ($requirement) {
                case 'planets_visited':
                    if (($sessionStats['planets_visited'] ?? 0) < $value) {
                        return false;
                    }
                    break;
                    
                case 'credits_earned':
                    if (($sessionStats['total_profit'] ?? 0) < $value) {
                        return false;
                    }
                    break;
                    
                case 'research_tier':
                    // Could implement tier unlocking logic here
                    break;
            }
        }
        
        return true;
    }
}
