# Planet Trader Backend - Troubleshooting Guide

This guide helps you diagnose and fix common issues when setting up the Planet Trader backend.

## Step-by-Step Troubleshooting

### Step 1: Test Database Connection

Before running migrations, test your database connection:

```bash
php scripts/test-connection.php
```

**Expected Output:**
```
🔌 Testing database connection...
📁 Loading .env file from: /path/to/.env
   DB_HOST = localhost
   DB_NAME = planet_trader
   DB_USER = root
   DB_PASSWORD = [HIDDEN]
   
🔗 Attempting to connect...
✅ Database connection successful!
🔍 Testing database query...
📊 Connected to database: planet_trader
🔢 MySQL version: 8.0.x
📋 Checking existing tables...
📄 No tables found - ready for migration!
```

### Step 2: Create Database if Needed

If the test connection fails with "database does not exist":

```bash
php scripts/create-database.php
```

**Expected Output:**
```
📁 Loading .env file...
🔌 Connecting to MySQL server...
   Host: localhost:3306
   User: root
   Target Database: planet_trader
✅ Connected to MySQL server
🔍 Checking if database 'planet_trader' exists...
📦 Creating database 'planet_trader'...
✅ Database 'planet_trader' created successfully
🔗 Testing connection to 'planet_trader'...
✅ Successfully connected to database 'planet_trader'
```

### Step 3: Run Migrations

Once the database connection is working:

```bash
php scripts/migrate.php up
```

**Expected Output:**
```
🔌 Connecting to database...
✅ Connected successfully
🗃️ Initializing migrations table...
📋 Checking migration status...
⚡ Running migration: CreatePlanetTraderTables
✅ Migration CreatePlanetTraderTables completed successfully
🌱 Running database seeder...
📊 Seeding planet_types: 6 records
📊 Seeding species: 5 records  
📊 Seeding tools: 10 records
✅ Database seeded successfully
```

## Common Error Messages & Solutions

### 1. "Connection failed: SQLSTATE[HY000] [1045] Access denied"

**Problem:** Incorrect MySQL credentials

**Solutions:**
- Check your `.env` file has the correct password
- Verify MySQL user exists and has correct permissions
- Test MySQL connection directly: `mysql -u root -p`

### 2. "Connection failed: SQLSTATE[HY000] [2002] No such file or directory"

**Problem:** MySQL server is not running

**Solutions:**
- Start MySQL service
- Check MySQL is running on the correct port (default: 3306)
- Verify DB_HOST and DB_PORT in `.env`

### 3. "Connection failed: SQLSTATE[HY000] [1049] Unknown database"

**Problem:** Database doesn't exist

**Solution:**
```bash
php scripts/create-database.php
```

### 4. "Class 'App\Database\Connection' not found"

**Problem:** Composer autoloader not working

**Solutions:**
```bash
composer install
composer dump-autoload
```

### 5. "Parse error: syntax error, unexpected..."

**Problem:** PHP version incompatibility

**Solutions:**
- Check PHP version: `php -v`
- Ensure PHP 8.1+ is installed
- Update PHP if necessary

### 6. "SQLSTATE[42S01]: Base table or view already exists"

**Problem:** Tables already exist from previous setup

**Solutions:**
```bash
# Option 1: Fresh migration (drops all tables)
php scripts/migrate.php fresh

# Option 2: Manual cleanup
mysql -u root -p planet_trader -e "DROP DATABASE planet_trader;"
php scripts/create-database.php
php scripts/migrate.php up
```

### 7. "File not found" or "Permission denied"

**Problem:** File permissions or missing files

**Solutions:**
```bash
# Check file exists
ls -la scripts/migrate.php

# Fix permissions if needed
chmod +x scripts/*.php

# Ensure storage directory exists and is writable
mkdir -p storage/logs
chmod 755 storage/
```

## Debugging Steps

### Enable Debug Mode

Add to your `.env` file:
```env
DEBUG=true
```

### Check PHP Configuration

```bash
php -m | grep pdo
php -m | grep mysql
```

Should show:
- PDO
- pdo_mysql
- mysqli

### Verify Composer Dependencies

```bash
composer validate
composer install --no-dev --optimize-autoloader
```

### Test PHP Version Compatibility

```bash
php -v
php -c /path/to/php.ini -l scripts/migrate.php
```

### Check MySQL Permissions

```sql
-- Connect to MySQL as root
mysql -u root -p

-- Check user privileges
SHOW GRANTS FOR 'root'@'localhost';

-- Create user if needed
CREATE USER 'planet_trader'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON planet_trader.* TO 'planet_trader'@'localhost';
FLUSH PRIVILEGES;
```

## Manual Database Setup

If scripts fail, you can set up manually:

### 1. Create Database
```sql
CREATE DATABASE planet_trader CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE planet_trader;
```

### 2. Create Tables
```sql
-- Copy SQL from: src/Database/Migrations/CreatePlanetTraderTables.php
-- Run each CREATE TABLE statement manually
```

### 3. Insert Seed Data
```sql
-- Insert data from JSON files in data/ directory
-- Or run: php scripts/migrate.php seed
```

## Environment Configuration Reference

### Complete .env Example
```env
# Database Configuration
DB_HOST=localhost
DB_PORT=3306
DB_NAME=planet_trader
DB_USER=root
DB_PASSWORD=your_mysql_password

# Server Configuration
PORT=3001
DEBUG=true

# CORS Configuration
CORS_ALLOWED_ORIGINS="http://localhost:3000,http://127.0.0.1:3000"

# API Configuration
API_PREFIX=/api
API_VERSION=v1

# Logging
LOG_LEVEL=debug
LOG_FILE=storage/logs/app.log

# Security
JWT_SECRET=your_jwt_secret_here
ENCRYPTION_KEY=your_32_character_encryption_key
```

## Getting Help

If you're still having issues:

1. Run the connection test: `php scripts/test-connection.php`
2. Check the exact error message
3. Verify all prerequisites are met
4. Try a fresh installation
5. Check the logs in `storage/logs/`

## Success Indicators

When everything is working correctly:

1. ✅ `php scripts/test-connection.php` - Shows successful connection
2. ✅ `php scripts/migrate.php status` - Shows completed migrations
3. ✅ `composer start` - Starts server without errors
4. ✅ `curl http://localhost:3001/` - Returns response

Your backend is ready when all these steps complete successfully!
