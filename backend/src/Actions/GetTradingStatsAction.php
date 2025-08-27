<?php
namespace App\Actions;

use App\Services\TradingService;

class GetTradingStatsAction
{
    private TradingService $tradingService;

    public function __construct(TradingService $tradingService)
    {
        $this->tradingService = $tradingService;
    }

    public function execute(string $sessionId): array
    {
        return $this->tradingService->getTradingStats($sessionId);
    }
}
