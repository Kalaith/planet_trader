#!/usr/bin/env php
<?php

/**
 * Database Migration CLI Script
 * 
 * Usage:
 *   php migrate.php up        - Run all pending migrations
 *   php migrate.php down      - Rollback the last migration
 *   php migrate.php status    - Show migration status
 *   php migrate.php fresh     - Drop all tables and re-run migrations
 */

// Define flag to prevent automatic table initialization
define('MIGRATION_SCRIPT', true);

// Try to load Composer autoloader
require_once __DIR__ . '/../vendor/autoload.php';

use App\Database\Connection;
use App\Database\MigrationRunner;
use App\Database\SeederEnhanced;

function main() {
    global $argv;
    $command = $argv[1] ?? 'up';
    
    try {
        echo "🔌 Connecting to database...\n";
        $connection = Connection::getInstance();
        $pdo = $connection->getPdo();
        echo "✅ Database connection established\n\n";
        
        $migrationRunner = new MigrationRunner($pdo);
        
        switch ($command) {
            case 'up':
                echo "🚀 Running migrations...\n\n";
                $results = $migrationRunner->runMigrations();
                foreach ($results as $result) {
                    echo $result . "\n";
                }
                echo "\n";
                
                // Ask if user wants to seed data
                echo "Would you like to seed the database with sample data? (y/n): ";
                $handle = fopen("php://stdin", "r");
                $response = trim(fgets($handle));
                fclose($handle);
                
                if (strtolower($response) === 'y' || strtolower($response) === 'yes') {
                    echo "\n🌱 Seeding database...\n";
                    $seeder = new SeederEnhanced($pdo);
                    $seeder->run();
                    echo "\n✅ Database seeded successfully\n";
                }
                break;
                
            case 'down':
                echo "⏪ Rolling back last migration...\n";
                $result = $migrationRunner->rollbackLastMigration();
                echo $result . "\n";
                break;
                
            case 'status':
                echo "📊 Migration Status:\n\n";
                $status = $migrationRunner->getStatus();
                if (empty($status)) {
                    echo "No migrations executed yet.\n";
                } else {
                    foreach ($status as $migration) {
                        echo "✅ {$migration['migration_name']} - {$migration['executed_at']}\n";
                    }
                }
                break;
                
            case 'fresh':
                echo "⚠️  This will DROP ALL TABLES and re-run migrations.\n";
                echo "Are you sure? (yes/no): ";
                $handle = fopen("php://stdin", "r");
                $response = trim(fgets($handle));
                fclose($handle);
                
                if (strtolower($response) === 'yes') {
                    echo "\n🗑️  Dropping all tables...\n";
                    $migrationRunner->rollbackLastMigration();
                    
                    echo "🚀 Re-running migrations...\n";
                    $results = $migrationRunner->runMigrations();
                    foreach ($results as $result) {
                        echo $result . "\n";
                    }
                    
                    echo "\n🌱 Seeding database...\n";
                    $seeder = new SeederEnhanced($pdo);
                    $seeder->run();
                    echo "\n✅ Fresh database setup complete\n";
                } else {
                    echo "Operation cancelled.\n";
                }
                break;
                
            default:
                echo "❌ Unknown command: {$command}\n";
                echo "Available commands: up, down, status, fresh\n";
                exit(1);
        }
        
    } catch (\Exception $e) {
        echo "❌ Error: " . $e->getMessage() . "\n";
        echo "Stack trace:\n" . $e->getTraceAsString() . "\n";
        exit(1);
    }
}

main();
