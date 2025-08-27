<?php

namespace App\Models;

class GameSession
{
    public string $id;
    public string $playerId;
    public \DateTime $startedAt;
    public ?\DateTime $endedAt;
    public ?int $finalCredits;
    public int $planetsTraded;

    public function __construct()
    {
        $this->startedAt = new \DateTime();
        $this->planetsTraded = 0;
    }

    public static function fromArray(array $data): self
    {
        $session = new self();
        $session->id = $data['id'] ?? '';
        $session->playerId = $data['player_id'] ?? '';
        $session->finalCredits = isset($data['final_credits']) ? (int) $data['final_credits'] : null;
        $session->planetsTraded = (int) ($data['planets_traded'] ?? 0);
        
        if (isset($data['started_at'])) {
            $session->startedAt = new \DateTime($data['started_at']);
        }
        
        if (isset($data['ended_at'])) {
            $session->endedAt = new \DateTime($data['ended_at']);
        }

        return $session;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'playerId' => $this->playerId,
            'startedAt' => $this->startedAt->format('c'),
            'endedAt' => $this->endedAt?->format('c'),
            'finalCredits' => $this->finalCredits,
            'planetsTraded' => $this->planetsTraded,
        ];
    }

    public function endSession(int $finalCredits): void
    {
        $this->endedAt = new \DateTime();
        $this->finalCredits = $finalCredits;
    }

    public function incrementPlanetsTraded(): void
    {
        $this->planetsTraded++;
    }

    public function isActive(): bool
    {
        return $this->endedAt === null;
    }

    public function getDuration(): \DateInterval
    {
        $endTime = $this->endedAt ?? new \DateTime();
        return $this->startedAt->diff($endTime);
    }
}
