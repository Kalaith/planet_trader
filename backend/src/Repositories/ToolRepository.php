<?php

namespace App\Repositories;

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
            return true;
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
                    break;
            }
        }

        return true;
    }
}
