<?php

declare(strict_types=1);

namespace Tests\Middleware;

use App\Http\Body;
use App\Http\Request;
use App\Http\Response;
use App\Middleware\AdminMiddleware;
use PHPUnit\Framework\TestCase;

final class AdminMiddlewareTest extends TestCase
{
    protected function setUp(): void
    {
        $_ENV['WEB_HATCHERY_LOGIN_URL'] = 'https://login.example.test';
    }

    public function testAdminRoleCanContinue(): void
    {
        $request = $this->request()->withAttribute('auth_user', [
            'id' => 'user_1',
            'role' => 'admin',
            'roles' => ['player'],
        ]);

        $result = (new AdminMiddleware())($request, new Response());

        self::assertInstanceOf(Request::class, $result);
    }

    public function testAdminRoleInRolesCanContinue(): void
    {
        $request = $this->request()->withAttribute('auth_user', [
            'id' => 'user_1',
            'role' => 'player',
            'roles' => ['player', 'admin'],
        ]);

        $result = (new AdminMiddleware())($request, new Response());

        self::assertInstanceOf(Request::class, $result);
    }

    public function testNonAdminReceivesForbidden(): void
    {
        $request = $this->request()->withAttribute('auth_user', [
            'id' => 'user_1',
            'role' => 'player',
            'roles' => ['player'],
        ]);

        $result = (new AdminMiddleware())($request, new Response());

        self::assertInstanceOf(Response::class, $result);
        self::assertSame(403, $result->getStatusCode());

        $payload = json_decode((string) $result->getBody(), true);
        self::assertSame(false, $payload['success']);
        self::assertSame('Forbidden', $payload['error']);
    }

    public function testMissingAuthReceivesLoginUrl(): void
    {
        $result = (new AdminMiddleware())($this->request(), new Response());

        self::assertInstanceOf(Response::class, $result);
        self::assertSame(401, $result->getStatusCode());

        $payload = json_decode((string) $result->getBody(), true);
        self::assertSame('https://login.example.test', $payload['login_url']);
    }

    private function request(): Request
    {
        return new Request([], [], [], [], [], 'POST', '/api/data/planet-names/reset', new Body());
    }
}
