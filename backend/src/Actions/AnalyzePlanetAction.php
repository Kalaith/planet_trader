<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;

class AnalyzePlanetAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(string $planetId): array
    {
        $planet = $this->planetRepository->findById($planetId);
        if (!$planet) {
            return ['success' => false, 'message' => 'Planet not found'];
        }
        $value = $planet->purchasePrice * 1.2; // Example: estimated value
        $rarity = ($planet->water > 0.7 && $planet->radiation < 0.1) ? 'Rare' : 'Common';
        $analysis = [
            'planet' => $planet->toArray(),
            'estimatedValue' => $value,
            'rarity' => $rarity,
            'profitPotential' => $value - $planet->purchasePrice,
            'characteristics' => [
                'habitable' => $planet->temperature >= -10 && $planet->temperature <= 40 && $planet->atmosphere >= 0.5 && $planet->atmosphere <= 2.0 && $planet->water >= 0.1 && $planet->gravity >= 0.5 && $planet->gravity <= 2.0 && $planet->radiation <= 0.5,
                'waterRich' => $planet->water > 0.5,
                'lowRadiation' => $planet->radiation < 0.2,
                'earthLikeGravity' => abs($planet->gravity - 1.0) < 0.3,
                'temperateClimate' => $planet->temperature >= 0 && $planet->temperature <= 30
            ]
        ];
        return ['success' => true, 'analysis' => $analysis, 'message' => 'Planet analysis completed'];
    }
}
