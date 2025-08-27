<?php

use Slim\App;
use Slim\Routing\RouteCollectorProxy;
use App\Controllers\GameController;
use App\Controllers\PlanetController;
use App\Controllers\TradingController;

return function (App $app) {
    
    // API Routes
    $app->group('/api', function (RouteCollectorProxy $group) {
        
        // Game Management Routes
        $group->group('/game', function (RouteCollectorProxy $game) {
            $game->get('/status', [GameController::class, 'getStatus']);
            $game->post('/start', [GameController::class, 'startGame']);
            $game->post('/end', [GameController::class, 'endGame']);
            $game->post('/reset', [GameController::class, 'resetGame']);
            $game->get('/stats', [GameController::class, 'getStats']);
            $game->put('/credits', [GameController::class, 'updateCredits']);
            $game->options('/{routes:.+}', [GameController::class, 'options']);
        });
        
        // Planet Management Routes
        $group->group('/planets', function (RouteCollectorProxy $planets) {
            $planets->post('', [PlanetController::class, 'generatePlanets']); // Generate new planets
            $planets->get('/owned', [PlanetController::class, 'getOwnedPlanets']);
            $planets->get('/current', [PlanetController::class, 'getCurrentPlanet']);
            $planets->get('/{id}', [PlanetController::class, 'getPlanet']);
            $planets->post('/{id}/purchase', [PlanetController::class, 'purchasePlanet']);
            $planets->post('/{id}/select', [PlanetController::class, 'setCurrentPlanet']);
            $planets->get('/{id}/analyze', [PlanetController::class, 'analyzePlanet']);
            $planets->options('/{routes:.+}', [PlanetController::class, 'options']);
        });
        
        // Trading Routes
        $group->group('/trading', function (RouteCollectorProxy $trading) {
            $trading->get('/buyers', [TradingController::class, 'getBuyers']);
            $trading->post('/sell', [TradingController::class, 'sellPlanet']);
            $trading->get('/profit', [TradingController::class, 'calculateProfit']);
            $trading->get('/market', [TradingController::class, 'getMarket']);
            $trading->get('/stats', [TradingController::class, 'getTradingStats']);
            $trading->get('/compatibility', [TradingController::class, 'getCompatibility']);
            $trading->get('/history', [TradingController::class, 'getTradeHistory']);
            $trading->options('/{routes:.+}', [TradingController::class, 'options']);
        });

        // Utility/Meta Routes
        $group->get('/planet-name', [PlanetController::class, 'getRandomPlanetName']);
        $group->post('/species/generate', [PlanetController::class, 'generateSpecies']);

        // Handle all OPTIONS requests for CORS
        $group->options('/{routes:.+}', function ($request, $response, $args) {
            return $response
                ->withHeader('Access-Control-Allow-Origin', '*')
                ->withHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
                ->withHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-Session-ID')
                ->withHeader('Access-Control-Max-Age', '3600');
        });
    });

    // Root endpoint
    $app->get('/', function ($request, $response) {
        $response->getBody()->write(json_encode([
            'name' => 'Planet Trader API',
            'version' => '1.0.0',
            'status' => 'running',
            'endpoints' => [
                'game' => '/api/game/*',
                'planets' => '/api/planets/*',
                'trading' => '/api/trading/*'
            ],
            'timestamp' => date('c')
        ]));
        
        return $response->withHeader('Content-Type', 'application/json');
    });

    // Health check endpoint
    $app->get('/health', function ($request, $response) {
        $response->getBody()->write(json_encode([
            'status' => 'healthy',
            'timestamp' => date('c'),
            'uptime' => time() - $_SERVER['REQUEST_TIME']
        ]));
        
        return $response->withHeader('Content-Type', 'application/json');
    });
};
