<?php

namespace App\Controllers;

use App\Core\Environment;
use App\Database\Connection;
use App\Http\Request;
use App\Http\Response;
use App\Repositories\GuestLinkRepository;
use Firebase\JWT\JWT;
use Firebase\JWT\Key;

class AuthController
{
    public static function loginInfo(Request $request, Response $response): Response
    {
        $response->getBody()->write(json_encode([
            'success' => true,
            'data' => [
                'login_url' => Environment::required('WEB_HATCHERY_LOGIN_URL'),
            ],
        ]));

        return $response->withHeader('Content-Type', 'application/json');
    }

    public static function session(Request $request, Response $response): Response
    {
        $authUser = $request->getAttribute('auth_user');
        if (!$authUser || empty($authUser['id'])) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'error' => 'Authentication required',
                'message' => 'Unauthorized',
                'login_url' => Environment::required('WEB_HATCHERY_LOGIN_URL')
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
        $jwtSecret = Environment::required('JWT_SECRET');

        $now = time();
        $guestId = 'guest_' . bin2hex(random_bytes(16));
        $guestTag = substr(str_replace('-', '', $guestId), 0, 8);
        $username = 'guest_' . $guestTag;

        $claims = [
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
            'roles' => ['guest'],
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

        $guestToken = trim((string) ($payload['guest_token'] ?? ''));
        if ($guestToken === '') {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'guest_token is required',
                'error' => 'Missing guest token',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        try {
            $guestClaims = (array) JWT::decode($guestToken, new Key(Environment::required('JWT_SECRET'), 'HS256'));
        } catch (\Throwable $exception) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Invalid guest token',
                'error' => 'Guest token could not be validated',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        $isGuestToken = (bool) ($guestClaims['is_guest'] ?? false) || (($guestClaims['auth_type'] ?? null) === 'guest');
        $guestUserId = trim((string) ($guestClaims['sub'] ?? $guestClaims['user_id'] ?? ''));
        if ($guestUserId === '' || !str_starts_with($guestUserId, 'guest_')) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'guest_token must identify a guest account',
                'error' => 'Invalid guest token',
            ]));

            return $response->withHeader('Content-Type', 'application/json')->withStatus(400);
        }

        if (!$isGuestToken) {
            $response->getBody()->write(json_encode([
                'success' => false,
                'message' => 'Guest token is not a guest session',
                'error' => 'Invalid guest token',
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

        try {
            $linkResult = (new GuestLinkRepository(Connection::getInstance()->getPdo()))
                ->transferGuestData($guestUserId, $currentUserId);
        } catch (\Throwable $e) {
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
                'moved_rows_by_table' => $linkResult['moved_rows_by_table'],
                'total_moved_rows' => $linkResult['total_moved_rows'],
            ],
        ]));

        return $response->withHeader('Content-Type', 'application/json');
    }
}
