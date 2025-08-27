<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;

class GetPlanetAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(string $planetId): ?object
    {
        return $this->planetRepository->findById($planetId);
    }
}
