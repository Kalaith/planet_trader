<?php

namespace App\Utils;

class RandomUtils
{
    /**
     * Get a random item from an array
     */
    public static function randomItem(array $arr): mixed
    {
        if (empty($arr)) {
            throw new \InvalidArgumentException('Array cannot be empty');
        }
        return $arr[array_rand($arr)];
    }

    /**
     * Generate a random float between min and max
     */
    public static function randomFloat(float $min = 0.0, float $max = 1.0): float
    {
        return $min + mt_rand() / mt_getrandmax() * ($max - $min);
    }

    /**
     * Generate a random value within a range around a base value
     */
    public static function randomRange(float $base, float $variance): float
    {
        return $base + (self::randomFloat() - 0.5) * $variance;
    }

    /**
     * Generate a random integer between min and max (inclusive)
     */
    public static function randomInt(int $min, int $max): int
    {
        return rand($min, $max);
    }

    /**
     * Generate a random boolean with given probability
     */
    public static function randomBool(float $probability = 0.5): bool
    {
        return self::randomFloat() < $probability;
    }

    /**
     * Shuffle an array and return the first $count items
     */
    public static function randomSample(array $array, int $count): array
    {
        if ($count >= count($array)) {
            return $array;
        }

        $shuffled = $array;
        shuffle($shuffled);
        return array_slice($shuffled, 0, $count);
    }

    /**
     * Generate a unique ID
     */
    public static function generateId(string $prefix = ''): string
    {
        $timestamp = time();
        $random = bin2hex(random_bytes(4));
        return $prefix ? "{$prefix}_{$timestamp}_{$random}" : "{$timestamp}_{$random}";
    }

    /**
     * Generate a weighted random selection
     */
    public static function weightedRandom(array $weights): mixed
    {
        $totalWeight = array_sum($weights);
        $randomValue = self::randomFloat() * $totalWeight;
        
        $currentWeight = 0;
        foreach ($weights as $key => $weight) {
            $currentWeight += $weight;
            if ($randomValue <= $currentWeight) {
                return $key;
            }
        }
        
        // Fallback to last item
        return array_key_last($weights);
    }

    /**
     * Generate a random string of given length
     */
    public static function randomString(int $length = 8, string $chars = 'abcdefghijklmnopqrstuvwxyz0123456789'): string
    {
        $result = '';
        $charsLength = strlen($chars);
        
        for ($i = 0; $i < $length; $i++) {
            $result .= $chars[rand(0, $charsLength - 1)];
        }
        
        return $result;
    }
}
