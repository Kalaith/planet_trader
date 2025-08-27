<?php

namespace App\Models;

class Planet
{
    public string $id;
    public PlanetType $type;
    public string $name;
    public float $temperature;
    public float $atmosphere;
    public float $water;
    public float $gravity;
    public float $radiation;
    public int $purchasePrice;
    public string $color;
    public ?string $ownerId = null;
    public \DateTime $createdAt;
    public ?\DateTime $soldAt = null;

    public function __construct()
    {
        $this->createdAt = new \DateTime();
    }

    public static function fromArray(array $data): self
    {
        $planet = new self();
        $planet->id = $data['id'] ?? '';
        $planet->name = $data['name'] ?? '';
        $planet->temperature = (float) ($data['temperature'] ?? 0);
        $planet->atmosphere = (float) ($data['atmosphere'] ?? 0);
        $planet->water = (float) ($data['water'] ?? 0);
        $planet->gravity = (float) ($data['gravity'] ?? 0);
        $planet->radiation = (float) ($data['radiation'] ?? 0);
        $planet->purchasePrice = (int) ($data['purchase_price'] ?? 0);
        $planet->color = $data['color'] ?? '#808080';
        $planet->ownerId = $data['owner_id'] ?? null;
        
        if (isset($data['created_at'])) {
            $planet->createdAt = new \DateTime($data['created_at']);
        }
        
        if (isset($data['sold_at'])) {
            $planet->soldAt = new \DateTime($data['sold_at']);
        }

        return $planet;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'type' => $this->type->toArray(),
            'name' => $this->name,
            'temperature' => $this->temperature,
            'atmosphere' => $this->atmosphere,
            'water' => $this->water,
            'gravity' => $this->gravity,
            'radiation' => $this->radiation,
            'purchasePrice' => $this->purchasePrice,
            'color' => $this->color,
            'ownerId' => $this->ownerId,
            'createdAt' => $this->createdAt->format('c'),
            'soldAt' => $this->soldAt?->format('c'),
        ];
    }

    public function calculateCompatibility(Species $species): float
    {
        $score = 0;
        $factors = 0;

        // Temperature compatibility
        if ($this->temperature >= $species->tempMin && $this->temperature <= $species->tempMax) {
            $score += 1;
        }
        $factors++;

        // Atmosphere compatibility
        if ($this->atmosphere >= $species->atmoMin && $this->atmosphere <= $species->atmoMax) {
            $score += 1;
        }
        $factors++;

        // Water compatibility
        if ($this->water >= $species->waterMin && $this->water <= $species->waterMax) {
            $score += 1;
        }
        $factors++;

        // Gravity compatibility
        if ($this->gravity >= $species->gravMin && $this->gravity <= $species->gravMax) {
            $score += 1;
        }
        $factors++;

        // Radiation compatibility
        if ($this->radiation >= $species->radMin && $this->radiation <= $species->radMax) {
            $score += 1;
        }
        $factors++;

        return $factors > 0 ? $score / $factors : 0;
    }
}
