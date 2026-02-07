<?php

namespace App\Controllers;

use App\Repositories\RepositoryManager;
use App\Http\Response;
use App\Http\Request;

class DataController extends BaseController
{
    private RepositoryManager $repositories;

    public function __construct(RepositoryManager $repositories)
    {
        $this->repositories = $repositories;
    }

    /**
     * Get all planet types
     */
    public function getPlanetTypes(Request $request, Response $response): Response
    {
        try {
            $planetTypes = $this->repositories->planetTypes()->getAll();
            
            $typesData = array_map(fn($type) => $type->toArray(), $planetTypes);

            return $this->successResponse($response, [
                'planetTypes' => $typesData
            ], 'Planet types retrieved');

        } catch (\Exception $e) {
            $this->logAction('planet_types_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get planet types: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get all species
     */
    public function getSpecies(Request $request, Response $response): Response
    {
        try {
            $species = $this->repositories->species()->getAll();
            
            $speciesData = array_map(fn($spec) => $spec->toArray(), $species);

            return $this->successResponse($response, [
                'species' => $speciesData
            ], 'Species retrieved');

        } catch (\Exception $e) {
            $this->logAction('species_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get species: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get all tools
     */
    public function getTools(Request $request, Response $response): Response
    {
        try {
            $tools = $this->repositories->tools()->getAll();
            
            $toolsData = array_map(fn($tool) => $tool->toArray(), $tools);

            return $this->successResponse($response, [
                'tools' => $toolsData
            ], 'Tools retrieved');

        } catch (\Exception $e) {
            $this->logAction('tools_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get tools: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get random planet names by category
     */
    public function getPlanetNames(Request $request, Response $response): Response
    {
        try {
            $queryParams = $request->getQueryParams();
            $category = $queryParams['category'] ?? null;
            $count = min(50, max(1, (int) ($queryParams['count'] ?? 10)));

            if ($category) {
                $names = $this->repositories->planetNames()->getUnusedByCategory($category);
            } else {
                $names = $this->repositories->planetNames()->getRandomUnused($count);
            }

            return $this->successResponse($response, [
                'names' => $names,
                'count' => count($names)
            ], 'Planet names retrieved');

        } catch (\Exception $e) {
            $this->logAction('planet_names_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get planet names: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get game configuration and static data
     */
    public function getGameConfig(Request $request, Response $response): Response
    {
        try {
            $config = [
                'maxPlanetsPerSession' => 100,
                'startingCredits' => 10000,
                'marketRefreshInterval' => 300, // 5 minutes
                'planetGenerationCost' => 100,
                'toolCostMultiplier' => 1.5,
                'version' => '1.0.0'
            ];

            return $this->successResponse($response, $config, 'Game configuration retrieved');

        } catch (\Exception $e) {
            $this->logAction('config_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get game config: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Reset unused planet names (admin function)
     */
    public function resetPlanetNames(Request $request, Response $response): Response
    {
        try {
            $success = $this->repositories->planetNames()->resetAllToUnused();
            
            if ($success) {
                $this->logAction('planet_names_reset', []);
                return $this->successResponse($response, [
                    'reset' => true
                ], 'Planet names reset successfully');
            } else {
                return $this->errorResponse($response, 'Failed to reset planet names', 500);
            }

        } catch (\Exception $e) {
            $this->logAction('planet_names_reset_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to reset planet names: ' . $e->getMessage(), 500);
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
