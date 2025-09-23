<?php

declare(strict_types=1);

use DI\Container;
use Slim\Factory\AppFactory;
use Psr\Http\Message\ResponseInterface as Response;
use Psr\Http\Message\ServerRequestInterface as Request;

// Include Composer autoloader
require __DIR__ . '/../vendor/autoload.php';

// Load environment variables if .env file exists
if (file_exists(__DIR__ . '/../.env')) {
    $dotenv = Dotenv\Dotenv::createImmutable(__DIR__ . '/../');
    $dotenv->load();
}

// Initialize Eloquent for Auth0 User model
\App\Database\EloquentSetup::initialize();

// Create Container using PHP-DI
$container = new Container();

// Configure dependencies
$container->set('settings', function () {
    return [
        'displayErrorDetails' => $_ENV['APP_DEBUG'] ?? false,
        'logErrors' => true,
        'logErrorDetails' => true,
        'db' => [
            'driver' => $_ENV['DB_CONNECTION'] ?? 'sqlite',
            'host' => $_ENV['DB_HOST'] ?? 'localhost',
            'port' => $_ENV['DB_PORT'] ?? '3306',
            'database' => $_ENV['DB_DATABASE'] ?? __DIR__ . '/../storage/database.sqlite',
            'username' => $_ENV['DB_USERNAME'] ?? '',
            'password' => $_ENV['DB_PASSWORD'] ?? '',
        ]
    ];
});

// Database connection using our enhanced Connection class
$container->set(PDO::class, function ($container) {
    $connection = \App\Database\Connection::getInstance();
    return $connection->getPdo();
});

// Repository Manager
$container->set(\App\Repositories\RepositoryManager::class, function ($container) {
    return new \App\Repositories\RepositoryManager($container->get(PDO::class));
});

// Enhanced Services with repository pattern
$container->set(\App\Services\GameStateServiceEnhanced::class, function ($container) {
    return new \App\Services\GameStateServiceEnhanced(
        $container->get(\App\Repositories\RepositoryManager::class),
        $container->get(\App\Services\PlanetGeneratorService::class)
    );
});

$container->set(\App\Services\PlanetNameService::class, function ($container) {
    return new \App\Services\PlanetNameService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\PlanetGeneratorService::class, function ($container) {
    return new \App\Services\PlanetGeneratorService(
        $container->get(\App\Services\PlanetNameService::class),
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\SpeciesGeneratorService::class, function ($container) {
    return new \App\Services\SpeciesGeneratorService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\PricingService::class, function ($container) {
    return new \App\Services\PricingService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\TradingService::class, function ($container) {
    return new \App\Services\TradingService(
        $container->get(\App\Services\GameStateServiceEnhanced::class),
        $container->get(\App\Services\PricingService::class),
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\TerraformingService::class, function ($container) {
    return new \App\Services\TerraformingService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\ResearchService::class, function ($container) {
    return new \App\Services\ResearchService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\MarketService::class, function ($container) {
    return new \App\Services\MarketService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Services\SessionService::class, function ($container) {
    return new \App\Services\SessionService(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

// Action Factory
$container->set(\App\Actions\ActionFactory::class, function ($container) {
    return new \App\Actions\ActionFactory(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

// Register controllers with enhanced services

// Register GameController with Actions (refactored)
$container->set(\App\Controllers\GameController::class, function ($container) {
    return new \App\Controllers\GameController(
        $container->get(\App\Actions\StartGameAction::class),
        $container->get(\App\Actions\LoadGameAction::class),
        $container->get(\App\Actions\SaveGameAction::class),
        $container->get(\App\Actions\GetGameStateAction::class),
        $container->get(\App\Actions\EndGameAction::class)
    );
});

// Register PlanetController with Actions (already refactored)
$container->set(\App\Controllers\PlanetController::class, function ($container) {
    return new \App\Controllers\PlanetController(
        $container->get(\App\Actions\GeneratePlanetOptionsAction::class),
        $container->get(\App\Actions\GetOwnedPlanetsAction::class),
        $container->get(\App\Actions\GetCurrentPlanetAction::class),
        $container->get(\App\Actions\GetPlanetAction::class),
        $container->get(\App\Actions\PurchasePlanetAction::class),
        $container->get(\App\Actions\SetCurrentPlanetAction::class),
        $container->get(\App\Actions\AnalyzePlanetAction::class)
    );
});

// Register TradingController with Actions (refactored)
$container->set(\App\Controllers\TradingController::class, function ($container) {
    return new \App\Controllers\TradingController(
        $container->get(\App\Actions\GetBuyersAction::class),
        $container->get(\App\Actions\SellPlanetAction::class),
        $container->get(\App\Actions\CalculateProfitAction::class),
        $container->get(\App\Actions\GetMarketAction::class),
        $container->get(\App\Actions\GetTradingStatsAction::class),
        $container->get(\App\Actions\GetCompatibilityAction::class),
        $container->get(\App\Actions\GetTradeHistoryAction::class),
        $container->get(\App\Actions\GetPlayerInventoryAction::class),
        $container->get(\App\Actions\BuyItemAction::class)
    );
});

$container->set(\App\Controllers\PlayerController::class, function ($container) {
    return new \App\Controllers\PlayerController(
        $container->get(\App\Services\GameStateServiceEnhanced::class),
        $container->get(\App\Services\SessionService::class)
    );
});

$container->set(\App\Controllers\DataController::class, function ($container) {
    return new \App\Controllers\DataController(
        $container->get(\App\Repositories\RepositoryManager::class)
    );
});

$container->set(\App\Controllers\Auth0Controller::class, function ($container) {
    return new \App\Controllers\Auth0Controller();
});

$container->set(\App\Actions\GetOwnedPlanetsAction::class, function ($container) {
    return new \App\Actions\GetOwnedPlanetsAction(
        $container->get(\App\Repositories\RepositoryManager::class)->planet()
    );
});

$container->set(\App\Actions\GetCurrentPlanetAction::class, function ($container) {
    return new \App\Actions\GetCurrentPlanetAction(
        $container->get(\App\Repositories\RepositoryManager::class)->planet()
    );
});

$container->set(\App\Actions\GetPlanetAction::class, function ($container) {
    return new \App\Actions\GetPlanetAction(
        $container->get(\App\Repositories\RepositoryManager::class)->planet()
    );
});

$container->set(\App\Actions\PurchasePlanetAction::class, function ($container) {
    $repoManager = $container->get(\App\Repositories\RepositoryManager::class);
    return new \App\Actions\PurchasePlanetAction(
        $repoManager->planet(),
        $repoManager->player(),
        $repoManager->gameSession()
    );
});

$container->set(\App\Actions\SetCurrentPlanetAction::class, function ($container) {
    return new \App\Actions\SetCurrentPlanetAction(
        $container->get(\App\Repositories\RepositoryManager::class)->planet()
    );
});

$container->set(\App\Actions\AnalyzePlanetAction::class, function ($container) {
    return new \App\Actions\AnalyzePlanetAction(
        $container->get(\App\Repositories\RepositoryManager::class)->planet()
    );
});

// Set container to create App with dependency injection
AppFactory::setContainer($container);

// Create App
$app = AppFactory::create();

// Add CORS middleware
$app->add(function (Request $request, Response $response, $next) {
    $response = $next($request, $response);
    return $response
        ->withHeader('Access-Control-Allow-Origin', $_ENV['CORS_ORIGIN'] ?? '*')
        ->withHeader('Access-Control-Allow-Headers', 'X-Requested-With, Content-Type, Accept, Origin, Authorization, X-Session-ID')
        ->withHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS')
        ->withHeader('Access-Control-Allow-Credentials', 'true');
});

// Add Error Middleware
$errorMiddleware = $app->addErrorMiddleware(true, true, true);

// Register routes
$routes = require __DIR__ . '/../src/Routes/routes.php';
$routes($app);

// Run app
$app->run();
