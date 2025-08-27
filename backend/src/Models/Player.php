<?php

namespace App\Models;

class Player
{
    public string $id;
    public string $sessionId;
    public int $credits;
    public ?string $currentPlanetId;
    public bool $gameStarted;
    public \DateTime $createdAt;
    public \DateTime $lastActivity;

    public function __construct()
    {
        $this->credits = 10000; // Default starting credits
        $this->gameStarted = false;
        $this->createdAt = new \DateTime();
        $this->lastActivity = new \DateTime();
    }

    public static function fromArray(array $data): self
    {
        $player = new self();
        $player->id = $data['id'] ?? '';
        $player->sessionId = $data['session_id'] ?? '';
        $player->credits = (int) ($data['credits'] ?? 10000);
        $player->currentPlanetId = $data['current_planet_id'] ?? null;
        $player->gameStarted = (bool) ($data['game_started'] ?? false);
        
        if (isset($data['created_at'])) {
            $player->createdAt = new \DateTime($data['created_at']);
        }
        
        if (isset($data['last_activity'])) {
            $player->lastActivity = new \DateTime($data['last_activity']);
        }

        return $player;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'sessionId' => $this->sessionId,
            'credits' => $this->credits,
            'currentPlanetId' => $this->currentPlanetId,
            'gameStarted' => $this->gameStarted,
            'createdAt' => $this->createdAt->format('c'),
            'lastActivity' => $this->lastActivity->format('c'),
        ];
    }

    public function updateActivity(): void
    {
        $this->lastActivity = new \DateTime();
    }

    public function canAfford(int $amount): bool
    {
        return $this->credits >= $amount;
    }

    public function spendCredits(int $amount): bool
    {
        if (!$this->canAfford($amount)) {
            return false;
        }
        
        $this->credits -= $amount;
        $this->updateActivity();
        return true;
    }

    public function addCredits(int $amount): void
    {
        $this->credits += $amount;
        $this->updateActivity();
    }
}
