#!/usr/bin/env php
<?php

/**
 * Database Creation Script
 * 
 * This script creates the planet_trader database if it doesn't exist.
 * Run this before running migrations.
 */

require_once __DIR__ . '/../vendor/autoload.php';

function createDatabase() {
    try {
        // Load environment variables
        $envPath = __DIR__ . '/../.env';
        if (file_exists($envPath)) {
            echo "📁 Loading .env file...\n";
            $lines = file($envPath, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
            foreach ($lines as $line) {
                if (strpos(trim($line), '#') === 0) continue;
                if (strpos($line, '=') !== false) {
                    list($key, $value) = explode('=', $line, 2);
                    $key = trim($key);
                    $value = trim($value, '"\'');
                    $_ENV[$key] = $value;
                    putenv("{$key}={$value}");
                }
            }
        }
        
        $host = $_ENV['DB_HOST'] ?? 'localhost';
        $port = $_ENV['DB_PORT'] ?? 3306;
        $username = $_ENV['DB_USER'] ?? $_ENV['DB_USERNAME'] ?? 'root';
        $password = $_ENV['DB_PASSWORD'] ?? '';
        $database = $_ENV['DB_NAME'] ?? $_ENV['DB_DATABASE'] ?? 'planet_trader';
        
        echo "🔌 Connecting to MySQL server...\n";
        echo "   Host: $host:$port\n";
        echo "   User: $username\n";
        echo "   Target Database: $database\n";
        
        // Connect without specifying database
        $dsn = "mysql:host=$host;port=$port;charset=utf8mb4";
        $options = [
            PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
            PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
        ];
        
        $pdo = new PDO($dsn, $username, $password, $options);
        echo "✅ Connected to MySQL server\n";
        
        // Check if database exists
        echo "🔍 Checking if database '$database' exists...\n";
        $stmt = $pdo->prepare("SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = ?");
        $stmt->execute([$database]);
        $exists = $stmt->fetch();
        
        if ($exists) {
            echo "✅ Database '$database' already exists\n";
        } else {
            echo "📦 Creating database '$database'...\n";
            $pdo->exec("CREATE DATABASE `$database` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
            echo "✅ Database '$database' created successfully\n";
        }
        
        // Test connection to the new database
        echo "🔗 Testing connection to '$database'...\n";
        $testDsn = "mysql:host=$host;port=$port;dbname=$database;charset=utf8mb4";
        $testPdo = new PDO($testDsn, $username, $password, $options);
        echo "✅ Successfully connected to database '$database'\n";
        
        return true;
        
    } catch (\Exception $e) {
        echo "❌ Error: " . $e->getMessage() . "\n";
        return false;
    }
}

// Run the database creation
$success = createDatabase();
exit($success ? 0 : 1);
