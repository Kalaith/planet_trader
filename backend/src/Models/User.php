<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Str;

class User extends Model
{
    protected $table = 'users';
    
    public $incrementing = false;
    protected $keyType = 'string';
    
    protected $fillable = [
        'email',
        'username',
        'display_name',
        'password_hash',
        'credits',
        'level',
        'experience',
        'reputation',
        'role',
        'is_active',
        'created_at',
        'updated_at'
    ];

    protected $casts = [
        'credits' => 'integer',
        'level' => 'integer', 
        'experience' => 'integer',
        'reputation' => 'integer',
        'is_active' => 'boolean',
        'created_at' => 'datetime',
        'updated_at' => 'datetime'
    ];

    protected static function boot()
    {
        parent::boot();

        static::creating(function ($model) {
            if (empty($model->id)) {
                $model->id = (string) Str::uuid();
            }
            
            // Set default game values for new users
            if (!isset($model->credits)) {
                $model->credits = 10000;
            }
            if (!isset($model->level)) {
                $model->level = 1;
            }
            if (!isset($model->experience)) {
                $model->experience = 0;
            }
            if (!isset($model->reputation)) {
                $model->reputation = 0;
            }
            if (!isset($model->role)) {
                $model->role = 'trader';
            }
            if (!isset($model->is_active)) {
                $model->is_active = true;
            }
        });
    }

    public function __construct(array $attributes = [])
    {
        parent::__construct($attributes);
        
        if (empty($this->id)) {
            $this->id = (string) Str::uuid();
        }
    }

    // Relationships
    public function gameSessions()
    {
        return $this->hasMany(GameSession::class, 'player_id');
    }

    // Game-specific helper methods
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
        $this->save();
        return true;
    }

    public function addCredits(int $amount): void
    {
        $this->credits += $amount;
        $this->save();
    }

    public function addExperience(int $amount): void
    {
        $this->experience += $amount;
        
        // Simple level calculation - level up every 1000 XP
        $newLevel = intval($this->experience / 1000) + 1;
        if ($newLevel > $this->level) {
            $this->level = $newLevel;
        }
        
        $this->save();
    }

    public function adjustReputation(int $amount): void
    {
        $this->reputation += $amount;
        $this->save();
    }

    /**
     * Get user's current level progress (percentage to next level)
     */
    public function getLevelProgress(): float
    {
        $currentLevelXP = ($this->level - 1) * 1000;
        $nextLevelXP = $this->level * 1000;
        $progressXP = $this->experience - $currentLevelXP;
        
        return ($progressXP / 1000) * 100;
    }

    /**
     * Get user's trader rank based on reputation
     */
    public function getTraderRank(): string
    {
        if ($this->reputation >= 10000) return 'Legendary Trader';
        if ($this->reputation >= 5000) return 'Master Trader';
        if ($this->reputation >= 2500) return 'Expert Trader';
        if ($this->reputation >= 1000) return 'Veteran Trader';
        if ($this->reputation >= 500) return 'Experienced Trader';
        if ($this->reputation >= 100) return 'Junior Trader';
        if ($this->reputation >= 0) return 'Novice Trader';
        
        return 'Apprentice Trader';
    }
}
