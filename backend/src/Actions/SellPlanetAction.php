<?php
namespace App\Actions;

use App\Services\TradingService;

class SellPlanetAction
{
    private TradingService $tradingService;

    public function __construct(TradingService $tradingService)
    {
        $this->tradingService = $tradingService;
    }

    public function execute(string $sessionId, string $planetId, int $buyerId): array
    {
        return $this->tradingService->sellPlanet($sessionId, $planetId, $buyerId);
    }
}
