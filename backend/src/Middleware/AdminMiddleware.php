<?php

declare(strict_types=1);

namespace App\Middleware;

use App\Core\Environment;
use App\Http\Request;
use App\Http\Response;

class AdminMiddleware
{
    public function __invoke(Request $request, Response $response, array $routeParams = []): Response|Request|bool
    {
        $authUser = $request->getAttribute('auth_user');
        if (!is_array($authUser) || empty($authUser['id'])) {
            return $this->json($response, 401, [
                'success' => false,
                'error' => 'Authentication required',
                'message' => 'Admin access requires authentication',
                'login_url' => Environment::required('WEB_HATCHERY_LOGIN_URL'),
            ]);
        }

        if ($this->isAdmin($authUser)) {
            return $request;
        }

        return $this->json($response, 403, [
            'success' => false,
            'error' => 'Forbidden',
            'message' => 'Admin role is required for this action',
        ]);
    }

    private function isAdmin(array $authUser): bool
    {
        $role = strtolower(trim((string) ($authUser['role'] ?? '')));
        if ($role === 'admin') {
            return true;
        }

        $roles = $authUser['roles'] ?? [];
        if (!is_array($roles)) {
            return false;
        }

        foreach ($roles as $candidate) {
            if (is_string($candidate) && strtolower(trim($candidate)) === 'admin') {
                return true;
            }
        }

        return false;
    }

    private function json(Response $response, int $status, array $payload): Response
    {
        $response->getBody()->write(json_encode($payload, JSON_UNESCAPED_SLASHES));
        return $response
            ->withStatus($status)
            ->withHeader('Content-Type', 'application/json');
    }
}
