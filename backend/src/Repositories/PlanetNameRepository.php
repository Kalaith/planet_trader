<?php

namespace App\Repositories;

/**
 * Repository for Planet Names
 */
class PlanetNameRepository extends BaseRepository
{
    protected $table = 'planet_names';
    protected $fillable = ['name', 'origin', 'category', 'is_used'];

    /**
     * Get a random unused planet name
     */
    public function getRandomUnused(): ?array
    {
        $sql = "
            SELECT * FROM planet_names
            WHERE is_used = FALSE
            ORDER BY RAND()
            LIMIT 1
        ";

        $stmt = $this->query($sql);
        return $stmt->fetch(\PDO::FETCH_ASSOC) ?: null;
    }

    /**
     * Mark a name as used
     */
    public function markAsUsed(int $id): bool
    {
        return $this->update($id, ['is_used' => true]);
    }

    /**
     * Get unused names by category
     */
    public function getUnusedByCategory(string $category): array
    {
        return $this->findBy([
            'category' => $category,
            'is_used' => false
        ]);
    }

    /**
     * Reset all names to unused (for testing/reset)
     */
    public function resetAllToUnused(): bool
    {
        $sql = "UPDATE planet_names SET is_used = FALSE";
        $stmt = $this->query($sql);
        return $stmt->rowCount() > 0;
    }
}
