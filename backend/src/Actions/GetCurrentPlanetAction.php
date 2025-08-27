<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;

class GetCurrentPlanetAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(string $sessionId): ?object
    {
        $planets = $this->planetRepository->findBySession($sessionId);
        foreach ($planets as $planet) {
            if ($planet->isCurrent ?? false) {
                return $planet;
            }
        }
        return null;
    }
}
