<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;

class GetOwnedPlanetsAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(string $sessionId): array
    {
        return $this->planetRepository->findBySession($sessionId);
    }
}
