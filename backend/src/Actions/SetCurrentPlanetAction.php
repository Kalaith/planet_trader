<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;

class SetCurrentPlanetAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(string $sessionId, string $planetId): array
    {
        $planets = $this->planetRepository->findBySession($sessionId);
        $found = false;
        foreach ($planets as $planet) {
            if ($planet->id === $planetId) {
                $planet->isCurrent = true;
                $this->planetRepository->save($planet);
                $found = true;
            } else if ($planet->isCurrent ?? false) {
                $planet->isCurrent = false;
                $this->planetRepository->save($planet);
            }
        }
        if (!$found) {
            return ['success' => false, 'message' => 'Planet not found in session'];
        }
        return ['success' => true, 'message' => 'Current planet set'];
    }
}
