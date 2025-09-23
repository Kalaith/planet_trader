<?php

namespace App\Database;

use Illuminate\Database\Capsule\Manager as Capsule;

class EloquentSetup
{
    private static bool $initialized = false;

    public static function initialize(): void
    {
        if (self::$initialized) {
            return;
        }

        // Load .env file
        self::loadEnvironmentFile();

        $capsule = new Capsule;

        // Configure the database connection
        $capsule->addConnection([
            'driver' => 'mysql',
            'host' => $_ENV['DB_HOST'] ?? 'localhost',
            'port' => $_ENV['DB_PORT'] ?? 3306,
            'database' => $_ENV['DB_NAME'] ?? 'planet_trader',
            'username' => $_ENV['DB_USER'] ?? 'root',
            'password' => $_ENV['DB_PASSWORD'] ?? '',
            'charset' => 'utf8mb4',
            'collation' => 'utf8mb4_unicode_ci',
            'prefix' => '',
        ]);

        // Set the event dispatcher used by Eloquent models
        $capsule->setEventDispatcher(new \Illuminate\Events\Dispatcher(new \Illuminate\Container\Container));

        // Make this Capsule instance available globally via static methods
        $capsule->setAsGlobal();

        // Setup the Eloquent ORM
        $capsule->bootEloquent();

        self::$initialized = true;
    }

    private static function loadEnvironmentFile(): void
    {
        $envPath = __DIR__ . '/../../.env';
        
        if (file_exists($envPath)) {
            $dotenv = \Dotenv\Dotenv::createImmutable(dirname($envPath));
            $dotenv->load();
        }
    }
}