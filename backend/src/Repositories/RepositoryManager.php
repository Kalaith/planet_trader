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
