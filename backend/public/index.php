<?php

require __DIR__ . '/../vendor/autoload.php';

use Dotenv\Dotenv;
use App\Core\Router;
use App\Database\Connection;
use App\Repositories\RepositoryManager;
use App\Services\PlanetNameService;
use App\Services\PlanetGeneratorService;
use App\Services\PricingService;
use App\Services\TradingService;
use App\Services\GameStateService;
use App\Services\GameStateServiceEnhanced;
use App\Controllers\GameController;
use App\Controllers\PlanetController;
use App\Controllers\TradingController;
use App\Controllers\DataController;
use App\Actions\CreatePlanetAction;
use App\Actions\GeneratePlanetOptionsAction;
use App\Actions\GetOwnedPlanetsAction;
use App\Actions\GetCurrentPlanetAction;
use App\Actions\GetPlanetAction;
use App\Actions\PurchasePlanetAction;
use App\Actions\SetCurrentPlanetAction;
use App\Actions\AnalyzePlanetAction;

// Load environment variables first
$dotenv = Dotenv::createImmutable(__DIR__ . '/..');
$dotenv->safeLoad();

// Initialize database connection
$connection = Connection::getInstance();
$pdo = $connection->getPdo();

// Repositories
$repositories = new RepositoryManager($pdo);

// Services
$planetNameService = new PlanetNameService();
$planetGeneratorService = new PlanetGeneratorService($planetNameService);
$gameStateServiceEnhanced = new GameStateServiceEnhanced($repositories, $planetGeneratorService);
$gameStateService = new GameStateService($pdo);
$pricingService = new PricingService();
$tradingService = new TradingService($gameStateService, $pricingService);

// Actions
$createPlanetAction = new CreatePlanetAction($repositories->planets());
$generatePlanetOptionsAction = new GeneratePlanetOptionsAction(
    $repositories->planetTypes(),
    $createPlanetAction
);
$getOwnedPlanetsAction = new GetOwnedPlanetsAction($repositories->planets());
$getCurrentPlanetAction = new GetCurrentPlanetAction($repositories->planets());
$getPlanetAction = new GetPlanetAction($repositories->planets());
$purchasePlanetAction = new PurchasePlanetAction(
    $repositories->planets(),
    $repositories->players(),
    $repositories->sessions()
);
$setCurrentPlanetAction = new SetCurrentPlanetAction($repositories->planets());
$analyzePlanetAction = new AnalyzePlanetAction($repositories->planets());

// Controllers
$gameController = new GameController($gameStateServiceEnhanced);
$planetController = new PlanetController(
    $planetGeneratorService,
    $tradingService,
    $gameStateServiceEnhanced,
    $generatePlanetOptionsAction,
    $getOwnedPlanetsAction,
    $getCurrentPlanetAction,
    $getPlanetAction,
    $purchasePlanetAction,
    $setCurrentPlanetAction,
    $analyzePlanetAction
);
$tradingController = new TradingController(
    $tradingService,
    $gameStateServiceEnhanced,
    $pricingService
);
$dataController = new DataController($repositories);

// Router
$router = new Router();

// Set base path for subdirectory deployment (preview environment)
if (isset($_ENV['APP_ENV']) && $_ENV['APP_ENV'] === 'preview') {
    $router->setBasePath('/planet_trader');
} else {
    $requestPath = $_SERVER['REQUEST_URI'] ?? '';
    $requestPath = parse_url($requestPath, PHP_URL_PATH) ?? '';
    $apiPos = strpos($requestPath, '/api');
    if ($apiPos !== false) {
        $basePath = substr($requestPath, 0, $apiPos);
        if ($basePath !== '') {
            $router->setBasePath($basePath);
        }
    } elseif (isset($_SERVER['SCRIPT_NAME'])) {
        $scriptName = $_SERVER['SCRIPT_NAME'];
        $basePath = str_replace('/public/index.php', '', $scriptName);
        if ($basePath !== $scriptName && $basePath !== '') {
            $router->setBasePath($basePath);
        }
    }
}

// Handle CORS preflight
if (($_SERVER['REQUEST_METHOD'] ?? 'GET') === 'OPTIONS') {
    header('Access-Control-Allow-Origin: *');
    header('Access-Control-Allow-Headers: Content-Type, Accept, Origin, Authorization');
    header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
    http_response_code(200);
    exit;
}

// Load routes
(require __DIR__ . '/../src/Routes/router.php')(
    $router,
    $gameController,
    $planetController,
    $tradingController,
    $dataController
);

// Run router
$router->handle();
