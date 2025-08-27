# Is It Done Yet? - Backend

A PHP backend for the "Is It Done Yet?" recursive project management application.

## Features

- **RESTful API** for project management
- **Hierarchical task structure** with unlimited nesting
- **Automatic progress calculation** based on completed subtasks
- **Completion propagation** - parent tasks auto-complete when all children are done
- **Robust error handling** and logging
- **CORS support** for frontend integration

## API Endpoints

### Projects
- `GET /api/projects` - Get all projects
- `GET /api/projects/{id}` - Get project by ID
- `POST /api/projects` - Create new project
- `PUT /api/projects/{id}` - Update project
- `DELETE /api/projects/{id}` - Delete project
- `POST /api/projects/{id}/complete` - Mark project complete
- `POST /api/projects/{id}/subtasks` - Add subtask to project

### System
- `GET /api/status` - API status and endpoints

## Quick Start

1. **Install Dependencies**
   ```bash
   composer install
   ```

2. **Environment Setup**
   ```bash
   cp .env.example .env
   # Edit .env with your database settings
   ```

3. **Initialize Database**
   ```bash
   php scripts/initialize-database.php
   ```

4. **Start Server**
   ```bash
   composer start
   # Server runs on http://localhost:3001
   ```

## Auth Portal Integration

This backend uses the WebHatchery Auth Portal for centralized authentication.

### Setup

1. **Install JWT dependency** (already included):
   ```bash
   composer require firebase/php-jwt
   ```

2. **Environment Variables** (.env):
   ```bash
   AUTH_PORTAL_JWT_SECRET=your_super_secret_jwt_key_change_this_in_production_2024
   AUTH_PORTAL_BASE_URL=http://127.0.0.1/auth
   AUTH_PORTAL_API_URL=http://127.0.0.1/auth/api
   AUTH_PORTAL_REDIRECT_URL=http://127.0.0.1/auth/callback
   ```

3. **Protected Endpoints**:
   - `/api/me` - Get current user info (requires JWT token)
   - All project endpoints can be protected by adding JWT middleware

### JWT Token Format

The Auth Portal provides JWT tokens with this structure:
```json
{
  "sub": 123,                    // user_id (int)
  "email": "user@example.com",   // string
  "roles": ["admin"],            // string[]
  "iat": 1754058063,             // int
  "exp": 1754144463,             // int
  "iss": "webhatchery",          // string
  "aud": "http://127.0.0.1/auth_portal" // string
}
```

### Usage

Include the JWT token in the Authorization header:
```bash
Authorization: Bearer <jwt_token>
```

## Project Structure

```
backend/
├── public/
│   └── index.php           # Application entry point
├── src/
│   ├── Actions/
│   │   └── ProjectActions.php     # Business logic
│   ├── Controllers/
│   │   └── ProjectController.php  # HTTP handlers
│   ├── External/
│   │   └── DatabaseService.php    # Database connection
│   ├── Models/
│   │   └── Project.php             # Eloquent model
│   ├── Routes/
│   │   └── api.php                 # API routes
│   └── Utils/
│       └── Logger.php              # Logging utility
├── scripts/
│   └── initialize-database.php    # Database setup
├── storage/
│   └── logs/                       # Application logs
├── composer.json
└── README.md
```

## Architecture

The backend follows the **Actions Pattern** used in the Mytherra project:

- **Controllers** handle HTTP requests/responses
- **Actions** contain business logic and database operations
- **Models** define data structure and relationships
- **External** services handle third-party integrations
- **Utils** provide common functionality

## Database Schema

### Projects Table
```sql
id              BIGINT AUTO_INCREMENT PRIMARY KEY
title           VARCHAR(255) NOT NULL
description     TEXT NULL
completed       BOOLEAN DEFAULT FALSE
parent_id       BIGINT NULL REFERENCES projects(id)
created_at      TIMESTAMP
updated_at      TIMESTAMP
```

## Frontend Integration

Update your frontend's API base URL to point to the backend:

```javascript
// In your frontend app.js or config
const API_BASE_URL = 'http://localhost:3001/api';
```

## Development

### Code Standards
- Follows PSR-12 coding standards
- Uses Actions pattern for business logic separation
- Comprehensive error handling and logging
- Type hints and proper documentation

### Testing
```bash
composer test        # Run PHPUnit tests
composer cs-check    # Check code standards
composer cs-fix      # Fix code formatting
```

## Environment Variables

```env
# Database Configuration
DB_HOST=localhost
DB_NAME=isitdoneyet
DB_USER=root
DB_PASSWORD=

# Server Configuration
PORT=3001
DEBUG=true

# CORS Configuration
CORS_ALLOWED_ORIGINS="http://localhost:3000,http://127.0.0.1:3000"
```

## Centralized WebHatchery Auth Portal Integration

This backend supports SSO via the WebHatchery Auth Portal. To enable:

1. Add to `.env`:
   ```
   AUTH_PORTAL_JWT_SECRET=your_super_secret_jwt_key_change_this_in_production_2024
   AUTH_PORTAL_BASE_URL=http://127.0.0.1/auth
   AUTH_PORTAL_API_URL=http://127.0.0.1/auth/api
   AUTH_PORTAL_REDIRECT_URL=http://127.0.0.1/auth/callback
   ```
2. Add `AuthPortalService.php` and `JwtAuthMiddleware.php` (see `src/Services` and `src/Middleware`).
3. Protect routes with JWT middleware.
4. On frontend, store JWT in `localStorage` as `token` and send as `Authorization: Bearer <token>`.
5. Use React `AuthContext` and `ProtectedRoute` for login flow.
6. On login, redirect to `/auth/login?redirect=<current_url>`.
7. On logout, clear token and redirect to `/auth/logout`.

See `auth_update.md` for full details and code samples.
