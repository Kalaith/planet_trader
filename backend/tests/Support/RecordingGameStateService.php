<?php

declare(strict_types=1);

namespace Tests\Support;

use App\Services\GameStateServiceEnhanced;

final class RecordingGameStateService extends GameStateServiceEnhanced
{
    public ?string $sessionId = null;
    public ?int $startingCredits = null;

    public function __construct()
    {
    }

    public function createSessionForOwner(string $ownerSessionId, int $startingCredits = 10000): array
    {
        $this->sessionId = $ownerSessionId;
        $this->startingCredits = $startingCredits;

        return [
            'success' => true,
            'session_id' => $ownerSessionId,
            'starting_credits' => $startingCredits,
            'initial_planets' => 0,
        ];
    }
}
