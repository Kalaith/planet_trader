<?php
// Simple debug script to test backend functionality
// Access via: https://webhatchery.au/isitdoneyet/backend/public/debug.php

header('Content-Type: application/json');
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With, Accept, Origin');

echo json_encode([
    'debug_info' => [
        'php_version' => phpversion(),
        'current_directory' => getcwd(),
        'script_name' => $_SERVER['SCRIPT_NAME'] ?? 'unknown',
        'request_uri' => $_SERVER['REQUEST_URI'] ?? 'unknown',
        'request_method' => $_SERVER['REQUEST_METHOD'] ?? 'unknown',
        'server_name' => $_SERVER['SERVER_NAME'] ?? 'unknown',
        'document_root' => $_SERVER['DOCUMENT_ROOT'] ?? 'unknown',
        'path_info' => $_SERVER['PATH_INFO'] ?? 'none',
        'query_string' => $_SERVER['QUERY_STRING'] ?? 'none'
    ],
    'file_checks' => [
        'autoload_exists' => file_exists(__DIR__ . '/../vendor/autoload.php'),
        'env_exists' => file_exists(__DIR__ . '/../.env'),
        'routes_file_exists' => file_exists(__DIR__ . '/../src/Routes/api.php'),
        'index_exists' => file_exists(__DIR__ . '/index.php')
    ],
    'message' => 'Backend debug endpoint working',
    'timestamp' => date('Y-m-d H:i:s')
], JSON_PRETTY_PRINT);
?>
