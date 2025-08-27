<?php

namespace App\Models;

class Tool
{
    public string $id;
    public string $name;
    public int $cost;
    public string $category;
    public string $description;
    public int $tier;
    public bool $unlocked;
    public ?string $upgradeRequired;
    public array $effects;
    public array $sideEffects;

    public function __construct()
    {
        $this->tier = 1;
        $this->unlocked = true;
        $this->effects = [];
        $this->sideEffects = [];
    }

    public static function fromArray(array $data): self
    {
        $tool = new self();
        $tool->id = $data['id'] ?? '';
        $tool->name = $data['name'] ?? '';
        $tool->cost = (int) ($data['cost'] ?? 0);
        $tool->category = $data['category'] ?? '';
        $tool->description = $data['description'] ?? '';
        $tool->tier = (int) ($data['tier'] ?? 1);
        $tool->unlocked = (bool) ($data['unlocked'] ?? true);
        $tool->upgradeRequired = $data['upgrade_required'] ?? null;
        
        // Parse JSON effects
        if (isset($data['effects'])) {
            $tool->effects = is_string($data['effects']) ? 
                json_decode($data['effects'], true) ?? [] : 
                $data['effects'];
        }
        
        if (isset($data['side_effects'])) {
            $tool->sideEffects = is_string($data['side_effects']) ? 
                json_decode($data['side_effects'], true) ?? [] : 
                $data['side_effects'];
        }

        return $tool;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'cost' => $this->cost,
            'category' => $this->category,
            'description' => $this->description,
            'tier' => $this->tier,
            'unlocked' => $this->unlocked,
            'upgradeRequired' => $this->upgradeRequired,
            'effects' => $this->effects,
            'sideEffects' => $this->sideEffects,
        ];
    }

    public function isAvailable(): bool
    {
        return $this->unlocked && ($this->upgradeRequired === null);
    }

    public function applyEffects(Planet $planet): void
    {
        foreach ($this->effects as $stat => $delta) {
            $this->applyStat($planet, $stat, $delta);
        }

        foreach ($this->sideEffects as $stat => $delta) {
            $this->applyStat($planet, $stat, $delta);
        }
    }

    private function applyStat(Planet $planet, string $stat, float $delta): void
    {
        switch ($stat) {
            case 'temperature':
                $planet->temperature = max(-100, min(200, $planet->temperature + $delta));
                break;
            case 'atmosphere':
                $planet->atmosphere = max(0, min(3, $planet->atmosphere + $delta));
                break;
            case 'water':
                $planet->water = max(0, min(1, $planet->water + $delta));
                break;
            case 'gravity':
                $planet->gravity = max(0.1, min(5, $planet->gravity + $delta));
                break;
            case 'radiation':
                $planet->radiation = max(0, min(2, $planet->radiation + $delta));
                break;
        }
    }
}
