<?php
namespace App\Actions;

use App\Repositories\RepositoryManager;

class ActionFactory
{
    private RepositoryManager $repositories;

    public function __construct(RepositoryManager $repositories)
    {
        $this->repositories = $repositories;
    }

    public function createPlanetAction(): CreatePlanetAction
    {
        return new CreatePlanetAction($this->repositories->planet());
    }

    public function generatePlanetOptionsAction(): GeneratePlanetOptionsAction
    {
        return new GeneratePlanetOptionsAction(
            $this->repositories->planetTypes(),
            $this->createPlanetAction()
        );
    }
}
