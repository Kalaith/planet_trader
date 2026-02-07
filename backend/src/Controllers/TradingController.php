<?php

namespace App\Controllers;

use App\Services\TradingService;
use App\Services\GameStateServiceEnhanced;
use App\Services\PricingService;
use App\Utils\RandomUtils;
use App\Http\Response;
use App\Http\Request;

class TradingController extends BaseController
{
    private TradingService $tradingService;
    private GameStateServiceEnhanced $gameStateService;
    private PricingService $pricingService;

    public function __construct(
        TradingService $tradingService,
        GameStateServiceEnhanced $gameStateService,
        PricingService $pricingService
    ) {
        $this->tradingService = $tradingService;
        $this->gameStateService = $gameStateService;
        $this->pricingService = $pricingService;
    }

    /**
     * Get current alien buyers in the market
     */
    public function getBuyers(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $queryParams = $request->getQueryParams();
            $count = max(1, min(8, (int) ($queryParams['count'] ?? 4))); // Between 1-8 buyers
            
            // Generate random buyers using pricing service
            $buyers = $this->pricingService->generateMarketBuyers($count);
            
            if (empty($buyers)) {
                return $this->errorResponse($response, 'No buyers available', 500);
            }

            // Convert to array format
            $buyersData = array_map(fn($buyer) => $buyer->toArray(), $buyers);
            
            $this->logAction('buyers_requested', [
                'session_id' => $sessionId,
                'count' => count($buyersData)
            ]);
            
            return $this->successResponse($response, [
                'buyers' => $buyersData,
                'count' => count($buyersData),
                'refreshTime' => time() + 300 // Buyers refresh every 5 minutes
            ], 'Current alien buyers retrieved');

        } catch (\Exception $e) {
            $this->logAction('buyers_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get buyers: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Sell a planet to an alien buyer
     */
    public function sellPlanet(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            $data = $this->getJsonBody($request);
            
            $this->validateRequired($data, ['planetId', 'buyerId']);
            
            $planetId = $data['planetId'];
            $buyerId = (int) $data['buyerId'];
            
            $result = $this->tradingService->sellPlanet($sessionId, $planetId, $buyerId);
            
            if (!$result['success']) {
                return $this->errorResponse($response, $result['message'], 400);
            }

            $this->logAction('planet_sold', [
                'session_id' => $sessionId,
                'planet_id' => $planetId,
                'buyer_id' => $buyerId,
                'sale_price' => $result['salePrice']
            ]);
            
            return $this->successResponse($response, $result, $result['message']);

        } catch (\Exception $e) {
            $this->logAction('planet_sale_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to sell planet: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Calculate potential profit for selling a planet
     */
    public function calculateProfit(Request $request, Response $response): Response
    {
        try {
            $queryParams = $request->getQueryParams();
            
            $planetId = $queryParams['planetId'] ?? '';
            $buyerId = (int) ($queryParams['buyerId'] ?? 0);
            
            if (empty($planetId) || $buyerId === 0) {
                return $this->errorResponse($response, 'Planet ID and Buyer ID are required', 400);
            }

            $profitData = $this->tradingService->calculatePotentialProfit($planetId, $buyerId);
            
            if (!$profitData['success']) {
                return $this->errorResponse($response, $profitData['message'], 400);
            }

            return $this->successResponse($response, $profitData, 'Profit calculation completed');

        } catch (\Exception $e) {
            $this->logAction('profit_calculation_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to calculate profit: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get market prices and trends
     */
    public function getMarket(Request $request, Response $response): Response
    {
        try {
            // Get market data using pricing service
            $marketData = $this->pricingService->getMarketData();
            
            return $this->successResponse($response, [
                'market' => $marketData,
                'timestamp' => time(),
                'season' => $this->getCurrentSeason()
            ], 'Market data retrieved');

        } catch (\Exception $e) {
            $this->logAction('market_data_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get market data: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get trading statistics for the current player
     */
    public function getTradingStats(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            
            $stats = $this->tradingService->getTradingStats($sessionId);
            
            return $this->successResponse($response, $stats, 'Trading statistics retrieved');

        } catch (\Exception $e) {
            $this->logAction('trading_stats_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get trading stats: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get compatibility analysis between a planet and buyer
     */
    public function getCompatibility(Request $request, Response $response): Response
    {
        try {
            $queryParams = $request->getQueryParams();
            
            $planetId = $queryParams['planetId'] ?? '';
            $buyerId = (int) ($queryParams['buyerId'] ?? 0);
            
            if (empty($planetId) || $buyerId === 0) {
                return $this->errorResponse($response, 'Planet ID and Buyer ID are required', 400);
            }

            $planet = $this->gameStateService->getPlanetById($planetId);
            $buyer = $this->gameStateService->getSpeciesById($buyerId);
            
            if (!$planet || !$buyer) {
                return $this->errorResponse($response, 'Planet or buyer not found', 404);
            }

            $compatibility = $planet->calculateCompatibility($buyer);
            
            // Detailed compatibility breakdown
            $breakdown = [
                'overall' => $compatibility,
                'temperature' => $this->checkRange($planet->temperature, $buyer->tempMin, $buyer->tempMax),
                'atmosphere' => $this->checkRange($planet->atmosphere, $buyer->atmoMin, $buyer->atmoMax),
                'water' => $this->checkRange($planet->water, $buyer->waterMin, $buyer->waterMax),
                'gravity' => $this->checkRange($planet->gravity, $buyer->gravMin, $buyer->gravMax),
                'radiation' => $this->checkRange($planet->radiation, $buyer->radMin, $buyer->radMax)
            ];
            
            $compatibilityData = [
                'planet' => [
                    'id' => $planet->id,
                    'name' => $planet->name
                ],
                'buyer' => [
                    'id' => $buyer->id,
                    'name' => $buyer->name
                ],
                'compatibility' => $breakdown,
                'priceMultiplier' => 0.5 + ($compatibility * 1.5),
                'recommendation' => $this->getRecommendation($compatibility)
            ];
            
            return $this->successResponse($response, $compatibilityData, 'Compatibility analysis completed');

        } catch (\Exception $e) {
            $this->logAction('compatibility_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to analyze compatibility: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Get trade history for the player
     */
    public function getTradeHistory(Request $request, Response $response): Response
    {
        try {
            $sessionId = $this->getSessionId($request);
            [$page, $perPage] = $this->getPaginationParams($request);
            
            // TODO: Implement trade history storage and retrieval
            // For now, return empty history
            $tradeHistory = [];
            
            $paginatedData = $this->paginate($tradeHistory, $page, $perPage);
            
            return $this->successResponse($response, $paginatedData, 'Trade history retrieved');

        } catch (\Exception $e) {
            $this->logAction('trade_history_error', ['error' => $e->getMessage()]);
            return $this->errorResponse($response, 'Failed to get trade history: ' . $e->getMessage(), 500);
        }
    }

    /**
     * Handle CORS preflight requests
     */
    public function options(Request $request, Response $response): Response
    {
        return $this->handleOptions($request, $response);
    }

    /**
     * Check if a value is within a range
     */
    private function checkRange(float $value, float $min, float $max): bool
    {
        return $value >= $min && $value <= $max;
    }

    /**
     * Get trading recommendation based on compatibility
     */
    private function getRecommendation(float $compatibility): string
    {
        if ($compatibility >= 0.8) {
            return 'Excellent match - High profit potential';
        } elseif ($compatibility >= 0.6) {
            return 'Good match - Solid profit expected';
        } elseif ($compatibility >= 0.4) {
            return 'Average match - Moderate profit';
        } elseif ($compatibility >= 0.2) {
            return 'Poor match - Low profit potential';
        } else {
            return 'Very poor match - Consider other buyers';
        }
    }

    /**
     * Get current trading season
     */
    private function getCurrentSeason(): string
    {
        $seasons = ['spring', 'summer', 'autumn', 'winter'];
        $index = (time() / 300) % 4; // Change season every 5 minutes for demo
        return $seasons[(int) $index];
    }
}
