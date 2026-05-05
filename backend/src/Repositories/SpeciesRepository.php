<?php

namespace App\Repositories;

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
