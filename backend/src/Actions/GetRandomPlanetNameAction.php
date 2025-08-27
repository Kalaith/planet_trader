<?php
namespace App\Actions;

use App\Services\PlanetNameService;

class GetRandomPlanetNameAction
{
    private PlanetNameService $planetNameService;

    public function __construct(PlanetNameService $planetNameService)
    {
        $this->planetNameService = $planetNameService;
    }

    public function execute(): array
    {
        $name = $this->planetNameService->getRandomPlanetName();
        return [
            'success' => true,
            'name' => $name
        ];
    }
}
