<?php

// Database Configuration for Planet Trader
// Copy this file to config.php and update with your database settings

return [
    'database' => [
        'driver' => 'mysql',  // mysql, sqlite, pgsql
        'host' => 'localhost',
        'port' => 3306,
        'database' => 'planet_trader',
        'username' => 'root',
        'password' => '',
        'charset' => 'utf8mb4',
        'collation' => 'utf8mb4_unicode_ci',
        'options' => [
            PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
            PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
            PDO::ATTR_EMULATE_PREPARES => false,
        ]
    ]
];
