<?php

use App\Core\Router;
use App\Controllers\AuthController;
use App\Controllers\GameController;
use App\Controllers\PlanetController;
use App\Controllers\TradingController;
use App\Controllers\DataController;
use App\Middleware\WebHatcheryJwtMiddleware;

return function (
    Router $router,
    GameController $gameController,
    PlanetController $planetController,
    TradingController $tradingController,
    DataController $dataController
): void {
    $api = '/api';

    // Auth session
    $router->get($api . '/auth/session', [AuthController::class, 'session'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/me', [AuthController::class, 'session'], [WebHatcheryJwtMiddleware::class]);

    // Status + health (public)
    $router->get($api . '/status', function ($request, $response) {
        $response->getBody()->write(json_encode([
            'status' => 'OK',
            'service' => 'Planet Trader API',
            'version' => '1.0.0'
        ]));
        return $response->withHeader('Content-Type', 'application/json');
    });

    $router->get('/health', function ($request, $response) {
        $response->getBody()->write(json_encode([
            'status' => 'healthy',
            'timestamp' => date('c'),
            'uptime' => time() - ($_SERVER['REQUEST_TIME'] ?? time())
        ]));
        return $response->withHeader('Content-Type', 'application/json');
    });

    // Game routes (protected)
    $router->get($api . '/game/status', [$gameController, 'getStatus'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/game/start', [$gameController, 'startGame'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/game/end', [$gameController, 'endGame'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/game/reset', [$gameController, 'resetGame'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/game/stats', [$gameController, 'getStats'], [WebHatcheryJwtMiddleware::class]);
    $router->put($api . '/game/credits', [$gameController, 'updateCredits'], [WebHatcheryJwtMiddleware::class]);

    // Planet routes (protected)
    $router->post($api . '/planets', [$planetController, 'generatePlanets'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/planets/owned', [$planetController, 'getOwnedPlanets'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/planets/current', [$planetController, 'getCurrentPlanet'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/planets/{id}', [$planetController, 'getPlanet'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/planets/{id}/purchase', [$planetController, 'purchasePlanet'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/planets/{id}/select', [$planetController, 'setCurrentPlanet'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/planets/{id}/analyze', [$planetController, 'analyzePlanet'], [WebHatcheryJwtMiddleware::class]);

    // Trading routes (protected)
    $router->get($api . '/trading/buyers', [$tradingController, 'getBuyers'], [WebHatcheryJwtMiddleware::class]);
    $router->post($api . '/trading/sell', [$tradingController, 'sellPlanet'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/trading/profit', [$tradingController, 'calculateProfit'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/trading/market', [$tradingController, 'getMarket'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/trading/stats', [$tradingController, 'getTradingStats'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/trading/compatibility', [$tradingController, 'getCompatibility'], [WebHatcheryJwtMiddleware::class]);
    $router->get($api . '/trading/history', [$tradingController, 'getTradeHistory'], [WebHatcheryJwtMiddleware::class]);

    // Data routes (public)
    $router->get($api . '/planet-name', [$planetController, 'getRandomPlanetName']);
    $router->post($api . '/species/generate', [$planetController, 'generateSpecies']);
    $router->get($api . '/data/planet-types', [$dataController, 'getPlanetTypes']);
    $router->get($api . '/data/species', [$dataController, 'getSpecies']);
    $router->get($api . '/data/tools', [$dataController, 'getTools']);
    $router->get($api . '/data/planet-names', [$dataController, 'getPlanetNames']);
    $router->get($api . '/data/config', [$dataController, 'getGameConfig']);
    $router->post($api . '/data/planet-names/reset', [$dataController, 'resetPlanetNames']);

    // Root endpoint
    $router->get('/', function ($request, $response) {
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
};
