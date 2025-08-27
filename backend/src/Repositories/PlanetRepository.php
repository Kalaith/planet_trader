<?php

namespace App\Repositories;

/**
 * Repository for Planet model
 */
class PlanetRepository extends BaseRepository 
{
    protected $table = 'planets';
    protected $fillable = [
        'session_id', 'name', 'planet_type_id', 'size', 'atmosphere',
        'base_value', 'current_value', 'species_id', 'position_x', 'position_y',
        'tools_applied', 'discovery_order', 'is_sold', 'sold_for', 'sold_at'
    ];
    
    /**
     * Find planets for a specific game session
     */
    public function findBySession(string $sessionId): array 
    {
        return $this->findBy(['session_id' => $sessionId], ['discovery_order' => 'ASC']);
    }
    
    /**
     * Find unsold planets for a session
     */
    public function findUnsoldBySession(string $sessionId): array 
    {
        return $this->findBy([
            'session_id' => $sessionId,
            'is_sold' => false
        ], ['discovery_order' => 'ASC']);
    }
    
    /**
     * Find planets by type
     */
    public function findByType(int $planetTypeId): array 
    {
        return $this->findBy(['planet_type_id' => $planetTypeId]);
    }
    
    /**
     * Find planets with specific species
     */
    public function findBySpecies(int $speciesId): array 
    {
        return $this->findBy(['species_id' => $speciesId]);
    }
    
    /**
     * Get planet with full type and species information
     */
    public function findWithDetails(int $id): ?array 
    {
        $sql = "
            SELECT p.*, 
                   pt.name as type_name, pt.description as type_description,
                   pt.color_scheme, pt.atmosphere_types,
                   s.name as species_name, s.description as species_description,
                   s.appearance as species_appearance, s.value_multiplier as species_multiplier
            FROM planets p
            LEFT JOIN planet_types pt ON p.planet_type_id = pt.id
            LEFT JOIN species s ON p.species_id = s.id
            WHERE p.id = ?
        ";
        
        $stmt = $this->query($sql, [$id]);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$result) {
            return null;
        }
        
        // Decode JSON fields
        $result['tools_applied'] = $this->decodeJson($result['tools_applied']);
        $result['color_scheme'] = $this->decodeJson($result['color_scheme']);
        $result['atmosphere_types'] = $this->decodeJson($result['atmosphere_types']);
        $result['species_appearance'] = $this->decodeJson($result['species_appearance']);
        
        return $result;
    }
    
    /**
     * Get planets with full details for a session
     */
    public function findSessionPlanetsWithDetails(string $sessionId): array 
    {
        $sql = "
            SELECT p.*, 
                   pt.name as type_name, pt.description as type_description,
                   pt.color_scheme, pt.atmosphere_types,
                   s.name as species_name, s.description as species_description,
                   s.appearance as species_appearance, s.value_multiplier as species_multiplier
            FROM planets p
            LEFT JOIN planet_types pt ON p.planet_type_id = pt.id
            LEFT JOIN species s ON p.species_id = s.id
            WHERE p.session_id = ?
            ORDER BY p.discovery_order ASC
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        // Decode JSON fields for each planet
        foreach ($results as &$planet) {
            $planet['tools_applied'] = $this->decodeJson($planet['tools_applied']);
            $planet['color_scheme'] = $this->decodeJson($planet['color_scheme']);
            $planet['atmosphere_types'] = $this->decodeJson($planet['atmosphere_types']);
            $planet['species_appearance'] = $this->decodeJson($planet['species_appearance']);
        }
        
        return $results;
    }
    
    /**
     * Mark a planet as sold
     */
    public function markAsSold(int $id, int $soldFor): bool 
    {
        return $this->update($id, [
            'is_sold' => true,
            'sold_for' => $soldFor,
            'sold_at' => date('Y-m-d H:i:s')
        ]);
    }
    
    /**
     * Apply tools to a planet
     */
    public function applyTools(int $id, array $toolsApplied): bool 
    {
        return $this->update($id, [
            'tools_applied' => $this->encodeJson($toolsApplied)
        ]);
    }
    
    /**
     * Update planet value
     */
    public function updateValue(int $id, int $newValue): bool 
    {
        return $this->update($id, ['current_value' => $newValue]);
    }
    
    /**
     * Get statistics for a session
     */
    public function getSessionStats(string $sessionId): array 
    {
        $sql = "
            SELECT 
                COUNT(*) as total_planets,
                COUNT(CASE WHEN is_sold = true THEN 1 END) as sold_planets,
                COALESCE(SUM(CASE WHEN is_sold = true THEN sold_for ELSE 0 END), 0) as total_sales,
                COALESCE(AVG(CASE WHEN is_sold = true THEN sold_for ELSE NULL END), 0) as avg_sale_price,
                COALESCE(MAX(sold_for), 0) as max_sale_price
            FROM planets 
            WHERE session_id = ?
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        return $stmt->fetch(\PDO::FETCH_ASSOC);
    }
    
    /**
     * Get next discovery order for a session
     */
    public function getNextDiscoveryOrder(string $sessionId): int 
    {
        $sql = "SELECT COALESCE(MAX(discovery_order), 0) + 1 FROM planets WHERE session_id = ?";
        $stmt = $this->query($sql, [$sessionId]);
        return (int) $stmt->fetchColumn();
    }
    
    /**
     * Create a new planet with auto-generated discovery order
     */
    public function createPlanet(string $sessionId, array $planetData): int 
    {
        $planetData['session_id'] = $sessionId;
        $planetData['discovery_order'] = $this->getNextDiscoveryOrder($sessionId);
        $planetData['is_sold'] = false;
        
        // Ensure JSON fields are properly encoded
        if (isset($planetData['tools_applied'])) {
            $planetData['tools_applied'] = $this->encodeJson($planetData['tools_applied']);
        }
        
        return $this->create($planetData);
    }
}
