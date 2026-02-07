<?php

namespace App\Controllers;

use App\Services\PlanetGeneratorService;
use App\Services\TradingService;
use App\Services\GameStateServiceEnhanced;
use App\Http\Response;
use App\Http\Request;

class PlanetController extends BaseController
{
    private PlanetGeneratorService $planetGeneratorService;
    private TradingService $tradingService;
    private GameStateServiceEnhanced $gameStateService;
    private \App\Actions\GeneratePlanetOptionsAction $generatePlanetOptionsAction;
    private \App\Actions\GetOwnedPlanetsAction $getOwnedPlanetsAction;
    private \App\Actions\GetCurrentPlanetAction $getCurrentPlanetAction;
    private \App\Actions\GetPlanetAction $getPlanetAction;
    private \App\Actions\PurchasePlanetAction $purchasePlanetAction;
    private \App\Actions\SetCurrentPlanetAction $setCurrentPlanetAction;
    private \App\Actions\AnalyzePlanetAction $analyzePlanetAction;

    public function __construct(
        PlanetGeneratorService $planetGeneratorService,
        TradingService $tradingService,
        GameStateServiceEnhanced $gameStateService,
        \App\Actions\GeneratePlanetOptionsAction $generatePlanetOptionsAction,
        \App\Actions\GetOwnedPlanetsAction $getOwnedPlanetsAction,
        \App\Actions\GetCurrentPlanetAction $getCurrentPlanetAction,
        \App\Actions\GetPlanetAction $getPlanetAction,
        \App\Actions\PurchasePlanetAction $purchasePlanetAction,
        \App\Actions\SetCurrentPlanetAction $setCurrentPlanetAction,
        \App\Actions\AnalyzePlanetAction $analyzePlanetAction
    ) {
        $this->planetGeneratorService = $planetGeneratorService;
        $this->tradingService = $tradingService;
        $this->gameStateService = $gameStateService;
        $this->generatePlanetOptionsAction = $generatePlanetOptionsAction;
        $this->getOwnedPlanetsAction = $getOwnedPlanetsAction;
        $this->getCurrentPlanetAction = $getCurrentPlanetAction;
        $this->getPlanetAction = $getPlanetAction;
        $this->purchasePlanetAction = $purchasePlanetAction;
        $this->setCurrentPlanetAction = $setCurrentPlanetAction;
        $this->analyzePlanetAction = $analyzePlanetAction;
    }

    /**
     * Generate new planet options for purchase
     */
    public function generatePlanets(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $queryParams = $request->getQueryParams();
            $count = max(1, min(10, (int) ($queryParams['count'] ?? 3))); // Between 1-10 planets
            
            // Use Action instead of service for business logic
            $planets = $this->generatePlanetOptionsAction->execute($count);
            
            // Convert to array format for JSON response
            $planetsData = array_map(fn($planet) => $planet->toArray(), $planets);
            
            $this->logAction('planets_generated', [
                'session_id' => $sessionId,
                'count' => $count,
                'planet_ids' => array_column($planetsData, 'id')
            ]);
            
            return $this->successResponse($response, [
                'planets' => $planetsData,
                'count' => count($planetsData)
            ], 'Planet options generated');

        } catch (\Exception $e) {
            $this->logAction('planet_generation_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to generate planets: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get specific planet details
     */
    public function getPlanet(Request $request, Response $response, array $args): Response
    {
        try {
            $planetId = $args['id'] ?? '';
            if (empty($planetId)) {
                return $this->errorResponse($response, 'Planet ID is required', 400);
            }
            $planet = $this->getPlanetAction->execute($planetId);
            if (!$planet) {
                return $this->errorResponse($response, 'Planet not found', 404);
            }
            return $this->successResponse($response, [
                'planet' => $planet->toArray()
            ], 'Planet details retrieved');
        } catch (\Exception $e) {
            $this->logAction('planet_get_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Purchase a planet
     */
    public function purchasePlanet(Request $request, Response $response, array $args): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $planetId = $args['id'] ?? '';
            if (empty($planetId)) {
                return $this->errorResponse($response, 'Planet ID is required', 400);
            }
            $result = $this->purchasePlanetAction->execute($sessionId, $planetId);
            if (!$result['success']) {
                return $this->errorResponse($response, $result['message'], 400);
            }
            $this->logAction('planet_purchased', [
                'session_id' => $sessionId,
                'planet_id' => $planetId,
                'price' => $result['planet']['purchasePrice'] ?? 0
            ]);
            return $this->successResponse($response, $result, $result['message']);
        } catch (\Exception $e) {
            $this->logAction('planet_purchase_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to purchase planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get owned planets for the current player
     */
    public function getOwnedPlanets(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            [$page, $perPage] = $this->getPaginationParams($request);
            $planets = $this->getOwnedPlanetsAction->execute($sessionId);
            $paginatedData = $this->paginate($planets, $page, $perPage);
            $paginatedData['items'] = array_map(fn($planet) => $planet->toArray(), $paginatedData['items']);
            return $this->successResponse($response, $paginatedData, 'Owned planets retrieved');
        } catch (\Exception $e) {
            $this->logAction('owned_planets_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get owned planets: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Set current planet for the player
     */
    public function setCurrentPlanet(Request $request, Response $response, array $args): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $planetId = $args['id'] ?? '';
            if (empty($planetId)) {
                return $this->errorResponse($response, 'Planet ID is required', 400);
            }
            $result = $this->setCurrentPlanetAction->execute($sessionId, $planetId);
            if (!$result['success']) {
                return $this->errorResponse($response, $result['message'], 400);
            }
            $this->logAction('current_planet_set', [
                'session_id' => $sessionId,
                'planet_id' => $planetId
            ]);
            return $this->successResponse($response, $result, $result['message']);
        } catch (\Exception $e) {
            $this->logAction('set_current_planet_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to set current planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get current planet for the player
     */
    public function getCurrentPlanet(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $planet = $this->getCurrentPlanetAction->execute($sessionId);
            if (!$planet) {
                return $this->successResponse($response, [
                    'currentPlanet' => null
                ], 'No current planet selected');
            }
            return $this->successResponse($response, [
                'currentPlanet' => $planet->toArray()
            ], 'Current planet retrieved');
        } catch (\Exception $e) {
            $this->logAction('get_current_planet_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get current planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Calculate planet value and rarity
     */
    public function analyzePlanet(Request $request, Response $response, array $args): Response
    {
        try {
            $planetId = $args['id'] ?? '';
            if (empty($planetId)) {
                return $this->errorResponse($response, 'Planet ID is required', 400);
            }
            $result = $this->analyzePlanetAction->execute($planetId);
            if (!$result['success']) {
                return $this->errorResponse($response, $result['message'], 404);
            }
            return $this->successResponse($response, $result['analysis'], $result['message']);
        } catch (\Exception $e) {
            $this->logAction('planet_analysis_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to analyze planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Handle CORS preflight requests
     */
    public function options(Request $request, Response $response): Response
    {
        return $this->handleOptions($request, $response);
    }

    /**
     * Check if planet is habitable
     */
    private function isHabitable($planet): bool
    {
        return $planet->temperature >= -10 && $planet->temperature <= 40 &&
               $planet->atmosphere >= 0.5 && $planet->atmosphere <= 2.0 &&
               $planet->water >= 0.1 &&
               $planet->gravity >= 0.5 && $planet->gravity <= 2.0 &&
               $planet->radiation <= 0.5;
    }
}
