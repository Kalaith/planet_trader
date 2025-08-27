<?php
namespace App\Actions;

use App\Models\PlanetType;
use App\Models\Planet;
use App\Repositories\PlanetRepository;
use App\Utils\RandomUtils;

class CreatePlanetAction
{
    private PlanetRepository $planetRepository;

    public function __construct(PlanetRepository $planetRepository)
    {
        $this->planetRepository = $planetRepository;
    }

    public function execute(PlanetType $type, string $name): Planet
    {
        $planet = new Planet();
        $planet->id = 'planet_' . time() . '_' . bin2hex(random_bytes(4));
        $planet->type = $type;
        $planet->name = $name;
        $planet->temperature = RandomUtils::randomRange($type->baseTemp, 20);
        $planet->atmosphere = max(0, RandomUtils::randomRange($type->baseAtmo, 0.4));
        $planet->water = max(0, min(1, RandomUtils::randomRange($type->baseWater, 0.3)));
        $planet->gravity = max(0.1, RandomUtils::randomRange($type->baseGrav, 0.4));
        $planet->radiation = max(0, RandomUtils::randomRange($type->baseRad, 0.3));
        $planet->purchasePrice = (int) floor(1000 + RandomUtils::randomFloat() * 2000);
        $planet->color = $type->color;
        $planet->createdAt = new \DateTime();

        $this->planetRepository->save($planet);

        return $planet;
    }
}
