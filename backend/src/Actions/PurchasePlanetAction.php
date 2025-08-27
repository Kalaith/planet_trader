<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;
use App\Repositories\PlayerRepository;
use App\Repositories\GameSessionRepository;
use App\Models\Planet;
use App\Models\Player;

class PurchasePlanetAction
{
    private PlanetRepository $planetRepository;
    private PlayerRepository $playerRepository;
    private GameSessionRepository $sessionRepository;

    public function __construct(
        PlanetRepository $planetRepository,
        PlayerRepository $playerRepository,
        GameSessionRepository $sessionRepository
    ) {
        $this->planetRepository = $planetRepository;
        $this->playerRepository = $playerRepository;
        $this->sessionRepository = $sessionRepository;
    }

    public function execute(string $sessionId, string $planetId): array
    {
        $session = $this->sessionRepository->findBySessionId($sessionId);
        if (!$session) {
            return ['success' => false, 'message' => 'Session not found'];
        }
        $player = $this->playerRepository->findById($session->playerId);
        if (!$player) {
            return ['success' => false, 'message' => 'Player not found'];
        }
        $planet = $this->planetRepository->findById($planetId);
        if (!$planet) {
            return ['success' => false, 'message' => 'Planet not found'];
        }
        if ($player->credits < $planet->purchasePrice) {
            return ['success' => false, 'message' => 'Insufficient credits'];
        }
        $player->credits -= $planet->purchasePrice;
        $planet->ownerId = $player->id;
        $this->playerRepository->save($player);
        $this->planetRepository->save($planet);
        return [
            'success' => true,
            'planet' => $planet->toArray(),
            'player' => $player->toArray(),
            'message' => 'Planet purchased successfully'
        ];
    }
}
