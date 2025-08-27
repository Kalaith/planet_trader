<?php

namespace App\Services;

use App\Models\Player;
use App\Models\Planet;
use App\Models\Species;
use App\Models\GameSession;
use App\Utils\RandomUtils;

class GameStateService
{
    private \PDO $db;

    public function __construct(\PDO $db)
    {
        $this->db = $db;
    }

    /**
     * Get player by session ID
     */
    public function getPlayerBySession(string $sessionId): ?Player
    {
        $stmt = $this->db->prepare("SELECT * FROM players WHERE session_id = ?");
        $stmt->execute([$sessionId]);
        $data = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$data) {
            return null;
        }
        
        return Player::fromArray($data);
    }

    /**
     * Create a new player with session
     */
    public function createPlayer(string $sessionId): Player
    {
        $player = new Player();
        $player->id = RandomUtils::generateId('player');
        $player->sessionId = $sessionId;
        
        $stmt = $this->db->prepare("
            INSERT INTO players (id, session_id, credits, game_started, created_at, last_activity)
            VALUES (?, ?, ?, ?, ?, ?)
        ");
        
        $stmt->execute([
            $player->id,
            $player->sessionId,
            $player->credits,
            $player->gameStarted ? 1 : 0,
            $player->createdAt->format('Y-m-d H:i:s'),
            $player->lastActivity->format('Y-m-d H:i:s')
        ]);
        
        return $player;
    }

    /**
     * Update player data
     */
    public function updatePlayer(Player $player): bool
    {
        $stmt = $this->db->prepare("
            UPDATE players 
            SET credits = ?, current_planet_id = ?, game_started = ?, last_activity = ?
            WHERE id = ?
        ");
        
        return $stmt->execute([
            $player->credits,
            $player->currentPlanetId,
            $player->gameStarted ? 1 : 0,
            $player->lastActivity->format('Y-m-d H:i:s'),
            $player->id
        ]);
    }

    /**
     * Get planet by ID
     */
    public function getPlanetById(string $planetId): ?Planet
    {
        $stmt = $this->db->prepare("
            SELECT p.*, pt.name as type_name, pt.base_temp, pt.base_atmo, 
                   pt.base_water, pt.base_grav, pt.base_rad, pt.color as type_color
            FROM planets p
            JOIN planet_types pt ON p.type_id = pt.id
            WHERE p.id = ?
        ");
        
        $stmt->execute([$planetId]);
        $data = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$data) {
            return null;
        }
        
        return $this->hydratePlanetFromData($data);
    }

    /**
     * Get planets owned by a player
     */
    public function getPlanetsByOwner(string $playerId): array
    {
        $stmt = $this->db->prepare("
            SELECT p.*, pt.name as type_name, pt.base_temp, pt.base_atmo, 
                   pt.base_water, pt.base_grav, pt.base_rad, pt.color as type_color
            FROM planets p
            JOIN planet_types pt ON p.type_id = pt.id
            WHERE p.owner_id = ?
            ORDER BY p.created_at DESC
        ");
        
        $stmt->execute([$playerId]);
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        $planets = [];
        foreach ($results as $data) {
            $planets[] = $this->hydratePlanetFromData($data);
        }
        
        return $planets;
    }

    /**
     * Save a planet to the database
     */
    public function savePlanet(Planet $planet): bool
    {
        $stmt = $this->db->prepare("
            INSERT INTO planets (id, type_id, name, temperature, atmosphere, water, gravity, radiation, purchase_price, color, owner_id, created_at, sold_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ");
        
        return $stmt->execute([
            $planet->id,
            $planet->type->id,
            $planet->name,
            $planet->temperature,
            $planet->atmosphere,
            $planet->water,
            $planet->gravity,
            $planet->radiation,
            $planet->purchasePrice,
            $planet->color,
            $planet->ownerId,
            $planet->createdAt->format('Y-m-d H:i:s'),
            $planet->soldAt?->format('Y-m-d H:i:s')
        ]);
    }

    /**
     * Update planet data
     */
    public function updatePlanet(Planet $planet): bool
    {
        $stmt = $this->db->prepare("
            UPDATE planets 
            SET owner_id = ?, sold_at = ?, temperature = ?, atmosphere = ?, water = ?, gravity = ?, radiation = ?
            WHERE id = ?
        ");
        
        return $stmt->execute([
            $planet->ownerId,
            $planet->soldAt?->format('Y-m-d H:i:s'),
            $planet->temperature,
            $planet->atmosphere,
            $planet->water,
            $planet->gravity,
            $planet->radiation,
            $planet->id
        ]);
    }

    /**
     * Get species by ID
     */
    public function getSpeciesById(int $speciesId): ?Species
    {
        $stmt = $this->db->prepare("SELECT * FROM species WHERE id = ?");
        $stmt->execute([$speciesId]);
        $data = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$data) {
            return null;
        }
        
        return Species::fromArray($data);
    }

    /**
     * Get all species
     */
    public function getAllSpecies(): array
    {
        $stmt = $this->db->query("SELECT * FROM species ORDER BY name");
        $results = $stmt->fetchAll(\PDO::FETCH_ASSOC);
        
        $species = [];
        foreach ($results as $data) {
            $species[] = Species::fromArray($data);
        }
        
        return $species;
    }

    /**
     * Get current game session for player
     */
    public function getCurrentGameSession(string $sessionId): ?GameSession
    {
        $player = $this->getPlayerBySession($sessionId);
        if (!$player) {
            return null;
        }
        
        $stmt = $this->db->prepare("
            SELECT * FROM game_sessions 
            WHERE player_id = ? AND ended_at IS NULL 
            ORDER BY started_at DESC 
            LIMIT 1
        ");
        
        $stmt->execute([$player->id]);
        $data = $stmt->fetch(\PDO::FETCH_ASSOC);
        
        if (!$data) {
            return null;
        }
        
        return GameSession::fromArray($data);
    }

    /**
     * Start a new game session
     */
    public function startGameSession(string $sessionId): GameSession
    {
        $player = $this->getPlayerBySession($sessionId);
        if (!$player) {
            throw new \Exception('Player not found');
        }
        
        // End any existing active session
        $existingSession = $this->getCurrentGameSession($sessionId);
        if ($existingSession) {
            $this->endGameSession($sessionId, $player->credits);
        }
        
        $gameSession = new GameSession();
        $gameSession->id = RandomUtils::generateId('session');
        $gameSession->playerId = $player->id;
        
        $stmt = $this->db->prepare("
            INSERT INTO game_sessions (id, player_id, started_at, planets_traded)
            VALUES (?, ?, ?, ?)
        ");
        
        $stmt->execute([
            $gameSession->id,
            $gameSession->playerId,
            $gameSession->startedAt->format('Y-m-d H:i:s'),
            $gameSession->planetsTraded
        ]);
        
        // Update player game started status
        $player->gameStarted = true;
        $this->updatePlayer($player);
        
        return $gameSession;
    }

    /**
     * End game session
     */
    public function endGameSession(string $sessionId, int $finalCredits): bool
    {
        $session = $this->getCurrentGameSession($sessionId);
        if (!$session) {
            return false;
        }
        
        $stmt = $this->db->prepare("
            UPDATE game_sessions 
            SET ended_at = ?, final_credits = ?
            WHERE id = ?
        ");
        
        return $stmt->execute([
            (new \DateTime())->format('Y-m-d H:i:s'),
            $finalCredits,
            $session->id
        ]);
    }

    /**
     * Increment planets traded counter
     */
    public function incrementPlanetsTraded(string $sessionId): bool
    {
        $session = $this->getCurrentGameSession($sessionId);
        if (!$session) {
            return false;
        }
        
        $stmt = $this->db->prepare("
            UPDATE game_sessions 
            SET planets_traded = planets_traded + 1
            WHERE id = ?
        ");
        
        return $stmt->execute([$session->id]);
    }

    /**
     * Get player state for API response
     */
    public function getPlayerState(string $sessionId): array
    {
        $player = $this->getPlayerBySession($sessionId);
        if (!$player) {
            throw new \Exception('Player not found');
        }
        
        $currentPlanet = null;
        if ($player->currentPlanetId) {
            $currentPlanet = $this->getPlanetById($player->currentPlanetId);
        }
        
        $ownedPlanets = $this->getPlanetsByOwner($player->id);
        $session = $this->getCurrentGameSession($sessionId);
        
        return [
            'player' => $player->toArray(),
            'currentPlanet' => $currentPlanet?->toArray(),
            'ownedPlanets' => array_map(fn($p) => $p->toArray(), $ownedPlanets),
            'session' => $session?->toArray(),
            'gameStarted' => $player->gameStarted
        ];
    }

    /**
     * Hydrate planet object from database data
     */
    private function hydratePlanetFromData(array $data): Planet
    {
        $planet = Planet::fromArray($data);
        
        // Create and attach planet type
        $planetType = new \App\Models\PlanetType();
        $planetType->id = $data['type_id'];
        $planetType->name = $data['type_name'];
        $planetType->baseTemp = $data['base_temp'];
        $planetType->baseAtmo = $data['base_atmo'];
        $planetType->baseWater = $data['base_water'];
        $planetType->baseGrav = $data['base_grav'];
        $planetType->baseRad = $data['base_rad'];
        $planetType->color = $data['type_color'];
        
        $planet->type = $planetType;
        
        return $planet;
    }
}
