<?php

use App\Database\Connection;
use App\Utils\Logger;
use App\Services\AuthPortalService;
use App\Middleware\JwtAuthMiddleware;

require __DIR__ . '/../vendor/autoload.php';

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__ . '/../');
$dotenv->load();

// Set CORS headers
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With, Accept, Origin');
header('Content-Type: application/json');

// Handle preflight OPTIONS requests
if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
    http_response_code(200);
    exit;
}

// Initialize services
try {
    // Test database connection
    $dbConnection = Connection::getInstance();
    
    $logger = new Logger();
    
    // Initialize auth services
    $authPortalService = new AuthPortalService();
    $jwtAuthMiddleware = new JwtAuthMiddleware($authPortalService);
    
} catch (Exception $e) {
    http_response_code(500);
    echo json_encode(['success' => false, 'message' => $e->getMessage()]);
    exit;
}

// Simple routing
$requestUri = $_SERVER['REQUEST_URI'];
$requestMethod = $_SERVER['REQUEST_METHOD'];
$path = parse_url($requestUri, PHP_URL_PATH);

// Extract path after /api/
if (preg_match('#/api/(.*)#', $path, $matches)) {
    $route = $matches[1];
} else {
    $route = '';
}

// Route handling
if ($route === 'status') {
    echo json_encode([
        'status' => 'OK',
        'service' => 'Planet Trader API - Simple PHP',
        'version' => '1.0.0'
    ]);
} elseif ($route === 'me') {
    // Auth endpoint - requires JWT token
    // Check multiple sources for Authorization header
    $authHeader = '';
    if (isset($_SERVER['HTTP_AUTHORIZATION'])) {
        $authHeader = $_SERVER['HTTP_AUTHORIZATION'];
    } elseif (isset($_SERVER['REDIRECT_HTTP_AUTHORIZATION'])) {
        $authHeader = $_SERVER['REDIRECT_HTTP_AUTHORIZATION'];
    } elseif (function_exists('apache_request_headers')) {
        $headers = apache_request_headers();
        if (isset($headers['Authorization'])) {
            $authHeader = $headers['Authorization'];
        } elseif (isset($headers['authorization'])) {
            $authHeader = $headers['authorization'];
        }
    }
    
    if (empty($authHeader)) {
        http_response_code(401);
        echo json_encode(['success' => false, 'message' => 'Authorization header required']);
        exit;
    }
    
    if (!preg_match('/Bearer\s+(.*)$/i', $authHeader, $matches)) {
        http_response_code(401);
        echo json_encode(['success' => false, 'message' => 'Invalid Authorization header format']);
        exit;
    }
    
    $token = $matches[1];
    $authUser = $authPortalService->getUserFromToken($token);
    
    if (!$authUser || !is_int($authUser['user_id'])) {
        http_response_code(401);
        echo json_encode(['success' => false, 'message' => 'Invalid or expired token']);
        exit;
    }
    
    // Return user info
    echo json_encode([
        'success' => true,
        'data' => $authUser
    ]);
} elseif ($route === 'test-db') {
    // Test database connection
    try {
        $pdo = $dbConnection->getPdo();
        $stmt = $pdo->query('SELECT 1');
        
        echo json_encode([
            'success' => true,
            'message' => 'Database connection successful',
            'timestamp' => date('Y-m-d H:i:s')
        ]);
    } catch (Exception $e) {
        http_response_code(500);
        echo json_encode([
            'success' => false,
            'message' => 'Database connection failed: ' . $e->getMessage()
        ]);
    }
} else {
    // Route not found
    http_response_code(404);
    echo json_encode([
        'success' => false,
        'message' => 'Route not found'
    ]);
}
