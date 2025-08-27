<?php
namespace App\Actions;

use App\Services\PricingService;

class GetMarketAction
{
    private PricingService $pricingService;

    public function __construct(PricingService $pricingService)
    {
        $this->pricingService = $pricingService;
    }

    public function execute(): array
    {
        $marketData = $this->pricingService->getMarketData();
        return [
            'market' => $marketData,
            'timestamp' => time(),
            // 'season' will be handled in controller for now
        ];
    }
}
