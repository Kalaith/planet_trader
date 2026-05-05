<?php

namespace App\Repositories;

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
            'profit_loss' => -$amount,
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
