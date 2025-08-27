<?php
namespace App\Actions;

use App\Services\GameStateServiceEnhanced;

class GetCompatibilityAction
{
    private GameStateServiceEnhanced $gameStateService;

    public function __construct(GameStateServiceEnhanced $gameStateService)
    {
        $this->gameStateService = $gameStateService;
    }

    public function execute(string $planetId, int $buyerId): array
    {
        $planet = $this->gameStateService->getPlanetById($planetId);
        $buyer = $this->gameStateService->getSpeciesById($buyerId);
        if (!$planet || !$buyer) {
            return ['success' => false, 'message' => 'Planet or buyer not found'];
        }
        $compatibility = $planet->calculateCompatibility($buyer);
        $breakdown = [
            'overall' => $compatibility,
            'temperature' => $planet->temperature >= $buyer->tempMin && $planet->temperature <= $buyer->tempMax,
            'atmosphere' => $planet->atmosphere >= $buyer->atmoMin && $planet->atmosphere <= $buyer->atmoMax,
            'water' => $planet->water >= $buyer->waterMin && $planet->water <= $buyer->waterMax,
            'gravity' => $planet->gravity >= $buyer->gravMin && $planet->gravity <= $buyer->gravMax,
            'radiation' => $planet->radiation >= $buyer->radMin && $planet->radiation <= $buyer->radMax
        ];
        return [
            'planet' => ['id' => $planet->id, 'name' => $planet->name],
            'buyer' => ['id' => $buyer->id, 'name' => $buyer->name],
            'compatibility' => $breakdown,
            'priceMultiplier' => 0.5 + ($compatibility * 1.5),
            'recommendation' => $compatibility >= 0.8 ? 'Excellent match - High profit potential' : ($compatibility >= 0.6 ? 'Good match - Solid profit expected' : ($compatibility >= 0.4 ? 'Average match - Moderate profit' : ($compatibility >= 0.2 ? 'Poor match - Low profit potential' : 'Very poor match - Consider other buyers'))),
            'success' => true
        ];
    }
}
