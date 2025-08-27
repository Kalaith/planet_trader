<?php
namespace App\Actions;

use App\Services\SpeciesGeneratorService;

class GenerateSpeciesAction
{
    private SpeciesGeneratorService $speciesGeneratorService;

    public function __construct(SpeciesGeneratorService $speciesGeneratorService)
    {
        $this->speciesGeneratorService = $speciesGeneratorService;
    }

    /**
     * @param array $types Array of Species types
     * @param int $count Number of species to generate
     * @return array
     */
    public function execute(array $types, int $count): array
    {
        $species = $this->speciesGeneratorService->generateRandomSpecies($types, $count);
        return [
            'success' => true,
            'species' => array_map(fn($s) => $s->toArray(), $species)
        ];
    }
}
