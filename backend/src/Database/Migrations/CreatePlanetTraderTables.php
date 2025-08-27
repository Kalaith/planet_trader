<?php

namespace App\Database\Migrations;

/**
 * Migration for creating all Planet Trader database tables
 */
class CreatePlanetTraderTables 
{
    private $pdo;
    
    public function __construct($pdo) 
    {
        $this->pdo = $pdo;
    }
    
    public function up(): void 
    {
        // Create planet_types table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS planet_types (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(50) NOT NULL UNIQUE,
                description TEXT,
                base_value_multiplier DECIMAL(4,2) DEFAULT 1.00,
                rarity_weight INT DEFAULT 1,
                color_scheme JSON,
                atmosphere_types JSON,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ");
        
        // Create species table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS species (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                type VARCHAR(50) NOT NULL,
                description TEXT,
                preferred_planet_types JSON,
                appearance JSON,
                value_multiplier DECIMAL(4,2) DEFAULT 1.00,
                rarity_weight INT DEFAULT 1,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ");
        
        // Create tools table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS tools (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                description TEXT,
                effect_type VARCHAR(50) NOT NULL,
                effect_value DECIMAL(10,2),
                cost INT NOT NULL,
                unlock_requirements JSON,
                applicable_planet_types JSON,
                research_tier INT DEFAULT 1,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ");
        
        // Create players table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS players (
                id INT AUTO_INCREMENT PRIMARY KEY,
                username VARCHAR(50) NOT NULL UNIQUE,
                email VARCHAR(100) UNIQUE,
                credits BIGINT DEFAULT 10000,
                experience_points INT DEFAULT 0,
                level INT DEFAULT 1,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
        ");
        
        // Create game_sessions table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS game_sessions (
                id VARCHAR(36) PRIMARY KEY,
                player_id INT,
                current_credits BIGINT DEFAULT 10000,
                current_planet_id INT,
                planets_visited INT DEFAULT 0,
                total_profit BIGINT DEFAULT 0,
                session_data JSON,
                is_active BOOLEAN DEFAULT TRUE,
                started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE SET NULL
            )
        ");
        
        // Create planets table
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS planets (
                id INT AUTO_INCREMENT PRIMARY KEY,
                session_id VARCHAR(36),
                name VARCHAR(100) NOT NULL,
                planet_type_id INT NOT NULL,
                size ENUM('small', 'medium', 'large') DEFAULT 'medium',
                atmosphere VARCHAR(50),
                base_value INT NOT NULL,
                current_value INT NOT NULL,
                species_id INT,
                position_x DECIMAL(10,6),
                position_y DECIMAL(10,6),
                tools_applied JSON,
                discovery_order INT,
                is_sold BOOLEAN DEFAULT FALSE,
                sold_for INT DEFAULT NULL,
                sold_at TIMESTAMP NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (session_id) REFERENCES game_sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (planet_type_id) REFERENCES planet_types(id),
                FOREIGN KEY (species_id) REFERENCES species(id) ON DELETE SET NULL
            )
        ");
        
        // Create player_tools table (many-to-many for owned tools)
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS player_tools (
                id INT AUTO_INCREMENT PRIMARY KEY,
                player_id INT NOT NULL,
                tool_id INT NOT NULL,
                quantity INT DEFAULT 1,
                acquired_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
                FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE,
                UNIQUE KEY unique_player_tool (player_id, tool_id)
            )
        ");
        
        // Create planet_names table for name generation
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS planet_names (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(100) NOT NULL UNIQUE,
                origin VARCHAR(50),
                category VARCHAR(50) DEFAULT 'general',
                is_used BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ");
        
        // Create transactions table for trading history
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS transactions (
                id INT AUTO_INCREMENT PRIMARY KEY,
                session_id VARCHAR(36) NOT NULL,
                planet_id INT NOT NULL,
                transaction_type ENUM('buy', 'sell') NOT NULL,
                amount BIGINT NOT NULL,
                profit_loss BIGINT DEFAULT 0,
                tools_used JSON,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (session_id) REFERENCES game_sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (planet_id) REFERENCES planets(id) ON DELETE CASCADE
            )
        ");
        
        // Create indexes for better performance
        $this->pdo->exec("CREATE INDEX idx_planets_session ON planets(session_id)");
        $this->pdo->exec("CREATE INDEX idx_planets_type ON planets(planet_type_id)");
        $this->pdo->exec("CREATE INDEX idx_planets_species ON planets(species_id)");
        $this->pdo->exec("CREATE INDEX idx_transactions_session ON transactions(session_id)");
        $this->pdo->exec("CREATE INDEX idx_game_sessions_player ON game_sessions(player_id)");
        $this->pdo->exec("CREATE INDEX idx_game_sessions_active ON game_sessions(is_active)");
    }
    
    public function down(): void 
    {
        // Drop tables in reverse order to avoid foreign key constraints
        $tables = [
            'transactions',
            'planet_names', 
            'player_tools',
            'planets',
            'game_sessions',
            'players',
            'tools',
            'species',
            'planet_types'
        ];
        
        foreach ($tables as $table) {
            $this->pdo->exec("DROP TABLE IF EXISTS {$table}");
        }
    }
}
