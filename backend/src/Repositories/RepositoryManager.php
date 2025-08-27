<?php

namespace App\Repositories;

use App\Database\Connection;

/**
 * Repository Manager - Handles repository instances and dependency injection
 */
class RepositoryManager 
{
    private $pdo;
    private $repositories = [];
    
    public function __construct($pdo = null) 
    {
        $this->pdo = $pdo ?? Connection::getInstance()->getPdo();
    }
    
    /**
     * Get Planet Repository
     */
    public function planets(): PlanetRepository 
    {
        if (!isset($this->repositories['planets'])) {
            $this->repositories['planets'] = new PlanetRepository($this->pdo);
        }
        return $this->repositories['planets'];
    }
    
    /**
     * Get Game Session Repository
     */
    public function sessions(): GameSessionRepository 
    {
        if (!isset($this->repositories['sessions'])) {
            $this->repositories['sessions'] = new GameSessionRepository($this->pdo);
        }
        return $this->repositories['sessions'];
    }
    
    /**
     * Get Planet Type Repository
     */
    public function planetTypes(): PlanetTypeRepository 
    {
        if (!isset($this->repositories['planetTypes'])) {
            $this->repositories['planetTypes'] = new PlanetTypeRepository($this->pdo);
        }
        return $this->repositories['planetTypes'];
    }
    
    /**
     * Get Species Repository
     */
    public function species(): SpeciesRepository 
    {
        if (!isset($this->repositories['species'])) {
            $this->repositories['species'] = new SpeciesRepository($this->pdo);
        }
        return $this->repositories['species'];
    }
    
    /**
     * Get Tool Repository
     */
    public function tools(): ToolRepository 
    {
        if (!isset($this->repositories['tools'])) {
            $this->repositories['tools'] = new ToolRepository($this->pdo);
        }
        return $this->repositories['tools'];
    }
    
    /**
     * Get Transaction Repository
     */
    public function transactions(): TransactionRepository 
    {
        if (!isset($this->repositories['transactions'])) {
            $this->repositories['transactions'] = new TransactionRepository($this->pdo);
        }
        return $this->repositories['transactions'];
    }
    
    /**
     * Get Planet Names Repository
     */
    public function planetNames(): PlanetNameRepository 
    {
        if (!isset($this->repositories['planetNames'])) {
            $this->repositories['planetNames'] = new PlanetNameRepository($this->pdo);
        }
        return $this->repositories['planetNames'];
    }
    
    /**
     * Get Player Repository
     */
    public function players(): PlayerRepository 
    {
        if (!isset($this->repositories['players'])) {
            $this->repositories['players'] = new PlayerRepository($this->pdo);
        }
        return $this->repositories['players'];
    }
    
    /**
     * Begin a database transaction across all repositories
     */
    public function beginTransaction(): bool 
    {
        return $this->pdo->beginTransaction();
    }
    
    /**
     * Commit transaction
     */
    public function commit(): bool 
    {
        return $this->pdo->commit();
    }
    
    /**
     * Rollback transaction
     */
    public function rollback(): bool 
    {
        return $this->pdo->rollback();
    }
}

/**
 * Repository for Transaction model
 */
class TransactionRepository extends BaseRepository 
{
    protected $table = 'transactions';
    protected $fillable = [
        'session_id', 'planet_id', 'transaction_type', 'amount', 
        'profit_loss', 'tools_used'
    ];
    
    /**
     * Record a buy transaction
     */
    public function recordBuy(string $sessionId, int $planetId, int $amount, array $toolsUsed = []): int 
    {
        return $this->create([
            'session_id' => $sessionId,
            'planet_id' => $planetId,
            'transaction_type' => 'buy',
            'amount' => $amount,
            'profit_loss' => -$amount, // Negative because it's a purchase
            'tools_used' => $this->encodeJson($toolsUsed)
        ]);
    }
    
    /**
     * Record a sell transaction
     */
    public function recordSell(string $sessionId, int $planetId, int $amount, int $profit, array $toolsUsed = []): int 
    {
        return $this->create([
            'session_id' => $sessionId,
            'planet_id' => $planetId,
            'transaction_type' => 'sell',
            'amount' => $amount,
            'profit_loss' => $profit,
            'tools_used' => $this->encodeJson($toolsUsed)
        ]);
    }
    
    /**
     * Get transaction history for a session
     */
    public function getSessionHistory(string $sessionId): array 
    {
        $sql = "
            SELECT t.*, p.name as planet_name, p.discovery_order
            FROM transactions t
            LEFT JOIN planets p ON t.planet_id = p.id
            WHERE t.session_id = ?
            ORDER BY t.created_at DESC
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        foreach ($results as &$transaction) {
            $transaction['tools_used'] = $this->decodeJson($transaction['tools_used']);
        }
        
        return $results;
    }
    
    /**
     * Get transactions for a specific planet
     */
    public function getPlanetHistory(int $planetId): array 
    {
        $transactions = $this->findBy(['planet_id' => $planetId], ['created_at' => 'ASC']);
        
        foreach ($transactions as &$transaction) {
            $transaction['tools_used'] = $this->decodeJson($transaction['tools_used']);
        }
        
        return $transactions;
    }
    
    /**
     * Get session transaction statistics
     */
    public function getSessionStats(string $sessionId): array 
    {
        $sql = "
            SELECT 
                COUNT(*) as total_transactions,
                COUNT(CASE WHEN transaction_type = 'buy' THEN 1 END) as total_purchases,
                COUNT(CASE WHEN transaction_type = 'sell' THEN 1 END) as total_sales,
                COALESCE(SUM(CASE WHEN transaction_type = 'buy' THEN amount ELSE 0 END), 0) as total_spent,
                COALESCE(SUM(CASE WHEN transaction_type = 'sell' THEN amount ELSE 0 END), 0) as total_earned,
                COALESCE(SUM(profit_loss), 0) as net_profit,
                COALESCE(AVG(CASE WHEN transaction_type = 'sell' THEN profit_loss ELSE NULL END), 0) as avg_profit_per_sale
            FROM transactions 
            WHERE session_id = ?
        ";
        
        $stmt = $this->query($sql, [$sessionId]);
        return $stmt->fetch(\PDO::FETCH_ASSOC);
    }
}

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
        // Simple level calculation: 1000 XP per level
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
