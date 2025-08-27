<?php
namespace App\Actions;

use App\Repositories\GameSessionRepository;
use App\Repositories\PlayerRepository;

class EndGameAction
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

    public function execute(string $sessionId): array
    {
        $session = $this->sessionRepository->findBySessionId($sessionId);
        if (!$session) {
            return ['success' => false, 'message' => 'Session not found'];
        }
        $session->endedAt = new \DateTime();
        $this->sessionRepository->save($session);
        return [
            'success' => true,
            'session_id' => $session->id,
            'message' => 'Game ended successfully'
        ];
    }
}
