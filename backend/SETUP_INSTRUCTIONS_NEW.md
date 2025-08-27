# Planet Trader Backend Setup Instructions

## Prerequisites

- PHP 8.1 or higher
- Composer
- MySQL 8.0 or higher
- Web server (Apache/Nginx) or PHP built-in server

## Quick Start

### 1. Install Dependencies

```bash
cd e:\WebHatchery\game_apps\planet_trader\backend
composer install
```

### 2. Configure Environment

Copy the example environment file:
```bash
copy .env.example .env
```

Edit `.env` with your database credentials:
```env
DB_HOST=localhost
DB_NAME=planet_trader
DB_USER=root
DB_PASSWORD=your_password_here
```

### 3. Create Database

Run the database creation script:
```bash
php scripts/create-database.php
```

### 4. Test Database Connection

Verify your database connection:
```bash
php scripts/test-connection.php
```

### 5. Run Migrations

Execute the migration script to set up all tables:
```bash
php scripts/migrate.php up
```

### 6. Start Development Server

```bash
composer start
# or manually:
php -S localhost:3001 -t public/
```

The API will be available at: `http://localhost:3001`

## Available Scripts

### Database Management

- **Create Database**: `php scripts/create-database.php`
- **Test Connection**: `php scripts/test-connection.php`
- **Run Migrations**: `php scripts/migrate.php up`
- **Rollback Migration**: `php scripts/migrate.php down`
- **Migration Status**: `php scripts/migrate.php status`
- **Fresh Install**: `php scripts/migrate.php fresh`

### Development

- **Start Server**: `composer start`
- **Run Tests**: `composer test`
- **Check Code Style**: `composer cs-check`
- **Fix Code Style**: `composer cs-fix`

## API Endpoints

### Game Management
- `GET /api/game/new` - Start new game
- `GET /api/game/state` - Get current game state
- `POST /api/game/save` - Save game state
- `POST /api/game/load/{id}` - Load saved game

### Planet Operations
- `GET /api/planets/generate` - Generate new planets
- `GET /api/planets/{id}` - Get planet details
- `POST /api/planets/{id}/visit` - Visit a planet

### Trading
- `POST /api/trade/buy` - Buy tools from planet
- `POST /api/trade/sell` - Sell tools to planet
- `GET /api/trade/compatibility/{toolId}/{planetId}` - Check compatibility

## Configuration Files

### Environment Variables (.env)
```env
# Database Configuration
DB_HOST=localhost
DB_NAME=planet_trader
DB_USER=root
DB_PASSWORD=your_password

# Server Configuration  
PORT=3001
DEBUG=true

# CORS Configuration
CORS_ALLOWED_ORIGINS="http://localhost:3000,http://127.0.0.1:3000"

# API Configuration
API_PREFIX=/api
API_VERSION=v1
```

### Database Configuration (config/database.php)
The system will automatically load environment variables into the database configuration. You can also manually edit this file if needed.

## Troubleshooting

### Common Issues

1. **"Connection failed" Error**
   - Check MySQL is running
   - Verify credentials in `.env`
   - Run `php scripts/test-connection.php`

2. **"Database does not exist" Error**
   - Run `php scripts/create-database.php`

3. **Migration Errors**
   - Check database permissions
   - Verify tables don't already exist
   - Run `php scripts/migrate.php fresh` for clean install

4. **Autoload Errors**
   - Run `composer install`
   - Check PSR-4 namespace mapping in composer.json

5. **Permission Errors**
   - Ensure PHP has write access to storage/ directory
   - Check MySQL user permissions

### Debug Mode

Enable debug mode in `.env`:
```env
DEBUG=true
```

This will provide detailed error messages and logging.

## Project Structure

```
backend/
├── config/           # Configuration files
├── public/           # Web server document root
├── scripts/          # CLI scripts (migrations, etc.)
├── src/              # Application source code
│   ├── Controllers/  # HTTP request handlers
│   ├── Services/     # Business logic
│   ├── Models/       # Data models
│   ├── Repositories/ # Data access layer
│   ├── Database/     # Database connections & migrations
│   └── Utils/        # Helper utilities
├── storage/          # Writable storage (logs, cache)
├── tests/            # Test suites
├── vendor/           # Composer dependencies
├── .env              # Environment configuration
└── composer.json     # PHP dependencies
```

## Next Steps

1. ✅ Set up backend infrastructure
2. ✅ Create database and run migrations  
3. 🔄 Test API endpoints with Bruno/Postman
4. 🔄 Update frontend to use new API
5. 🔄 Add comprehensive error handling
6. 🔄 Implement logging and monitoring
7. 🔄 Add unit and integration tests
8. 🔄 Optimize for production deployment
