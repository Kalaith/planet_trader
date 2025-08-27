<?php
namespace App\Actions;

use App\Services\PricingService;

class GetBuyersAction
{
    private PricingService $pricingService;

    public function __construct(PricingService $pricingService)
    {
        $this->pricingService = $pricingService;
    }

    public function execute(int $count = 4): array
    {
        $buyers = $this->pricingService->generateMarketBuyers($count);
        return array_map(fn($buyer) => $buyer->toArray(), $buyers);
    }
}
