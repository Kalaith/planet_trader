<?php

namespace App\Services;

use App\Models\GameSession;
use App\Models\Planet;
use App\Models\Player;
use App\Models\Species;
use App\Repositories\LegacyGameStateRepository;

class GameStateService
{
    public function __construct(private LegacyGameStateRepository $repository)
    {
    }

    public function getPlayerBySession(string $sessionId): ?Player
    {
        return $this->repository->getPlayerBySession($sessionId);
    }

    public function createPlayer(string $sessionId): Player
    {
        return $this->repository->createPlayer($sessionId);
    }

    public function updatePlayer(Player $player): bool
    {
        return $this->repository->updatePlayer($player);
    }

    public function getPlanetById(string $planetId): ?Planet
    {
        return $this->repository->getPlanetById($planetId);
    }

    public function getPlanetsByOwner(string $playerId): array
    {
        return $this->repository->getPlanetsByOwner($playerId);
    }

    public function savePlanet(Planet $planet): bool
    {
        return $this->repository->savePlanet($planet);
    }

    public function updatePlanet(Planet $planet): bool
    {
        return $this->repository->updatePlanet($planet);
    }

    public function getSpeciesById(int $speciesId): ?Species
    {
        return $this->repository->getSpeciesById($speciesId);
    }

    public function getAllSpecies(): array
    {
        return $this->repository->getAllSpecies();
    }

    public function getCurrentGameSession(string $sessionId): ?GameSession
    {
        return $this->repository->getCurrentGameSession($sessionId);
    }

    public function startGameSession(string $sessionId): GameSession
    {
        return $this->repository->startGameSession($sessionId);
    }

    public function endGameSession(string $sessionId, int $finalCredits): bool
    {
        return $this->repository->endGameSession($sessionId, $finalCredits);
    }

    public function incrementPlanetsTraded(string $sessionId): bool
    {
        return $this->repository->incrementPlanetsTraded($sessionId);
    }

    public function getPlayerState(string $sessionId): array
    {
        return $this->repository->getPlayerState($sessionId);
    }
}
