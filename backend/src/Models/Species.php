<?php

namespace App\Models;

class Species
{
    public int $id;
    public string $name;
    public string $description;
    public float $tempMin;
    public float $tempMax;
    public float $atmoMin;
    public float $atmoMax;
    public float $waterMin;
    public float $waterMax;
    public float $gravMin;
    public float $gravMax;
    public float $radMin;
    public float $radMax;
    public int $basePrice;
    public string $color;
    public ?int $currentPrice = null;

    public static function fromArray(array $data): self
    {
        $species = new self();
        $species->id = (int) ($data['id'] ?? 0);
        $species->name = $data['name'] ?? '';
        $species->description = $data['description'] ?? '';
        $species->tempMin = (float) ($data['temp_min'] ?? 0);
        $species->tempMax = (float) ($data['temp_max'] ?? 0);
        $species->atmoMin = (float) ($data['atmo_min'] ?? 0);
        $species->atmoMax = (float) ($data['atmo_max'] ?? 0);
        $species->waterMin = (float) ($data['water_min'] ?? 0);
        $species->waterMax = (float) ($data['water_max'] ?? 0);
        $species->gravMin = (float) ($data['grav_min'] ?? 0);
        $species->gravMax = (float) ($data['grav_max'] ?? 0);
        $species->radMin = (float) ($data['rad_min'] ?? 0);
        $species->radMax = (float) ($data['rad_max'] ?? 0);
        $species->basePrice = (int) ($data['base_price'] ?? 0);
        $species->color = $data['color'] ?? '#808080';

        return $species;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'description' => $this->description,
            'tempRange' => [$this->tempMin, $this->tempMax],
            'atmoRange' => [$this->atmoMin, $this->atmoMax],
            'waterRange' => [$this->waterMin, $this->waterMax],
            'gravRange' => [$this->gravMin, $this->gravMax],
            'radRange' => [$this->radMin, $this->radMax],
            'basePrice' => $this->basePrice,
            'currentPrice' => $this->currentPrice,
            'color' => $this->color,
        ];
    }

    public function generateCurrentPrice(): int
    {
        // Add price variation: ±25% of base price
        $variation = rand(-250, 250);
        $this->currentPrice = $this->basePrice + $variation;
        return $this->currentPrice;
    }
}
