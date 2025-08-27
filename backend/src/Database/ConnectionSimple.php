<?php

namespace App\Database;

/**
 * Simple Database Connection for Migration Scripts
 */
class Connection 
{
    private static ?Connection $instance = null;
    private \PDO $pdo;
    private array $config;
    
    private function __construct() 
    {
        $this->loadConfig();
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
    
    private function loadConfig(): void 
    {
        $configPath = __DIR__ . '/../../config/database.php';
        
        if (!file_exists($configPath)) {
            // Fallback configuration
            $this->config = [
                'database' => [
                    'driver' => 'mysql',
                    'host' => 'localhost',
                    'port' => 3306,
                    'database' => 'planet_trader',
                    'username' => 'root',
                    'password' => '',
                    'charset' => 'utf8mb4',
                    'options' => [
                        \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
                        \PDO::ATTR_DEFAULT_FETCH_MODE => \PDO::FETCH_ASSOC,
                        \PDO::ATTR_EMULATE_PREPARES => false,
                    ]
                ]
            ];
            
            echo "⚠️  Config file not found, using default MySQL settings\n";
            echo "💡 Create config/database.php to customize database settings\n";
        } else {
            $this->config = require $configPath;
        }
    }
    
    private function connect(): void 
    {
        $db = $this->config['database'];
        
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
                $dsn = 'sqlite:' . $dbPath;
                break;
                
            case 'pgsql':
                $dsn = sprintf(
                    'pgsql:host=%s;port=%d;dbname=%s',
                    $db['host'],
                    $db['port'],
                    $db['database']
                );
                break;
                
            default:
                throw new \Exception("Unsupported database driver: {$db['driver']}");
        }
        
        try {
            $this->pdo = new \PDO(
                $dsn,
                $db['username'] ?? null,
                $db['password'] ?? null,
                $db['options'] ?? []
            );
            
            // Set charset for MySQL
            if ($db['driver'] === 'mysql') {
                $this->pdo->exec("SET NAMES {$db['charset']} COLLATE {$db['collation']}");
            }
            
        } catch (\PDOException $e) {
            throw new \Exception("Database connection failed: " . $e->getMessage());
        }
    }
    
    /**
     * Test database connection
     */
    public function testConnection(): bool 
    {
        try {
            $this->pdo->query('SELECT 1');
            return true;
        } catch (\PDOException $e) {
            return false;
        }
    }
    
    /**
     * Get database information
     */
    public function getDatabaseInfo(): array 
    {
        $db = $this->config['database'];
        
        return [
            'driver' => $db['driver'],
            'host' => $db['host'] ?? 'N/A',
            'database' => $db['database'],
            'connected' => $this->testConnection()
        ];
    }
}
