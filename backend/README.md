# Planet Trader Backend

PHP API backend for Planet Trader.

## Setup

1. Install dependencies:
   ```bash
   composer install
   ```

2. Configure the required environment variables in `.env`:
   ```env
   DB_HOST=localhost
   DB_PORT=3306
   DB_NAME=planet_trader
   DB_USER=planet_trader_user
   DB_PASSWORD=change_me
   JWT_SECRET=shared_web_hatchery_jwt_secret
   WEB_HATCHERY_LOGIN_URL=http://127.0.0.1/auth/login
   WEB_HATCHERY_REGISTER_URL=http://127.0.0.1/auth/register
   ```

3. Apply the schema migration:
   ```bash
   mysql -u "$DB_USER" -p "$DB_NAME" < database/001_create_planet_trader_tables.sql
   ```

   The PHP migration runner can also create the schema:
   ```bash
   php scripts/migrate-simple.php --no-seed
   ```

## Auth

Protected routes require `Authorization: Bearer <token>` from Web Hatchery Login or a backend-created guest session. Unauthenticated requests return `401` JSON with `login_url`.

Admin-only maintenance routes require an `admin` role in either `role` or `roles`.

## Routes

- `GET /api/auth/login-info`
- `POST /api/auth/guest-session`
- `POST /api/auth/link-guest`
- `GET /api/game/status`
- `POST /api/game/start`
- `POST /api/game/reset`
- `POST /api/game/end`
- `GET /api/game/stats`
- `GET /api/data/planet-types`
- `GET /api/data/species`
- `GET /api/data/tools`
- `GET /api/data/planet-names`
- `POST /api/data/planet-names/reset` requires admin

`POST /api/game/start` and `POST /api/game/reset` use server-controlled starting credits. The client cannot set credits directly.

## Checks

```bash
composer test
composer cs-check
```
