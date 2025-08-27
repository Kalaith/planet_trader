<?php
namespace App\Actions;

use App\Services\GameStateServiceEnhanced;

class LoadGameAction
{
    private GameStateServiceEnhanced $gameStateService;

    public function __construct(GameStateServiceEnhanced $gameStateService)
    {
        $this->gameStateService = $gameStateService;
    }

    public function execute(string $sessionId): array
    {
        return $this->gameStateService->loadSession($sessionId);
    }
}
