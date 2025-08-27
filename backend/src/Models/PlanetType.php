<?php

namespace App\Models;

class PlanetType
{
    public int $id;
    public string $name;
    public float $baseTemp;
    public float $baseAtmo;
    public float $baseWater;
    public float $baseGrav;
    public float $baseRad;
    public string $color;

    public static function fromArray(array $data): self
    {
        $type = new self();
        $type->id = (int) ($data['id'] ?? 0);
        $type->name = $data['name'] ?? '';
        $type->baseTemp = (float) ($data['base_temp'] ?? 0);
        $type->baseAtmo = (float) ($data['base_atmo'] ?? 0);
        $type->baseWater = (float) ($data['base_water'] ?? 0);
        $type->baseGrav = (float) ($data['base_grav'] ?? 1);
        $type->baseRad = (float) ($data['base_rad'] ?? 0);
        $type->color = $data['color'] ?? '#808080';

        return $type;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'baseTemp' => $this->baseTemp,
            'baseAtmo' => $this->baseAtmo,
            'baseWater' => $this->baseWater,
            'baseGrav' => $this->baseGrav,
            'baseRad' => $this->baseRad,
            'color' => $this->color,
        ];
    }
}
