# Planet Trader Backend Setup Instructions

## 🚀 Quick Setup Guide

### 1. Database Configuration

**Option A: Use config/database.php (Recommended)**
Update `backend/config/database.php` with your MySQL credentials:
```php
return [
    'database' => [
        'driver' => 'mysql',
        'host' => 'localhost',
        'port' => 3306,
        'database' => 'planet_trader',
        'username' => 'your_username',
        'password' => 'your_password',
        'charset' => 'utf8mb4',
    ]
];
```

**Option B: Use Environment Variables**
Copy `.env.example` to `.env` and update:
```
DB_HOST=localhost
DB_PORT=3306
DB_DATABASE=planet_trader
DB_USERNAME=root
DB_PASSWORD=your_password
```

### 2. Install Dependencies & Run Migration

```powershell
# Navigate to backend directory
cd e:\WebHatchery\game_apps\planet_trader\backend

# Install Composer dependencies (if not already done)
composer install

# Run migration with automatic seeding
php scripts/migrate.php up
```

## 🔧 Configuration Options

### Database Settings (`config/database.php`)

**MySQL Example:**
```php
return [
    'database' => [
        'driver' => 'mysql',
        'host' => 'localhost',
        'port' => 3306,
        'database' => 'planet_trader',
        'username' => 'your_username',
        'password' => 'your_password',
        'charset' => 'utf8mb4',
        'collation' => 'utf8mb4_unicode_ci'
    ]
];
```

**SQLite Example:**
```php
return [
    'database' => [
        'driver' => 'sqlite',
        'database' => __DIR__ . '/../storage/database.sqlite'
    ]
];
```

## 📋 Migration Commands

```powershell
# Run all pending migrations
php scripts/migrate.php up

# Show migration status
php scripts/migrate.php status

# Rollback last migration
php scripts/migrate.php down

# Fresh setup (drops all tables and re-creates)
php scripts/migrate.php fresh
```

## ⚠️ Troubleshooting

### Error: "Database connection failed"
- Check your database credentials in `config/database.php`
- Ensure MySQL/MariaDB service is running
- Verify the database exists

### Error: "Table doesn't exist"
- Run `php scripts/migrate.php up` to create tables

### Error: "Composer autoloader not found"
- This is normal - the script will work with manual includes
- Optionally run `composer install` for better autoloading

## 🎯 What the Migration Does

1. **Creates Tables:**
   - `planet_types` - Different types of planets
   - `species` - Alien species information
   - `tools` - Terraforming tools and effects
   - `players` - Player accounts
   - `game_sessions` - Active game sessions
   - `planets` - Generated planets for each session
   - `transactions` - Trading history
   - `planet_names` - Pool of planet names

2. **Seeds Data:**
   - Loads all planet types from your JSON files
   - Imports alien species with preferences
   - Sets up terraforming tools with effects
   - Populates planet name pool

3. **Sets Up Indexes:**
   - Optimizes common queries
   - Ensures referential integrity

## 📊 Verification

After running the migration, you should see output like:
```
✅ Migration 'create_planet_trader_tables' executed successfully
🌱 Seeding planet types...
👽 Seeding alien species...
🔧 Seeding terraforming tools...
🪐 Seeding planet names...
✅ Database seeded successfully
```

## 🔗 Next Steps

Once the database is set up:
1. Update controllers to use new repository services
2. Test API endpoints
3. Update frontend to use new API structure

Ready to transform your Planet Trader game! 🚀
