<?php

namespace App\Controllers;

use App\Database\Connection;
use App\Http\Request;
use App\Http\Response;
use Firebase\JWT\JWT;

class AuthController
{
    public static function session(Request $request, Response $response): Response
    {
        $authUser = $request->getAttribute('auth_user');
        if (!$authUser || empty($authUser['id'])) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'error' => 'Authentication required',
                'message' => 'Unauthorized',
                'login_url' => $_ENV['WEB_HATCHERY_LOGIN_URL'] ?? ''
            ]));
            return $response->withStatus(401)->withHeader('Content-Type', 'application/json');
        }

        $payload = [
            'success' => true,
            'data' => [
                'user' => [
                    'id' => (string) $authUser['id'],
                    'email' => $authUser['email'] ?? null,
                    'username' => $authUser['username'] ?? null,
                    'display_name' => $authUser['display_name'] ?? ($authUser['username'] ?? null),
                    'roles' => $authUser['roles'] ?? [],
                    'role' => $authUser['role'] ?? 'player',
                    'is_guest' => (bool) ($authUser['is_guest'] ?? false),
                    'auth_type' => $authUser['auth_type'] ?? 'frontpage',
                ]
            ]
        ];

        $response->getBody()->write(json_encode($payload));
        return $response->withHeader('Content-Type', 'application/json');
    }

    public static function currentUser(Request $request, Response $response): Response
    {
        return self::session($request, $response);
    }

    public static function createGuestSession(Request $request, Response $response): Response
    {
        $jwtSecret = trim((string) ($_ENV['JWT_SECRET'] ?? $_SERVER['JWT_SECRET'] ?? getenv('JWT_SECRET') ?: ''));
        if ($jwtSecret === '') {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Guest session is unavailable',
                'error' => 'JWT secret is not configured',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(500);
        }

        $now = time();
        $guestId = 'guest_' . bin2hex(random_bytes(16));
        $guestTag = substr(str_replace('-', '', $guestId), 0, 8);
        $username = 'guest_' . $guestTag;

        $claims = [
            'iss' => $_ENV['JWT_ISSUER'] ?? 'webhatchery',
            'aud' => $_ENV['JWT_AUDIENCE'] ?? ($_ENV['APP_URL'] ?? 'planet-trader-app'),
            'iat' => $now,
            'nbf' => $now - 5,
            'exp' => $now + (60 * 60 * 24 * 365),
            'jti' => bin2hex(random_bytes(16)),
            'sub' => $guestId,
            'user_id' => $guestId,
            'username' => $username,
            'display_name' => 'Guest Trader',
            'email' => '',
            'role' => 'guest',
            'auth_type' => 'guest',
            'is_guest' => true,
        ];

        $token = JWT::encode($claims, $jwtSecret, 'HS256');

        $response->getBody()->write(json_encode([
            'success' => true,
            'message' => 'Guest session created',
            'data' => [
                'token' => $token,
                'user' => [
                    'id' => $guestId,
                    'email' => '',
                    'username' => $username,
                    'display_name' => 'Guest Trader',
                    'role' => 'guest',
                    'roles' => ['guest'],
                    'is_guest' => true,
                    'auth_type' => 'guest',
                ],
            ],
        ]));

        return $response->withHeader('Content-Type', 'application/json')->withStatus(201);
    }

    public static function linkGuestAccount(Request $request, Response $response): Response
    {
        $authUser = $request->getAttribute('auth_user');
        if (!$authUser || empty($authUser['id'])) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'User not authenticated',
                'error' => 'Authentication required',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(401);
        }

        $currentUserId = trim((string) ($authUser['id'] ?? ''));
        $currentRole = trim((string) ($authUser['role'] ?? 'player'));
        $isCurrentGuest = (bool) ($authUser['is_guest'] ?? false) || $currentRole === 'guest' || str_starts_with($currentUserId, 'guest_');

        if ($currentRole === 'admin') {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Guest linking is disabled for admin accounts',
                'error' => 'Guest and admin accounts cannot be linked',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(403);
        }

        if ($isCurrentGuest) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Linking requires a signed-in non-guest account',
                'error' => 'Guest destination is not allowed',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        $payload = $request->getParsedBody();
        if (!is_array($payload)) {
            $payload = json_decode((string) $request->getBody(), true);
        }

        $guestUserId = trim((string) ($payload['guest_user_id'] ?? ''));
        if ($guestUserId === '' || !str_starts_with($guestUserId, 'guest_')) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'guest_user_id must be a guest account id',
                'error' => 'Invalid guest_user_id',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        if ($guestUserId === $currentUserId) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'guest_user_id cannot match current user id',
                'error' => 'Invalid transfer request',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        $pdo = Connection::getInstance()->getPdo();
        $movedByTable = [];
        $totalMoved = 0;

        try {
            $pdo->beginTransaction();

            $guestPlayer = self::findPlayerForUser($pdo, $guestUserId);
            $currentPlayer = self::findPlayerForUser($pdo, $currentUserId);

            if ($guestPlayer && $currentPlayer && isset($guestPlayer['id'], $currentPlayer['id']) && (string) $guestPlayer['id'] !== (string) $currentPlayer['id']) {
                $movedByTable['players.session_id'] = self::updateIfPossible($pdo, 'players', 'session_id', $guestUserId, $currentUserId);
                $movedByTable['planets.owner_id'] = self::updateIfPossible($pdo, 'planets', 'owner_id', (string) $guestPlayer['id'], (string) $currentPlayer['id']);
                $movedByTable['game_sessions.player_id'] = self::updateIfPossible($pdo, 'game_sessions', 'player_id', (string) $guestPlayer['id'], (string) $currentPlayer['id']);
            } elseif ($guestPlayer) {
                $movedByTable['players.session_id'] = self::updateIfPossible($pdo, 'players', 'session_id', $guestUserId, $currentUserId);
            } else {
                $movedByTable['players.session_id'] = 0;
            }

            $movedByTable['planets.session_id'] = self::updateIfPossible($pdo, 'planets', 'session_id', $guestUserId, $currentUserId);
            $movedByTable['game_sessions.id'] = self::updateIfPossible($pdo, 'game_sessions', 'id', $guestUserId, $currentUserId, true);
            $movedByTable['transactions.session_id'] = self::updateIfPossible($pdo, 'transactions', 'session_id', $guestUserId, $currentUserId);

            foreach ($movedByTable as $count) {
                $totalMoved += (int) $count;
            }

            $pdo->commit();
        } catch (\Throwable $e) {
            if ($pdo->inTransaction()) {
                $pdo->rollBack();
            }

            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Failed to link guest data',
                'error' => $e->getMessage(),
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(500);
        }

        $response->getBody()->write(json_encode([
            'success' => true,
            'message' => 'Guest account data linked successfully',
            'data' => [
                'guest_user_id' => $guestUserId,
                'linked_to_user_id' => $currentUserId,
                'moved_rows_by_table' => $movedByTable,
                'total_moved_rows' => $totalMoved,
            ],
        ]));

        return $response->withHeader('Content-Type', 'application/json');
    }

    private static function findPlayerForUser(\PDO $pdo, string $userId): ?array
    {
        if (!self::tableExists($pdo, 'players')) {
            return null;
        }

        $sql = 'SELECT * FROM players WHERE session_id = :user_id';
        if (self::columnExists($pdo, 'players', 'id')) {
            $sql .= ' OR id = :id_match';
        }

        $stmt = $pdo->prepare($sql . ' LIMIT 1');
        $stmt->bindValue(':user_id', $userId);
        if (str_contains($sql, ':id_match')) {
            $stmt->bindValue(':id_match', $userId);
        }
        $stmt->execute();

        $result = $stmt->fetch(\PDO::FETCH_ASSOC);
        return $result ?: null;
    }

    private static function updateIfPossible(\PDO $pdo, string $table, string $column, string $fromValue, string $toValue, bool $skipOnTargetExists = false): int
    {
        if ($fromValue === $toValue || !self::tableExists($pdo, $table) || !self::columnExists($pdo, $table, $column)) {
            return 0;
        }

        if ($skipOnTargetExists) {
            $check = $pdo->prepare("SELECT COUNT(*) FROM {$table} WHERE {$column} = ?");
            $check->execute([$toValue]);
            if ((int) $check->fetchColumn() > 0) {
                return 0;
            }
        }

        $stmt = $pdo->prepare("UPDATE {$table} SET {$column} = ? WHERE {$column} = ?");
        $stmt->execute([$toValue, $fromValue]);

        return (int) $stmt->rowCount();
    }

    private static function tableExists(\PDO $pdo, string $table): bool
    {
        $driver = $pdo->getAttribute(\PDO::ATTR_DRIVER_NAME);
        if ($driver === 'sqlite') {
            $stmt = $pdo->prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?");
            $stmt->execute([$table]);
            return (bool) $stmt->fetchColumn();
        }

        $stmt = $pdo->prepare('SHOW TABLES LIKE ?');
        $stmt->execute([$table]);
        return (bool) $stmt->fetchColumn();
    }

    private static function columnExists(\PDO $pdo, string $table, string $column): bool
    {
        $driver = $pdo->getAttribute(\PDO::ATTR_DRIVER_NAME);
        if ($driver === 'sqlite') {
            $stmt = $pdo->query("PRAGMA table_info({$table})");
            $columns = $stmt ? $stmt->fetchAll(\PDO::FETCH_ASSOC) : [];
            foreach ($columns as $definition) {
                if (($definition['name'] ?? null) === $column) {
                    return true;
                }
            }
            return false;
        }

        $stmt = $pdo->prepare("SHOW COLUMNS FROM {$table} LIKE ?");
        $stmt->execute([$column]);
        return (bool) $stmt->fetchColumn();
    }
}
