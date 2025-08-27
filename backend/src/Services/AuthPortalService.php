<?php

namespace App\Services;

use Firebase\JWT\JWT;
use Firebase\JWT\Key;

class AuthPortalService
{
    private string $jwtSecret;
    public function __construct()
    {
        $this->jwtSecret = $_ENV['AUTH_PORTAL_JWT_SECRET'] ?? 'default_secret';
    }
    /**
     * @param string $token
     * @return array|null Decoded JWT payload, or null if invalid
     */
    public function validateToken(string $token): ?array
    {
        try {
            $decoded = JWT::decode($token, new Key($this->jwtSecret, 'HS256'));
            return (array) $decoded;
        } catch (\Exception $e) {
            error_log('JWT validation failed: ' . $e->getMessage());
            return null;
        }
    }
    /**
     * @param string $token
     * @return array|null User info extracted from JWT
     *   - user_id: int
     *   - email: string
     *   - username: string|null
     *   - role: string
     *   - roles: string[]
     *   - exp: int (unix timestamp)
     *   - iat: int (unix timestamp)
     */
    public function getUserFromToken(string $token): ?array
    {
        $decoded = $this->validateToken($token);
        if (!$decoded) return null;
        $roles = $decoded['roles'] ?? ['user'];
        $primaryRole = is_array($roles) ? ($roles[0] ?? 'user') : $roles;
        return [
            'user_id' => isset($decoded['sub']) ? (int)$decoded['sub'] : (isset($decoded['user_id']) ? (int)$decoded['user_id'] : null),
            'email' => $decoded['email'] ?? null,
            'username' => $decoded['username'] ?? null,
            'role' => $primaryRole,
            'roles' => $roles,
            'exp' => isset($decoded['exp']) ? (int)$decoded['exp'] : null,
            'iat' => isset($decoded['iat']) ? (int)$decoded['iat'] : null
        ];
    }
}
