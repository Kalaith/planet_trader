<?php

namespace App\Controllers;

use App\Services\GameStateServiceEnhanced;
use App\Services\SessionService;
use App\Http\Response;
use App\Http\Request;

class PlayerController extends BaseController
{
    private GameStateServiceEnhanced $gameStateService;
    private SessionService $sessionService;

    public function __construct(
        GameStateServiceEnhanced $gameStateService,
        SessionService $sessionService
    ) {
        $this->gameStateService = $gameStateService;
        $this->sessionService = $sessionService;
    }

    /**
     * Get current player information
     */
    public function getPlayer(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            
            if (!$player) {
                return $this->errorResponse($response, 'Player not found', 404);
            }

            return $this->successResponse($response, [
                'player' => $player->toArray()
            ], 'Player information retrieved');

        } catch (\Exception $e) {
            $this->logAction('player_get_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get player: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Update player settings
     */
    public function updatePlayer(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $data = $this->getJsonBody($request);
            
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            
            if (!$player) {
                return $this->errorResponse($response, 'Player not found', 404);
            }

            // Update allowed fields
            if (isset($data['name'])) {
                $player->name = $data['name'];
            }
            
            $this->gameStateService->updatePlayer($player);
            
            $this->logAction('player_updated', [
                'session_id' => $sessionId,
                'player_id' => $player->id
            ]);

            return $this->successResponse($response, [
                'player' => $player->toArray()
            ], 'Player updated successfully');

        } catch (\Exception $e) {
            $this->logAction('player_update_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to update player: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get player statistics
     */
    public function getStats(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            
            $player = $this->gameStateService->getPlayerBySession($sessionId);
            
            if (!$player) {
                return $this->errorResponse($response, 'Player not found', 404);
            }

            $sessions = $this->sessionService->getPlayerSessions($player->id);
            $ownedPlanets = $this->gameStateService->getPlanetsByOwner($player->id);
            
            $totalGamesPlayed = count($sessions);
            $totalInvested = array_sum(array_map(fn($p) => $p->purchasePrice, $ownedPlanets));
            $netWorth = $player->credits + $totalInvested;
            
            $stats = [
                'player' => $player->toArray(),
                'totalGamesPlayed' => $totalGamesPlayed,
                'currentCredits' => $player->credits,
                'planetsOwned' => count($ownedPlanets),
                'totalInvested' => $totalInvested,
                'netWorth' => $netWorth,
                'averageCreditsPerGame' => $totalGamesPlayed > 0 ? $player->credits / $totalGamesPlayed : 0
            ];

            return $this->successResponse($response, $stats, 'Player statistics retrieved');

        } catch (\Exception $e) {
            $this->logAction('player_stats_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get player stats: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Handle CORS preflight requests
     */
    public function options(Request $request, Response $response): Response
    {
        return $this->handleOptions($request, $response);
    }
}
