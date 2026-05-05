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
