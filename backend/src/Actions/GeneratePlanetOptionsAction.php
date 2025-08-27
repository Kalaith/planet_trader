<?php
namespace App\Actions;

use App\Repositories\PlanetRepository;
use App\Repositories\PlanetTypeRepository;
use App\Actions\CreatePlanetAction;

class GeneratePlanetOptionsAction
{
    private PlanetTypeRepository $planetTypeRepository;
    private CreatePlanetAction $createPlanetAction;

    public function __construct(
        PlanetTypeRepository $planetTypeRepository,
        CreatePlanetAction $createPlanetAction
    ) {
        $this->planetTypeRepository = $planetTypeRepository;
        $this->createPlanetAction = $createPlanetAction;
    }

    public function execute(int $count = 3): array
    {
        $planetTypes = $this->planetTypeRepository->getAll();
        $planets = [];
        for ($i = 0; $i < $count; $i++) {
            $type = $planetTypes[array_rand($planetTypes)];
            $name = 'Planet ' . strtoupper(bin2hex(random_bytes(2)));
            $planets[] = $this->createPlanetAction->execute($type, $name);
        }
        return $planets;
    }
}
