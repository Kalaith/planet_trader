<?php
namespace App\Actions;

use App\Services\TradingService;

class BuyItemAction
{
    private TradingService $tradingService;

    public function __construct(TradingService $tradingService)
    {
        $this->tradingService = $tradingService;
    }

    public function execute(string $sessionId, string $itemId, int $quantity): array
    {
        return $this->tradingService->buyItem($sessionId, $itemId, $quantity);
    }
}
