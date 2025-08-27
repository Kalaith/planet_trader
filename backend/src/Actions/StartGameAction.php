<?php
namespace App\Actions;

use App\Repositories\GameSessionRepository;
use App\Repositories\PlayerRepository;
use App\Models\GameSession;
use App\Models\Player;

class StartGameAction
{
    private GameSessionRepository $sessionRepository;
    private PlayerRepository $playerRepository;

    public function __construct(
        GameSessionRepository $sessionRepository,
        PlayerRepository $playerRepository
    ) {
        $this->sessionRepository = $sessionRepository;
        $this->playerRepository = $playerRepository;
    }

    public function execute(?int $playerId, int $startingCredits = 10000): array
    {
        $player = $playerId ? $this->playerRepository->findById($playerId) : new Player();
        if (!$player) {
            $player = new Player();
            $player->credits = $startingCredits;
            $this->playerRepository->save($player);
        }
        $session = new GameSession();
        $session->playerId = $player->id;
        $session->startedAt = new \DateTime();
        $this->sessionRepository->save($session);
        return [
            'success' => true,
            'session_id' => $session->id,
            'player' => $player->toArray(),
            'message' => 'Game started successfully'
        ];
    }
}
