<?php

namespace App\Database;

class Connection
{
    private static ?Connection $instance = null;
    private \PDO $pdo;

    private function __construct()
    {
        $this->connect();
    }

    public static function getInstance(): Connection
    {
        if (self::$instance === null) {
            self::$instance = new self();
        }
        return self::$instance;
    }

    public function getPdo(): \PDO
    {
        return $this->pdo;
    }

    private function connect(): void
    {
        // Load .env file if it exists
        $this->loadEnvironmentFile();
        
        // Load configuration
        $configPath = __DIR__ . '/../../config/database.php';
        
        if (file_exists($configPath)) {
            $config = require $configPath;
            $db = $config['database'];
        } else {
            // Default MySQL configuration using environment variables
            $db = [
                'driver' => 'mysql',
                'host' => $_ENV['DB_HOST'] ?? 'localhost',
                'port' => $_ENV['DB_PORT'] ?? 3306,
                'database' => $_ENV['DB_NAME'] ?? $_ENV['DB_DATABASE'] ?? 'planet_trader',
                'username' => $_ENV['DB_USER'] ?? $_ENV['DB_USERNAME'] ?? 'root',
                'password' => $_ENV['DB_PASSWORD'] ?? '',
                'charset' => 'utf8mb4',
            ];
        }
        
        // Build DSN based on driver
        switch ($db['driver']) {
            case 'mysql':
                $dsn = sprintf(
                    'mysql:host=%s;port=%d;dbname=%s;charset=%s',
                    $db['host'],
                    $db['port'],
                    $db['database'],
                    $db['charset']
                );
                break;
                
            case 'sqlite':
                $dbPath = $db['database'] ?? __DIR__ . '/../../storage/database.sqlite';
                $storageDir = dirname($dbPath);
                if (!is_dir($storageDir)) {
                    mkdir($storageDir, 0755, true);
                }
                $dsn = "sqlite:" . $dbPath;
                break;
                
            default:
                throw new \Exception("Unsupported database driver: {$db['driver']}");
        }
        
        $options = [
            \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
            \PDO::ATTR_DEFAULT_FETCH_MODE => \PDO::FETCH_ASSOC,
            \PDO::ATTR_EMULATE_PREPARES => false,
        ];

        try {
            $this->pdo = new \PDO(
                $dsn, 
                $db['username'] ?? null, 
                $db['password'] ?? null, 
                $options
            );
            
            // Set charset for MySQL
            if ($db['driver'] === 'mysql') {
                $this->pdo->exec("SET NAMES {$db['charset']}");
            }
            
            // Skip automatic table initialization for migration scripts
            if (!defined('MIGRATION_SCRIPT')) {
                $this->initializeTables();
            }
            
        } catch (\PDOException $e) {
            throw new \Exception("Connection failed: " . $e->getMessage());
        }
    }
    
    /**
     * Load environment variables from .env file
     */
    private function loadEnvironmentFile(): void
    {
        $envPath = __DIR__ . '/../../.env';
        
        if (!file_exists($envPath)) {
            return;
        }
        
        $lines = file($envPath, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
        
        foreach ($lines as $line) {
            // Skip comments
            if (strpos(trim($line), '#') === 0) {
                continue;
            }
            
            // Parse key=value pairs
            if (strpos($line, '=') !== false) {
                list($key, $value) = explode('=', $line, 2);
                $key = trim($key);
                $value = trim($value);
                
                // Remove quotes if present
                $value = trim($value, '"\'');
                
                // Set environment variable if not already set
                if (!isset($_ENV[$key])) {
                    $_ENV[$key] = $value;
                    putenv("{$key}={$value}");
                }
            }
        }
    }

    private function initializeTables(): void
    {
        // Create tables if they don't exist
        $this->createPlanetTypesTable();
        $this->createSpeciesTable();
        $this->createToolsTable();
        $this->createPlayersTable();
        $this->createPlanetsTable();
        $this->createGameSessionsTable();
    }

    private function createPlanetTypesTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS planet_types (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            base_temp DECIMAL(5,2),
            base_atmo DECIMAL(3,2),
            base_water DECIMAL(3,2),
            base_grav DECIMAL(3,2),
            base_rad DECIMAL(3,2),
            color VARCHAR(7)
        )";
        
        $this->pdo->exec($sql);
    }

    private function createSpeciesTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS species (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            temp_min DECIMAL(5,2),
            temp_max DECIMAL(5,2),
            atmo_min DECIMAL(3,2),
            atmo_max DECIMAL(3,2),
            water_min DECIMAL(3,2),
            water_max DECIMAL(3,2),
            grav_min DECIMAL(3,2),
            grav_max DECIMAL(3,2),
            rad_min DECIMAL(3,2),
            rad_max DECIMAL(3,2),
            base_price INTEGER,
            color VARCHAR(7)
        )";
        
        $this->pdo->exec($sql);
    }

    private function createToolsTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS tools (
            id VARCHAR(255) PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            cost INTEGER,
            category VARCHAR(100),
            description TEXT,
            tier INTEGER DEFAULT 1,
            unlocked BOOLEAN DEFAULT TRUE,
            upgrade_required VARCHAR(255),
            effects TEXT, -- JSON string
            side_effects TEXT -- JSON string
        )";
        
        $this->pdo->exec($sql);
    }

    private function createPlayersTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS players (
            id VARCHAR(255) PRIMARY KEY,
            session_id VARCHAR(255) UNIQUE,
            credits INTEGER DEFAULT 10000,
            current_planet_id VARCHAR(255),
            game_started BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )";
        
        $this->pdo->exec($sql);
    }

    private function createPlanetsTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS planets (
            id VARCHAR(255) PRIMARY KEY,
            type_id INTEGER,
            name VARCHAR(255) NOT NULL,
            temperature DECIMAL(5,2),
            atmosphere DECIMAL(3,2),
            water DECIMAL(3,2),
            gravity DECIMAL(3,2),
            radiation DECIMAL(3,2),
            purchase_price INTEGER,
            color VARCHAR(7),
            owner_id VARCHAR(255),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sold_at TIMESTAMP NULL,
            FOREIGN KEY (type_id) REFERENCES planet_types(id),
            FOREIGN KEY (owner_id) REFERENCES players(id)
        )";
        
        $this->pdo->exec($sql);
    }

    private function createGameSessionsTable(): void
    {
        $sql = "CREATE TABLE IF NOT EXISTS game_sessions (
            id VARCHAR(255) PRIMARY KEY,
            player_id VARCHAR(255),
            started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            ended_at TIMESTAMP NULL,
            final_credits INTEGER,
            planets_traded INTEGER DEFAULT 0,
            FOREIGN KEY (player_id) REFERENCES players(id)
        )";
        
        $this->pdo->exec($sql);
    }

    public function testConnection(): bool
    {
        try {
            $stmt = $this->pdo->query("SELECT 1");
            return $stmt !== false;
        } catch (\PDOException $e) {
            return false;
        }
    }
}
