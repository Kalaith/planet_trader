<?php

namespace App\Middleware;

use App\Http\Request;
use App\Http\Response;
use Firebase\JWT\JWT;
use Firebase\JWT\Key;

class WebHatcheryJwtMiddleware
{
    public function __invoke(Request $request, Response $response, array $routeParams = []): Response|Request|bool
    {
        $authHeader = $request->getHeaderLine('Authorization');
        if (!$authHeader || !preg_match('/Bearer\s+(.*)$/i', $authHeader, $matches)) {
            return $this->unauthorized($response, 'Authorization header missing or invalid');
        }

        $token = trim((string) $matches[1]);
        $secret = $_ENV['JWT_SECRET']
            ?? $_SERVER['JWT_SECRET']
            ?? getenv('JWT_SECRET')
            ?: '';
        if ($secret === '') {
            return $this->unauthorized($response, 'JWT secret not configured');
        }

        try {
            $decoded = JWT::decode($token, new Key($secret, 'HS256'));

            $expectedIssuer = $_ENV['JWT_ISSUER'] ?? 'webhatchery';
            if (isset($decoded->iss) && $decoded->iss !== $expectedIssuer) {
                return $this->unauthorized($response, 'Invalid token issuer');
            }

            $expectedAudience = $_ENV['JWT_AUDIENCE'] ?? ($_ENV['APP_URL'] ?? null);
            if ($expectedAudience && isset($decoded->aud)) {
                $aud = $decoded->aud;
                $isValidAudience = is_array($aud) ? in_array($expectedAudience, $aud, true) : $aud === $expectedAudience;
                if (!$isValidAudience) {
                    return $this->unauthorized($response, 'Invalid token audience');
                }
            }

            $userId = $decoded->sub ?? $decoded->user_id ?? null;
            if (!$userId) {
                return $this->unauthorized($response, 'Token missing user identifier');
            }

            $isGuest = $this->extractBool($decoded->is_guest ?? false) || (($decoded->auth_type ?? null) === 'guest');
            $role = $isGuest ? 'guest' : 'player';
            if (isset($decoded->role) && is_string($decoded->role) && trim($decoded->role) !== '') {
                $role = $isGuest ? 'guest' : trim($decoded->role);
            }

            $request = $request->withAttribute('auth_user', [
                'id' => (string) $userId,
                'email' => $decoded->email ?? null,
                'username' => $decoded->username ?? null,
                'display_name' => $decoded->display_name ?? ($decoded->username ?? null),
                'roles' => $decoded->roles ?? ($isGuest ? ['guest'] : []),
                'role' => $role,
                'is_guest' => $isGuest,
                'auth_type' => $isGuest ? 'guest' : 'frontpage',
            ]);

            return $request;
        } catch (\Exception $e) {
            return $this->unauthorized($response, 'Invalid token');
        }
    }

    private function extractBool(mixed $value): bool
    {
        if (is_bool($value)) {
            return $value;
        }
        if (is_numeric($value)) {
            return (int) $value === 1;
        }
        if (is_string($value)) {
            return in_array(strtolower(trim($value)), ['1', 'true', 'yes'], true);
        }
        return false;
    }

    private function unauthorized(Response $response, string $message): Response
    {
        $loginUrl = $_ENV['WEB_HATCHERY_LOGIN_URL'] ?? '';
        $payload = [
            'success' => false,
            'error' => 'Authentication required',
            'message' => $message,
            'login_url' => $loginUrl
        ];
        $response->getBody()->write(json_encode($payload));
        return $response
            ->withStatus(401)
            ->withHeader('Content-Type', 'application/json');
    }
}
