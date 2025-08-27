<?php

namespace App\Database;

use App\Database\Connection;

class Seeder
{
    private \PDO $pdo;

    public function __construct()
    {
        $this->pdo = Connection::getInstance()->getPdo();
    }

    public function seed(): void
    {
        $this->seedPlanetTypes();
        $this->seedSpecies();
        $this->seedTools();
        echo "Database seeded successfully!\n";
    }

    private function seedPlanetTypes(): void
    {
        // Check if data already exists
        $stmt = $this->pdo->query("SELECT COUNT(*) FROM planet_types");
        if ($stmt->fetchColumn() > 0) {
            return; // Data already seeded
        }

        $planetTypes = [
            [
                'name' => 'Terrestrial',
                'base_temp' => 15.0,
                'base_atmo' => 1.0,
                'base_water' => 0.5,
                'base_grav' => 1.0,
                'base_rad' => 0.1,
                'color' => '#4a90e2'
            ],
            [
                'name' => 'Desert',
                'base_temp' => 35.0,
                'base_atmo' => 0.3,
                'base_water' => 0.1,
                'base_grav' => 0.8,
                'base_rad' => 0.3,
                'color' => '#d4a574'
            ],
            [
                'name' => 'Ice',
                'base_temp' => -20.0,
                'base_atmo' => 0.1,
                'base_water' => 0.9,
                'base_grav' => 1.2,
                'base_rad' => 0.05,
                'color' => '#a8d8ea'
            ],
            [
                'name' => 'Gas Giant',
                'base_temp' => -100.0,
                'base_atmo' => 3.0,
                'base_water' => 0.0,
                'base_grav' => 2.5,
                'base_rad' => 0.8,
                'color' => '#ff6b6b'
            ],
            [
                'name' => 'Volcanic',
                'base_temp' => 80.0,
                'base_atmo' => 0.8,
                'base_water' => 0.2,
                'base_grav' => 1.1,
                'base_rad' => 0.6,
                'color' => '#ff8c42'
            ]
        ];

        $stmt = $this->pdo->prepare("
            INSERT INTO planet_types (name, base_temp, base_atmo, base_water, base_grav, base_rad, color) 
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ");

        foreach ($planetTypes as $type) {
            $stmt->execute([
                $type['name'],
                $type['base_temp'],
                $type['base_atmo'],
                $type['base_water'],
                $type['base_grav'],
                $type['base_rad'],
                $type['color']
            ]);
        }
    }

    private function seedSpecies(): void
    {
        // Check if data already exists
        $stmt = $this->pdo->query("SELECT COUNT(*) FROM species");
        if ($stmt->fetchColumn() > 0) {
            return; // Data already seeded
        }

        $species = [
            [
                'name' => 'Zephyrians',
                'description' => 'Ethereal beings that thrive in low-gravity, low-radiation environments',
                'temp_min' => -50.0, 'temp_max' => 20.0,
                'atmo_min' => 0.1, 'atmo_max' => 0.8,
                'water_min' => 0.0, 'water_max' => 0.3,
                'grav_min' => 0.2, 'grav_max' => 0.8,
                'rad_min' => 0.0, 'rad_max' => 0.2,
                'base_price' => 1200,
                'color' => '#9b59b6'
            ],
            [
                'name' => 'Aquarians',
                'description' => 'Water-loving species that require high humidity and moderate temperatures',
                'temp_min' => 10.0, 'temp_max' => 40.0,
                'atmo_min' => 0.8, 'atmo_max' => 2.0,
                'water_min' => 0.6, 'water_max' => 1.0,
                'grav_min' => 0.8, 'grav_max' => 1.5,
                'rad_min' => 0.0, 'rad_max' => 0.3,
                'base_price' => 950,
                'color' => '#3498db'
            ],
            [
                'name' => 'Crystallines',
                'description' => 'Silicon-based life forms that can withstand extreme radiation',
                'temp_min' => -30.0, 'temp_max' => 100.0,
                'atmo_min' => 0.0, 'atmo_max' => 1.5,
                'water_min' => 0.0, 'water_max' => 0.2,
                'grav_min' => 0.5, 'grav_max' => 2.0,
                'rad_min' => 0.3, 'rad_max' => 2.0,
                'base_price' => 1400,
                'color' => '#e74c3c'
            ],
            [
                'name' => 'Terrans',
                'description' => 'Earth-like humanoids preferring moderate, balanced conditions',
                'temp_min' => 0.0, 'temp_max' => 35.0,
                'atmo_min' => 0.8, 'atmo_max' => 1.2,
                'water_min' => 0.3, 'water_max' => 0.7,
                'grav_min' => 0.8, 'grav_max' => 1.2,
                'rad_min' => 0.0, 'rad_max' => 0.2,
                'base_price' => 800,
                'color' => '#2ecc71'
            ],
            [
                'name' => 'Pyroclasts',
                'description' => 'Heat-resistant beings that thrive in volcanic environments',
                'temp_min' => 40.0, 'temp_max' => 150.0,
                'atmo_min' => 0.3, 'atmo_max' => 2.5,
                'water_min' => 0.0, 'water_max' => 0.4,
                'grav_min' => 0.6, 'grav_max' => 1.8,
                'rad_min' => 0.2, 'rad_max' => 1.0,
                'base_price' => 1100,
                'color' => '#f39c12'
            ],
            [
                'name' => 'Gravitons',
                'description' => 'Dense beings adapted to high-gravity environments',
                'temp_min' => -10.0, 'temp_max' => 50.0,
                'atmo_min' => 0.5, 'atmo_max' => 3.0,
                'water_min' => 0.2, 'water_max' => 0.8,
                'grav_min' => 1.5, 'grav_max' => 5.0,
                'rad_min' => 0.0, 'rad_max' => 0.5,
                'base_price' => 1300,
                'color' => '#8e44ad'
            ]
        ];

        $stmt = $this->pdo->prepare("
            INSERT INTO species (name, description, temp_min, temp_max, atmo_min, atmo_max, 
                                water_min, water_max, grav_min, grav_max, rad_min, rad_max, base_price, color) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ");

        foreach ($species as $spec) {
            $stmt->execute([
                $spec['name'],
                $spec['description'],
                $spec['temp_min'], $spec['temp_max'],
                $spec['atmo_min'], $spec['atmo_max'],
                $spec['water_min'], $spec['water_max'],
                $spec['grav_min'], $spec['grav_max'],
                $spec['rad_min'], $spec['rad_max'],
                $spec['base_price'],
                $spec['color']
            ]);
        }
    }

    private function seedTools(): void
    {
        // Check if data already exists
        $stmt = $this->pdo->query("SELECT COUNT(*) FROM tools");
        if ($stmt->fetchColumn() > 0) {
            return; // Data already seeded
        }

        $tools = [
            [
                'id' => 'atmospheric_processor',
                'name' => 'Atmospheric Processor',
                'cost' => 500,
                'category' => 'atmosphere',
                'description' => 'Increases atmospheric density',
                'tier' => 1,
                'unlocked' => true,
                'effects' => json_encode(['atmosphere' => 0.2]),
                'side_effects' => json_encode([])
            ],
            [
                'id' => 'thermal_regulator',
                'name' => 'Thermal Regulator',
                'cost' => 400,
                'category' => 'temperature',
                'description' => 'Moderates planet temperature',
                'tier' => 1,
                'unlocked' => true,
                'effects' => json_encode(['temperature' => -10]),
                'side_effects' => json_encode([])
            ],
            [
                'id' => 'water_extractor',
                'name' => 'Water Extractor',
                'cost' => 600,
                'category' => 'water',
                'description' => 'Extracts and distributes water',
                'tier' => 1,
                'unlocked' => true,
                'effects' => json_encode(['water' => 0.3]),
                'side_effects' => json_encode([])
            ],
            [
                'id' => 'gravity_stabilizer',
                'name' => 'Gravity Stabilizer',
                'cost' => 800,
                'category' => 'gravity',
                'description' => 'Adjusts gravitational field',
                'tier' => 2,
                'unlocked' => false,
                'effects' => json_encode(['gravity' => -0.2]),
                'side_effects' => json_encode([])
            ],
            [
                'id' => 'radiation_shield',
                'name' => 'Radiation Shield',
                'cost' => 700,
                'category' => 'radiation',
                'description' => 'Reduces harmful radiation',
                'tier' => 1,
                'unlocked' => true,
                'effects' => json_encode(['radiation' => -0.3]),
                'side_effects' => json_encode([])
            ]
        ];

        $stmt = $this->pdo->prepare("
            INSERT INTO tools (id, name, cost, category, description, tier, unlocked, effects, side_effects) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ");

        foreach ($tools as $tool) {
            $stmt->execute([
                $tool['id'],
                $tool['name'],
                $tool['cost'],
                $tool['category'],
                $tool['description'],
                $tool['tier'],
                $tool['unlocked'] ? 1 : 0,
                $tool['effects'],
                $tool['side_effects']
            ]);
        }
    }
}

// Script can be run directly to seed the database
if (php_sapi_name() === 'cli') {
    require_once __DIR__ . '/../../vendor/autoload.php';
    
    $seeder = new Seeder();
    $seeder->seed();
}
