<?php
namespace App\Actions;

use App\Services\TradingService;

class CalculateProfitAction
{
    private TradingService $tradingService;

    public function __construct(TradingService $tradingService)
    {
        $this->tradingService = $tradingService;
    }

    public function execute(string $planetId, int $buyerId): array
    {
        return $this->tradingService->calculatePotentialProfit($planetId, $buyerId);
    }
}
