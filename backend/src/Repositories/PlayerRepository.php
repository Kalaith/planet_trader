<?php

namespace App\Repositories;

/**
 * Repository for Player model
 */
class PlayerRepository extends BaseRepository
{
    protected $table = 'players';
    protected $fillable = [
        'username', 'email', 'credits', 'experience_points', 'level'
    ];

    /**
     * Find player by username
     */
    public function findByUsername(string $username): ?array
    {
        return $this->findOneBy(['username' => $username]);
    }

    /**
     * Find player by email
     */
    public function findByEmail(string $email): ?array
    {
        return $this->findOneBy(['email' => $email]);
    }

    /**
     * Add experience points and level up if needed
     */
    public function addExperience(int $playerId, int $points): bool
    {
        $player = $this->find($playerId);
        if (!$player) {
            return false;
        }

        $newExp = $player['experience_points'] + $points;
        $newLevel = $this->calculateLevel($newExp);

        return $this->update($playerId, [
            'experience_points' => $newExp,
            'level' => $newLevel
        ]);
    }

    /**
     * Calculate level based on experience points
     */
    private function calculateLevel(int $exp): int
    {
        return max(1, floor($exp / 1000) + 1);
    }

    /**
     * Get player statistics including sessions
     */
    public function getPlayerStats(int $playerId): array
    {
        $player = $this->find($playerId);
        if (!$player) {
            return [];
        }

        $sql = "
            SELECT
                COUNT(gs.id) as total_sessions,
                COUNT(CASE WHEN gs.is_active = TRUE THEN 1 END) as active_sessions,
                COALESCE(SUM(gs.total_profit), 0) as lifetime_profit,
                COALESCE(MAX(gs.total_profit), 0) as best_session_profit,
                COALESCE(SUM(gs.planets_visited), 0) as total_planets_visited
            FROM game_sessions gs
            WHERE gs.player_id = ?
        ";

        $stmt = $this->query($sql, [$playerId]);
        $sessionStats = $stmt->fetch(\PDO::FETCH_ASSOC);

        return array_merge($player, ['session_stats' => $sessionStats]);
    }
}
