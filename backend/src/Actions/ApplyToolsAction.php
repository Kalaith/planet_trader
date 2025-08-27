<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;
use App\Repositories\PlayerRepository;

class ApplyToolsAction
{
    private PlanetRepository $planetRepository;
    private PlayerRepository $playerRepository;

    public function __construct(
        PlanetRepository $planetRepository,
        PlayerRepository $playerRepository
    ) {
        $this->planetRepository = $planetRepository;
        $this->playerRepository = $playerRepository;
    }

    public function execute(string $sessionId, string $planetId, array $toolIds): array
    {
        // Example: apply tools logic (details depend on your domain)
        $planet = $this->planetRepository->findById($planetId);
        if (!$planet) {
            return ['success' => false, 'message' => 'Planet not found'];
        }
        // ...apply tool effects to planet...
        $this->planetRepository->save($planet);
        return [
            'success' => true,
            'planet' => $planet->toArray(),
            'message' => 'Tools applied successfully'
        ];
    }
}
