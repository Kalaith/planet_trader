<?php
namespace App\Actions;

use App\Services\TradingService;

class GetPlayerInventoryAction
{
    private TradingService $tradingService;

    public function __construct(TradingService $tradingService)
    {
        $this->tradingService = $tradingService;
    }

    public function execute(string $sessionId): array
    {
        return $this->tradingService->getPlayerInventory($sessionId);
    }
}
