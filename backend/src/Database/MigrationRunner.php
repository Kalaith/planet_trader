<?php

namespace App\Database;

use App\Database\Migrations\CreatePlanetTraderTables;

/**
 * Database migration runner
 */
class MigrationRunner 
{
    private $pdo;
    
    public function __construct($pdo) 
    {
        $this->pdo = $pdo;
        $this->createMigrationsTable();
    }
    
    /**
     * Create migrations tracking table
     */
    private function createMigrationsTable(): void 
    {
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS migrations (
                id INT AUTO_INCREMENT PRIMARY KEY,
                migration_name VARCHAR(255) NOT NULL UNIQUE,
                executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ");
    }
    
    /**
     * Run all pending migrations
     */
    public function runMigrations(): array 
    {
        $results = [];
        
        // Define migrations in order
        $migrations = [
            'create_planet_trader_tables' => CreatePlanetTraderTables::class
        ];
        
        foreach ($migrations as $name => $class) {
            if (!$this->isMigrationExecuted($name)) {
                try {
                    $migration = new $class($this->pdo);
                    $migration->up();
                    $this->markMigrationExecuted($name);
                    $results[] = "✅ Migration '{$name}' executed successfully";
                } catch (\Exception $e) {
                    $results[] = "❌ Migration '{$name}' failed: " . $e->getMessage();
                    throw $e; // Stop on first failure
                }
            } else {
                $results[] = "⏭️ Migration '{$name}' already executed";
            }
        }
        
        return $results;
    }
    
    /**
     * Rollback the last migration
     */
    public function rollbackLastMigration(): string 
    {
        $stmt = $this->pdo->prepare("
            SELECT migration_name FROM migrations 
            ORDER BY executed_at DESC 
            LIMIT 1
        ");
        $stmt->execute();
        $lastMigration = $stmt->fetchColumn();
        
        if (!$lastMigration) {
            return "No migrations to rollback";
        }
        
        $migrations = [
            'create_planet_trader_tables' => CreatePlanetTraderTables::class
        ];
        
        if (isset($migrations[$lastMigration])) {
            $class = $migrations[$lastMigration];
            $migration = new $class($this->pdo);
            $migration->down();
            
            $stmt = $this->pdo->prepare("DELETE FROM migrations WHERE migration_name = ?");
            $stmt->execute([$lastMigration]);
            
            return "✅ Rolled back migration: {$lastMigration}";
        }
        
        return "❌ Unknown migration: {$lastMigration}";
    }
    
    /**
     * Check if a migration has been executed
     */
    private function isMigrationExecuted(string $name): bool 
    {
        $stmt = $this->pdo->prepare("SELECT COUNT(*) FROM migrations WHERE migration_name = ?");
        $stmt->execute([$name]);
        return $stmt->fetchColumn() > 0;
    }
    
    /**
     * Mark a migration as executed
     */
    private function markMigrationExecuted(string $name): void 
    {
        $stmt = $this->pdo->prepare("INSERT INTO migrations (migration_name) VALUES (?)");
        $stmt->execute([$name]);
    }
    
    /**
     * Get migration status
     */
    public function getStatus(): array 
    {
        $stmt = $this->pdo->prepare("
            SELECT migration_name, executed_at 
            FROM migrations 
            ORDER BY executed_at DESC
        ");
        $stmt->execute();
        return $stmt->fetchAll(\PDO::FETCH_ASSOC);
    }
}
