<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Slim\App;
use Slim\Psr7\Factory\ServerRequestFactory;

class TradingRoutesTest extends TestCase
{
    private App $app;

    protected function setUp(): void
    {
        $this->app = require __DIR__ . '/../../../src/Routes/routes.php';
    }

    public function testGetBuyers()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/buyers');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testSellPlanet()
    {
        $request = (new ServerRequestFactory())->createServerRequest('POST', '/api/trading/sell');
        $response = $this->app->handle($request);
        $this->assertTrue(in_array($response->getStatusCode(), [200, 400]));
    }

    public function testCalculateProfit()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/profit');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetMarket()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/market');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetTradingStats()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/stats');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetCompatibility()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/compatibility');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }

    public function testGetTradeHistory()
    {
        $request = (new ServerRequestFactory())->createServerRequest('GET', '/api/trading/history');
        $response = $this->app->handle($request);
        $this->assertEquals(200, $response->getStatusCode());
    }
}
