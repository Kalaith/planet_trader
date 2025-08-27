<?php

namespace App\Repositories;

/**
 * Repository for GameSession model
 */
class GameSessionRepository extends BaseRepository 
{
    protected $table = 'game_sessions';
    protected $fillable = [
        'id', 'player_id', 'current_credits', 'current_planet_id',
        'planets_visited', 'total_profit', 'session_data', 'is_active'
    ];
    
    /**
     * Find active sessions for a player
     */
    public function findActiveByPlayer(int $playerId): array 
    {
        return $this->findBy([
            'player_id' => $playerId,
            'is_active' => true
        ], ['last_activity' => 'DESC']);
    }
    
    /**
     * Find a session by ID with player details
     */
    public function findWithPlayer(string $sessionId): ?array 
    {
        $sql = "
            SELECT gs.*, p.username, p.email, p.level
            FROM game_sessions gs
            LEFT JOIN players p ON gs.player_id = p.id
            WHERE gs.id = ?
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$result) {
            return null;
        }
        
        // Decode JSON session data
        $result['session_data'] = $this->decodeJson($result['session_data']);
        
        return $result;
    }
    
    /**
     * Create a new game session
     */
    public function createSession(string $sessionId, ?int $playerId = null, int $startingCredits = 10000): int 
    {
        $sessionData = [
            'game_started' => date('Y-m-d H:i:s'),
            'difficulty' => 'normal',
            'achievements' => []
        ];
        
        return $this->create([
            'id' => $sessionId,
            'player_id' => $playerId,
            'current_credits' => $startingCredits,
            'planets_visited' => 0,
            'total_profit' => 0,
            'session_data' => $this->encodeJson($sessionData),
            'is_active' => true
        ]);
    }
    
    /**
     * Update session credits
     */
    public function updateCredits(string $sessionId, int $newCredits): bool 
    {
        return $this->update($sessionId, ['current_credits' => $newCredits]);
    }
    
    /**
     * Add profit to session
     */
    public function addProfit(string $sessionId, int $profit): bool 
    {
        $sql = "
            UPDATE game_sessions 
            SET total_profit = total_profit + ?,
                last_activity = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        $stmt = $this->query($sql, [$profit, $sessionId]);
        return $stmt->rowCount() > 0;
    }
    
    /**
     * Increment planets visited
     */
    public function incrementPlanetsVisited(string $sessionId): bool 
    {
        $sql = "
            UPDATE game_sessions 
            SET planets_visited = planets_visited + 1,
                last_activity = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        return $stmt->rowCount() > 0;
    }
    
    /**
     * Set current planet
     */
    public function setCurrentPlanet(string $sessionId, ?int $planetId): bool 
    {
        return $this->update($sessionId, [
            'current_planet_id' => $planetId
        ]);
    }
    
    /**
     * Update session data
     */
    public function updateSessionData(string $sessionId, array $data): bool 
    {
        return $this->update($sessionId, [
            'session_data' => $this->encodeJson($data)
        ]);
    }
    
    /**
     * Merge data into existing session data
     */
    public function mergeSessionData(string $sessionId, array $newData): bool 
    {
        $session = $this->find($sessionId);
        if (!$session) {
            return false;
        }
        
        $existingData = $this->decodeJson($session['session_data']) ?? [];
        $mergedData = array_merge($existingData, $newData);
        
        return $this->updateSessionData($sessionId, $mergedData);
    }
    
    /**
     * End a game session
     */
    public function endSession(string $sessionId): bool 
    {
        return $this->update($sessionId, [
            'is_active' => false
        ]);
    }
    
    /**
     * Update last activity timestamp
     */
    public function updateActivity(string $sessionId): bool 
    {
        $sql = "UPDATE game_sessions SET last_activity = CURRENT_TIMESTAMP WHERE id = ?";
        $stmt = $this->query($sql, [$sessionId]);
        return $stmt->rowCount() > 0;
    }
    
    /**
     * Get session statistics
     */
    public function getStats(string $sessionId): ?array 
    {
        $session = $this->find($sessionId);
        if (!$session) {
            return null;
        }
        
        // Get additional stats from related tables
        $sql = "
            SELECT 
                COUNT(t.id) as total_transactions,
                COALESCE(SUM(CASE WHEN t.transaction_type = 'buy' THEN t.amount ELSE 0 END), 0) as total_purchases,
                COALESCE(SUM(CASE WHEN t.transaction_type = 'sell' THEN t.amount ELSE 0 END), 0) as total_sales,
                COALESCE(SUM(t.profit_loss), 0) as net_profit
            FROM transactions t
            WHERE t.session_id = ?
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        $transactionStats = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        return [
            'session_id' => $session['id'],
            'current_credits' => $session['current_credits'],
            'planets_visited' => $session['planets_visited'],
            'total_profit' => $session['total_profit'],
            'is_active' => (bool) $session['is_active'],
            'started_at' => $session['started_at'],
            'last_activity' => $session['last_activity'],
            'transactions' => $transactionStats
        ];
    }
    
    /**
     * Find sessions by ID (override to handle string IDs)
     */
    public function find($id): ?array 
    {
        $stmt = $this->pdo->prepare("SELECT * FROM {$this->table} WHERE id = ?");
        $stmt->execute([$id]);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$result) {
            return null;
        }
        
        // Decode JSON session data
        $result['session_data'] = $this->decodeJson($result['session_data']);
        
        return $result;
    }
    
    /**
     * Update by ID (override to handle string IDs)
     */
    public function update($id, array $data): bool 
    {
        $data = $this->filterFillable($data);
        
        if (empty($data)) {
            return false;
        }
        
        $setParts = [];
        foreach (array_keys($data) as $field) {
            $setParts[] = "{$field} = ?";
        }
        
        $sql = "UPDATE {$this->table} SET " . implode(', ', $setParts) . " WHERE id = ?";
        
        $params = array_values($data);
        $params[] = $id;
        
        $stmt = $this->pdo->prepare($sql);
        return $stmt->execute($params);
    }
    
    /**
     * Clean up old inactive sessions
     */
    public function cleanupOldSessions(int $daysOld = 30): int 
    {
        $sql = "
            DELETE FROM game_sessions 
            WHERE is_active = FALSE 
            AND last_activity < DATE_SUB(NOW(), INTERVAL ? DAY)
        ";
        
        $stmt = $this->query($sql, [$daysOld]);
        return $stmt->rowCount();
    }
}
