<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Slim\App;
use Slim\Psr7\Factory\ServerRequestFactory;

class PlanetRoutesTest extends TestCase
{
    private App $app;

    protected function setUp(): void
    {
        $this->app = require __DIR__ . '/../../../src/Routes/routes.php';
    }

    public function testGeneratePlanets()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/planets');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetOwnedPlanets()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/planets/owned');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetCurrentPlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/planets/current');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetPlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/planets/1');
        $response = $this->app->handle($request);
        $this->assertTrue(in_array($response->getStatusCode(), [200, 404]));
    }

    public function testPurchasePlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/planets/1/purchase');
        $response = $this->app->handle($request);
        $this->assertTrue(in_array($response->getStatusCode(), [200, 400, 404]));
    }

    public function testSetCurrentPlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/planets/1/select');
        $response = $this->app->handle($request);
        $this->assertTrue(in_array($response->getStatusCode(), [200, 400, 404]));
    }

    public function testAnalyzePlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/planets/1/analyze');
        $response = $this->app->handle($request);
        $this->assertTrue(in_array($response->getStatusCode(), [200, 404]));
    }
}
