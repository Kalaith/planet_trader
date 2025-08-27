#!/usr/bin/env php
<?php

/**
 * Database Connection Test Script
 * 
 * This script tests the database connection using the same configuration
 * as the migration script.
 */

// Define flag to prevent automatic table initialization
define('MIGRATION_SCRIPT', true);

require_once __DIR__ . '/../vendor/autoload.php';

use App\Database\Connection;

function testConnection() {
    try {
        echo "🔌 Testing database connection...\n";
        
        // Load environment variables manually for testing
        $envPath = __DIR__ . '/../.env';
        if (file_exists($envPath)) {
            echo "📁 Loading .env file from: $envPath\n";
            $lines = file($envPath, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
            foreach ($lines as $line) {
                if (strpos(trim($line), '#') === 0) continue;
                if (strpos($line, '=') !== false) {
                    list($key, $value) = explode('=', $line, 2);
                    $key = trim($key);
                    $value = trim($value, '"\'');
                    $_ENV[$key] = $value;
                    putenv("{$key}={$value}");
                    if (strpos($key, 'PASSWORD') === false) {
                        echo "   $key = $value\n";
                    } else {
                        echo "   $key = [HIDDEN]\n";
                    }
                }
            }
        } else {
            echo "❌ .env file not found at: $envPath\n";
        }
        
        echo "\n🔗 Attempting to connect...\n";
        $connection = Connection::getInstance();
        $pdo = $connection->getPdo();
        
        echo "✅ Database connection successful!\n";
        
        // Test a simple query
        echo "🔍 Testing database query...\n";
        $stmt = $pdo->query("SELECT DATABASE() as current_db, VERSION() as mysql_version");
        $result = $stmt->fetch();
        
        echo "📊 Connected to database: " . ($result['current_db'] ?? 'Unknown') . "\n";
        echo "🔢 MySQL version: " . ($result['mysql_version'] ?? 'Unknown') . "\n";
        
        // Check if any tables exist
        echo "📋 Checking existing tables...\n";
        $stmt = $pdo->query("SHOW TABLES");
        $tables = $stmt->fetchAll(PDO::FETCH_COLUMN);
        
        if (empty($tables)) {
            echo "📄 No tables found - ready for migration!\n";
        } else {
            echo "📊 Found " . count($tables) . " existing tables:\n";
            foreach ($tables as $table) {
                echo "   - $table\n";
            }
        }
        
        return true;
        
    } catch (\Exception $e) {
        echo "❌ Connection failed: " . $e->getMessage() . "\n";
        return false;
    }
}

// Run the test
$success = testConnection();
exit($success ? 0 : 1);
