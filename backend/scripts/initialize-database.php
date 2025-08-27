<?php

require_once __DIR__ . '/../vendor/autoload.php';

use App\External\DatabaseService;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Database\Capsule\Manager as Capsule;

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__ . '/../');
$dotenv->load();

echo "Initializing database...\n";

// Initialize database connection
$dbService = new DatabaseService();

try {
    // Test connection
    if (!$dbService->testConnection()) {
        throw new Exception('Could not connect to database');
    }
    
    echo "✓ Database connection established\n";
  
    // Create tables
} catch (Exception $e) {
    echo "✗ Error: " . $e->getMessage() . "\n";
    exit(1);
}