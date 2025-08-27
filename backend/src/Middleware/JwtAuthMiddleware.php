<?php

namespace App\Middleware;

use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\ServerRequestInterface;
use Psr\Http\Server\RequestHandlerInterface;
use Psr\Http\Server\MiddlewareInterface;
use App\Services\AuthPortalService;

class JwtAuthMiddleware implements MiddlewareInterface
{
    private AuthPortalService $authPortalService;

    public function __construct(AuthPortalService $authPortalService)
    {
        $this->authPortalService = $authPortalService;
    }

    public function process(ServerRequestInterface $request, RequestHandlerInterface $handler): ResponseInterface
    {
        $authHeader = $request->getHeaderLine('Authorization');
        if (empty($authHeader)) {
            return $this->unauthorizedResponse();
        }
        if (!preg_match('/Bearer\s+(.*)$/i', $authHeader, $matches)) {
            return $this->unauthorizedResponse();
        }
        $token = $matches[1];
        $authUser = $this->authPortalService->getUserFromToken($token);
        if (!$authUser || !is_int($authUser['user_id'])) {
            return $this->unauthorizedResponse();
        }
        if (isset($authUser['exp']) && $authUser['exp'] < time()) {
            return $this->unauthorizedResponse('Token expired');
        }
        $request = $request->withAttribute('auth_user', $authUser);
        return $handler->handle($request);
    }

    private function unauthorizedResponse(string $message = 'Unauthorized'): ResponseInterface
    {
        $response = new \Nyholm\Psr7\Response();
        $response->getBody()->write(json_encode([
            'success' => false,
            'message' => $message,
            'error_code' => 'UNAUTHORIZED'
        ]));
        return $response->withHeader('Content-Type', 'application/json')->withStatus(401);
    }
}
