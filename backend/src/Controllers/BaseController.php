<?php

namespace App\Controllers;

use App\Http\Response;
use App\Http\Request;

abstract class BaseController
{
    /**
     * Return a JSON response
     */
    protected function jsonResponse(Response $response, array $data, int $status = 200): Response
    {
        $response->getBody()->write(json_encode($data, JSON_UNESCAPED_SLASHES));
        return $response
            ->withHeader('Content-Type', 'application/json')
            ->withStatus($status);
    }

    /**
     * Return an error response
     */
    protected function errorResponse(Response $response, string $message, int $status = 400): Response
    {
        return $this->jsonResponse($response, [
            'success' => false,
            'error' => $message,
            'timestamp' => date('c')
        ], $status);
    }

    /**
     * Return a success response
     */
    protected function successResponse(Response $response, array $data = [], string $message = 'Success'): Response
    {
        return $this->jsonResponse($response, [
            'success' => true,
            'message' => $message,
            'data' => $data,
            'timestamp' => date('c')
        ]);
    }

    /**
     * Get session ID from request headers or query params
     */
    protected function getSessionId(Request $request): string
    {
        $authUser = $request->getAttribute('auth_user');
        if ($authUser && isset($authUser['id'])) {
            return (string) $authUser['id'];
        }

        // Try to get from header first
        $sessionHeader = $request->getHeaderLine('X-Session-ID');
        if (!empty($sessionHeader)) {
            return $sessionHeader;
        }

        // Try to get from query parameters
        $queryParams = $request->getQueryParams();
        if (isset($queryParams['session_id'])) {
            return $queryParams['session_id'];
        }

        // Try to get from cookies
        $cookies = $request->getCookieParams();
        if (isset($cookies['session_id'])) {
            return $cookies['session_id'];
        }

        // Generate new session ID if none found
        return $this->generateSessionId();
    }

    /**
     * Generate a new session ID
     */
    protected function generateSessionId(): string
    {
        return 'session_' . time() . '_' . bin2hex(random_bytes(8));
    }

    /**
     * Parse JSON body from request
     */
    protected function getJsonBody(Request $request): array
    {
        $body = $request->getBody()->getContents();
        
        if (empty($body)) {
            return [];
        }

        $data = json_decode($body, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON in request body');
        }

        return $data ?? [];
    }

    /**
     * Validate required parameters
     */
    protected function validateRequired(array $data, array $required): void
    {
        foreach ($required as $field) {
            if (!isset($data[$field]) || $data[$field] === '') {
                throw new \InvalidArgumentException("Required field '{$field}' is missing");
            }
        }
    }

    /**
     * Add CORS headers to response
     */
    protected function addCorsHeaders(Response $response): Response
    {
        return $response
            ->withHeader('Access-Control-Allow-Origin', '*')
            ->withHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
            ->withHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-Session-ID')
            ->withHeader('Access-Control-Max-Age', '3600');
    }

    /**
     * Handle OPTIONS request for CORS
     */
    protected function handleOptions(Request $request, Response $response): Response
    {
        return $this->addCorsHeaders($response)->withStatus(200);
    }

    /**
     * Log an action for debugging
     */
    protected function logAction(string $action, array $context = []): void
    {
        // In a real application, you would use a proper logger
        error_log(sprintf(
            '[%s] %s: %s',
            date('Y-m-d H:i:s'),
            $action,
            json_encode($context)
        ));
    }

    /**
     * Paginate results
     */
    protected function paginate(array $items, int $page = 1, int $perPage = 10): array
    {
        $total = count($items);
        $offset = ($page - 1) * $perPage;
        $paginatedItems = array_slice($items, $offset, $perPage);
        
        return [
            'items' => $paginatedItems,
            'pagination' => [
                'current_page' => $page,
                'per_page' => $perPage,
                'total_items' => $total,
                'total_pages' => ceil($total / $perPage),
                'has_next' => $page < ceil($total / $perPage),
                'has_prev' => $page > 1
            ]
        ];
    }

    /**
     * Extract pagination parameters from request
     */
    protected function getPaginationParams(Request $request): array
    {
        $queryParams = $request->getQueryParams();
        
        $page = max(1, (int) ($queryParams['page'] ?? 1));
        $perPage = max(1, min(100, (int) ($queryParams['per_page'] ?? 10))); // Max 100 items per page
        
        return [$page, $perPage];
    }
}
