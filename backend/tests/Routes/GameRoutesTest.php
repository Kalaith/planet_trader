<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Slim\App;
use Slim\Psr7\Factory\ServerRequestFactory;

class GameRoutesTest extends TestCase
{
    private App $app;

    protected function setUp(): void
    {
        $this->app = require __DIR__ . '/../../../src/Routes/routes.php';
    }

    public function testGetStatus()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/game/status');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testStartGame()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/game/start');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testEndGame()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/game/end');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testResetGame()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/game/reset');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetStats()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/game/stats');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testUpdateCredits()
    {
        $request = (new ServerRequestFactory())->createServerRequest('PUT', '/api/game/credits');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }
}
