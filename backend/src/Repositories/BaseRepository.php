<?php

namespace App\Repositories;

/**
 * Base repository class providing common database operations
 */
abstract class BaseRepository 
{
    protected $pdo;
    protected $table;
    protected $fillable = [];
    
    public function __construct($pdo) 
    {
        $this->pdo = $pdo;
    }
    
    /**
     * Find a record by ID
     */
    public function find(int $id): ?array 
    {
        $stmt = $this->pdo->prepare("SELECT * FROM {$this->table} WHERE id = ?");
        $stmt->execute([$id]);
        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        return $result ?: null;
    }
    
    /**
     * Find multiple records by criteria
     */
    public function findBy(array $criteria, array $orderBy = [], int $limit = null, int $offset = 0): array 
    {
        $whereClause = '';
        $params = [];
        
        if (!empty($criteria)) {
            $conditions = [];
            foreach ($criteria as $field => $value) {
                if (is_array($value)) {
                    $placeholders = str_repeat('?,', count($value) - 1) . '?';
                    $conditions[] = "{$field} IN ({$placeholders})";
                    $params = array_merge($params, $value);
                } else {
                    $conditions[] = "{$field} = ?";
                    $params[] = $value;
                }
            }
            $whereClause = 'WHERE ' . implode(' AND ', $conditions);
        }
        
        $orderClause = '';
        if (!empty($orderBy)) {
            $orderParts = [];
            foreach ($orderBy as $field => $direction) {
                $orderParts[] = "{$field} " . strtoupper($direction);
            }
            $orderClause = 'ORDER BY ' . implode(', ', $orderParts);
        }
        
        $limitClause = $limit ? "LIMIT {$limit} OFFSET {$offset}" : '';
        
        $sql = "SELECT * FROM {$this->table} {$whereClause} {$orderClause} {$limitClause}";
        $stmt = $this->pdo->prepare($sql);
        $stmt->execute($params);
        
        return $stmt->fetchAll(\PDO::FETCH_ASSOC);
    }
    
    /**
     * Find one record by criteria
     */
    public function findOneBy(array $criteria): ?array 
    {
        $results = $this->findBy($criteria, [], 1);
        return !empty($results) ? $results[0] : null;
    }
    
    /**
     * Get all records
     */
    public function findAll(array $orderBy = [], int $limit = null, int $offset = 0): array 
    {
        return $this->findBy([], $orderBy, $limit, $offset);
    }
    
    /**
     * Create a new record
     */
    public function create(array $data): int 
    {
        $data = $this->filterFillable($data);
        
        if (empty($data)) {
            throw new \InvalidArgumentException('No valid data provided for creation');
        }
        
        $fields = array_keys($data);
        $placeholders = str_repeat('?,', count($fields) - 1) . '?';
        
        $sql = "INSERT INTO {$this->table} (" . implode(', ', $fields) . ") VALUES ({$placeholders})";
        
        $stmt = $this->pdo->prepare($sql);
        $stmt->execute(array_values($data));
        
        return (int) $this->pdo->lastInsertId();
    }
    
    /**
     * Update a record by ID
     */
    public function update(int $id, array $data): bool 
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
     * Delete a record by ID
     */
    public function delete(int $id): bool 
    {
        $stmt = $this->pdo->prepare("DELETE FROM {$this->table} WHERE id = ?");
        return $stmt->execute([$id]);
    }
    
    /**
     * Count records matching criteria
     */
    public function count(array $criteria = []): int 
    {
        $whereClause = '';
        $params = [];
        
        if (!empty($criteria)) {
            $conditions = [];
            foreach ($criteria as $field => $value) {
                $conditions[] = "{$field} = ?";
                $params[] = $value;
            }
            $whereClause = 'WHERE ' . implode(' AND ', $conditions);
        }
        
        $stmt = $this->pdo->prepare("SELECT COUNT(*) FROM {$this->table} {$whereClause}");
        $stmt->execute($params);
        
        return (int) $stmt->fetchColumn();
    }
    
    /**
     * Check if a record exists
     */
    public function exists(array $criteria): bool 
    {
        return $this->count($criteria) > 0;
    }
    
    /**
     * Begin database transaction
     */
    public function beginTransaction(): bool 
    {
        return $this->pdo->beginTransaction();
    }
    
    /**
     * Commit database transaction
     */
    public function commit(): bool 
    {
        return $this->pdo->commit();
    }
    
    /**
     * Rollback database transaction
     */
    public function rollback(): bool 
    {
        return $this->pdo->rollback();
    }
    
    /**
     * Execute a raw SQL query
     */
    protected function query(string $sql, array $params = []): \PDOStatement 
    {
        $stmt = $this->pdo->prepare($sql);
        $stmt->execute($params);
        return $stmt;
    }
    
    /**
     * Filter data to only include fillable fields
     */
    protected function filterFillable(array $data): array 
    {
        if (empty($this->fillable)) {
            return $data;
        }
        
        return array_intersect_key($data, array_flip($this->fillable));
    }
    
    /**
     * Handle JSON encoding for database storage
     */
    protected function encodeJson($data): string 
    {
        return json_encode($data, JSON_UNESCAPED_UNICODE);
    }
    
    /**
     * Handle JSON decoding from database
     */
    protected function decodeJson(?string $data): ?array 
    {
        if ($data === null) {
            return null;
        }
        
        $decoded = json_decode($data, true);
        return $decoded === null ? [] : $decoded;
    }
}
