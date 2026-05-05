<?php

namespace App\Repositories;

class GuestLinkRepository
{
    public function __construct(private \PDO $pdo)
    {
    }

    public function transferGuestData(string $guestUserId, string $currentUserId): array
    {
        $movedByTable = [];

        $this->pdo->beginTransaction();

        try {
            $guestPlayer = $this->findPlayerForUser($guestUserId);
            $currentPlayer = $this->findPlayerForUser($currentUserId);

            if (
                $guestPlayer
                && $currentPlayer
                && isset($guestPlayer['id'], $currentPlayer['id'])
                && (string) $guestPlayer['id'] !== (string) $currentPlayer['id']
            ) {
                $movedByTable['players.session_id'] = $this->updateIfPossible(
                    'players',
                    'session_id',
                    $guestUserId,
                    $currentUserId
                );
                $movedByTable['planets.owner_id'] = $this->updateIfPossible(
                    'planets',
                    'owner_id',
                    (string) $guestPlayer['id'],
                    (string) $currentPlayer['id']
                );
                $movedByTable['game_sessions.player_id'] = $this->updateIfPossible(
                    'game_sessions',
                    'player_id',
                    (string) $guestPlayer['id'],
                    (string) $currentPlayer['id']
                );
            } elseif ($guestPlayer) {
                $movedByTable['players.session_id'] = $this->updateIfPossible(
                    'players',
                    'session_id',
                    $guestUserId,
                    $currentUserId
                );
            } else {
                $movedByTable['players.session_id'] = 0;
            }

            $movedByTable['planets.session_id'] = $this->updateIfPossible(
                'planets',
                'session_id',
                $guestUserId,
                $currentUserId
            );
            $movedByTable['game_sessions.id'] = $this->updateIfPossible(
                'game_sessions',
                'id',
                $guestUserId,
                $currentUserId,
                true
            );
            $movedByTable['transactions.session_id'] = $this->updateIfPossible(
                'transactions',
                'session_id',
                $guestUserId,
                $currentUserId
            );

            $this->pdo->commit();
        } catch (\Throwable $exception) {
            if ($this->pdo->inTransaction()) {
                $this->pdo->rollBack();
            }

            throw $exception;
        }

        return [
            'moved_rows_by_table' => $movedByTable,
            'total_moved_rows' => array_sum(array_map('intval', $movedByTable)),
        ];
    }

    private function findPlayerForUser(string $userId): ?array
    {
        if (!$this->tableExists('players')) {
            return null;
        }

        $sql = 'SELECT * FROM players WHERE session_id = :user_id';
        if ($this->columnExists('players', 'id')) {
            $sql .= ' OR id = :id_match';
        }

        $stmt = $this->pdo->prepare($sql . ' LIMIT 1');
        $stmt->bindValue(':user_id', $userId);
        if (str_contains($sql, ':id_match')) {
            $stmt->bindValue(':id_match', $userId);
        }
        $stmt->execute();

        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        return $result ?: null;
    }

    private function updateIfPossible(
        string $table,
        string $column,
        string $fromValue,
        string $toValue,
        bool $skipOnTargetExists = false
    ): int {
        if ($fromValue === $toValue || !$this->tableExists($table) || !$this->columnExists($table, $column)) {
            return 0;
        }

        if ($skipOnTargetExists) {
            $check = $this->pdo->prepare("SELECT COUNT(*) FROM {$table} WHERE {$column} = ?");
            $check->execute([$toValue]);
            if ((int) $check->fetchColumn() > 0) {
                return 0;
            }
        }

        $stmt = $this->pdo->prepare("UPDATE {$table} SET {$column} = ? WHERE {$column} = ?");
        $stmt->execute([$toValue, $fromValue]);

        return (int) $stmt->rowCount();
    }

    private function tableExists(string $table): bool
    {
        $driver = $this->pdo->getAttribute(\PDO::ATTR_DRIVER_NAME);
        if ($driver === 'sqlite') {
            $stmt = $this->pdo->prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?");
            $stmt->execute([$table]);
            return (bool) $stmt->fetchColumn();
        }

        $stmt = $this->pdo->prepare('SHOW TABLES LIKE ?');
        $stmt->execute([$table]);
        return (bool) $stmt->fetchColumn();
    }

    private function columnExists(string $table, string $column): bool
    {
        $driver = $this->pdo->getAttribute(\PDO::ATTR_DRIVER_NAME);
        if ($driver === 'sqlite') {
            $stmt = $this->pdo->query("PRAGMA table_info({$table})");
            $columns = $stmt ? $stmt->fetchAll(\PDO::FETCH_ASSOC) : [];
            foreach ($columns as $definition) {
                if (($definition['name'] ?? null) === $column) {
                    return true;
                }
            }
            return false;
        }

        $stmt = $this->pdo->prepare("SHOW COLUMNS FROM {$table} LIKE ?");
        $stmt->execute([$column]);
        return (bool) $stmt->fetchColumn();
    }
}
