<?php

declare(strict_types=1);

namespace App\Controllers;

use App\Services\GameStateServiceEnhanced;
use App\Http\Response;
use App\Http\Request;

class GameController extends BaseController
{
    private GameStateServiceEnhanced $gameStateService;

    public function __construct(GameStateServiceEnhanced $gameStateService)
    {
        $this->gameStateService = $gameStateService;
    }

    /**
     * Get current game status and player state
     */
    public function getStatus(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // Get session status using enhanced service
            $gameState = $this->gameStateService->getSessionStatus($sessionId);

            $this->logAction('game_status_requested', ['session_id' => $sessionId]);

            return $this->successResponse($response, $gameState, 'Game status retrieved');
        } catch (\Exception $e) {
            $this->logAction('game_status_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get game status: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Start a new game
     */
    public function startGame(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // Credits are server-controlled. The client may request a start/reset, not a balance.
            $result = $this->gameStateService->createSessionForOwner($sessionId);

            if ($result['success']) {
                $this->logAction('game_started', [
                    'session_id' => $result['session_id'],
                    'starting_credits' => $result['starting_credits'] ?? null
                ]);

                return $this->successResponse($response, $result, 'Game started successfully');
            }

            return $this->errorResponse($response, $result['message'] ?? $result['error'] ?? 'Failed to start game', 400);
        } catch (\Exception $e) {
            $this->logAction('game_start_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to start game: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Reset the authenticated user's game to a new server-approved starting state.
     */
    public function resetGame(Request $request, Response $response): Response
    {
        return $this->startGame($request, $response);
    }

    /**
     * Load an existing game session
     */
    public function loadGame(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // Load session using enhanced service
            $result = $this->gameStateService->loadSession($sessionId);

            $this->logAction('game_loaded', ['session_id' => $sessionId]);

            return $this->successResponse($response, $result, 'Game loaded successfully');
        } catch (\Exception $e) {
            $this->logAction('game_load_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to load game: ' . $e->getMessage(), 500);
        }
    }

    /**
     * End current game
     */
    public function endGame(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // End session using enhanced service
            $result = $this->gameStateService->endSession($sessionId);

            $this->logAction('game_ended', ['session_id' => $sessionId]);

            return $this->successResponse($response, $result, 'Game ended successfully');
        } catch (\Exception $e) {
            $this->logAction('game_end_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to end game: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Save current game state
     */
    public function saveGame(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // The enhanced service automatically persists state, so we just need to get current status
            $result = $this->gameStateService->getSessionStatus($sessionId);

            $this->logAction('game_saved', ['session_id' => $sessionId]);

            return $this->successResponse($response, $result, 'Game saved successfully');
        } catch (\Exception $e) {
            $this->logAction('game_save_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to save game: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get aggregate game stats for the authenticated user's active session.
     */
    public function getStats(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $result = $this->gameStateService->getSessionStatus($sessionId);

            if (!($result['success'] ?? false)) {
                return $this->errorResponse($response, $result['error'] ?? 'Session not found', 404);
            }

            return $this->successResponse($response, [
                'session_id' => $sessionId,
                'statistics' => $result['statistics'] ?? [],
                'current_credits' => $result['current_credits'] ?? 0,
                'planets_discovered' => $result['planets_discovered'] ?? 0,
                'planets_unsold' => $result['planets_unsold'] ?? 0,
            ], 'Game stats retrieved');
        } catch (\Exception $e) {
            $this->logAction('game_stats_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get game stats: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get list of saved games for the current player
     */
    public function getSavedGames(Request $request, Response $response): Response
    {
        try {
            // For now, return empty array since we're using session-based approach
            // This could be enhanced later to support multiple saved games per user
            $savedGames = [];

            $this->logAction('saved_games_requested');

            return $this->successResponse($response, $savedGames, 'Saved games retrieved');
        } catch (\Exception $e) {
            $this->logAction('saved_games_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get saved games: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Generate a new planet for the current session
     */
    public function generatePlanet(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);

            // Generate new planet using enhanced service
            $result = $this->gameStateService->generateNewPlanet($sessionId);

            if ($result['success']) {
                $this->logAction('planet_generated', [
                    'session_id' => $sessionId,
                    'planet_id' => $result['planet']['id'] ?? null
                ]);

                return $this->successResponse($response, $result, 'Planet generated successfully');
            } else {
                return $this->errorResponse($response, $result['message'] ?? 'Failed to generate planet', 400);
            }
        } catch (\Exception $e) {
            $this->logAction('planet_generation_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to generate planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Apply tools to a planet
     */
    public function applyTools(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $body = $request->getParsedBody() ?? [];
            $planetId = $body['planet_id'] ?? null;
            $toolIds = $body['tool_ids'] ?? [];

            if (!$planetId) {
                return $this->errorResponse($response, 'Planet ID is required', 400);
            }

            if (empty($toolIds)) {
                return $this->errorResponse($response, 'Tool IDs are required', 400);
            }

            // Apply tools using enhanced service
            $result = $this->gameStateService->applyTools($sessionId, $planetId, $toolIds);

            if ($result['success']) {
                $this->logAction('tools_applied', [
                    'session_id' => $sessionId,
                    'planet_id' => $planetId,
                    'tool_ids' => $toolIds
                ]);

                return $this->successResponse($response, $result, 'Tools applied successfully');
            } else {
                return $this->errorResponse($response, $result['message'] ?? 'Failed to apply tools', 400);
            }
        } catch (\Exception $e) {
            $this->logAction('tools_application_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to apply tools: ' . $e->getMessage(), 500);
        }
    }
}
