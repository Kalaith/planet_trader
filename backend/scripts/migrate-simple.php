#!/usr/bin/env php
<?php

/**
 * Simple Database Migration Script (Non-interactive)
 * 
 * Usage:
 *   php migrate-simple.php          - Run migrations and seed data
 *   php migrate-simple.php --no-seed - Run migrations only
 */

// Define flag to prevent automatic table initialization
define('MIGRATION_SCRIPT', true);

require_once __DIR__ . '/../vendor/autoload.php';

use App\Database\Connection;
use App\Database\MigrationRunner;
use App\Database\SeederEnhanced;

function main() {
    global $argv;
    $skipSeed = in_array('--no-seed', $argv);
    
    try {
        echo "🔌 Connecting to database...\n";
        $connection = Connection::getInstance();
        $pdo = $connection->getPdo();
        echo "✅ Database connection established\n\n";
        
        $migrationRunner = new MigrationRunner($pdo);
        
        echo "🚀 Running migrations...\n";
        $results = $migrationRunner->runMigrations();
        foreach ($results as $result) {
            echo $result . "\n";
        }
        echo "\n";
        
        if (!$skipSeed) {
            echo "🌱 Seeding database...\n";
            $seeder = new SeederEnhanced($pdo);
            $seeder->run();
            echo "\n✅ Database setup complete!\n";
        } else {
            echo "✅ Migrations complete (seeding skipped)\n";
        }
        
    } catch (\Exception $e) {
        echo "❌ Error: " . $e->getMessage() . "\n";
        echo "Stack trace:\n" . $e->getTraceAsString() . "\n";
        exit(1);
    }
}

main();
