<?php

namespace App\Utils;

class ValidationUtils
{
    /**
     * Validate that a value is within a numeric range
     */
    public static function validateRange(float $value, float $min, float $max, string $fieldName = 'Value'): bool
    {
        if ($value < $min || $value > $max) {
            throw new \InvalidArgumentException("{$fieldName} must be between {$min} and {$max}");
        }
        return true;
    }

    /**
     * Validate that a string is not empty
     */
    public static function validateNotEmpty(string $value, string $fieldName = 'Value'): bool
    {
        if (trim($value) === '') {
            throw new \InvalidArgumentException("{$fieldName} cannot be empty");
        }
        return true;
    }

    /**
     * Validate session ID format
     */
    public static function validateSessionId(string $sessionId): bool
    {
        if (!preg_match('/^[a-zA-Z0-9_-]+$/', $sessionId) || strlen($sessionId) < 8) {
            throw new \InvalidArgumentException('Invalid session ID format');
        }
        return true;
    }

    /**
     * Validate planet ID format
     */
    public static function validatePlanetId(string $planetId): bool
    {
        if (!preg_match('/^planet_\d+_[a-f0-9]+$/', $planetId)) {
            throw new \InvalidArgumentException('Invalid planet ID format');
        }
        return true;
    }

    /**
     * Validate color hex format
     */
    public static function validateHexColor(string $color): bool
    {
        if (!preg_match('/^#[a-fA-F0-9]{6}$/', $color)) {
            throw new \InvalidArgumentException('Invalid hex color format');
        }
        return true;
    }

    /**
     * Validate credits amount
     */
    public static function validateCredits(int $credits): bool
    {
        if ($credits < 0) {
            throw new \InvalidArgumentException('Credits cannot be negative');
        }
        return true;
    }

    /**
     * Validate planet properties
     */
    public static function validatePlanetProperties(array $properties): bool
    {
        $validations = [
            'temperature' => [-100, 200],
            'atmosphere' => [0, 3],
            'water' => [0, 1],
            'gravity' => [0.1, 5],
            'radiation' => [0, 2],
        ];

        foreach ($validations as $property => $range) {
            if (isset($properties[$property])) {
                self::validateRange(
                    $properties[$property], 
                    $range[0], 
                    $range[1], 
                    ucfirst($property)
                );
            }
        }

        return true;
    }

    /**
     * Sanitize string input
     */
    public static function sanitizeString(string $input): string
    {
        return htmlspecialchars(trim($input), ENT_QUOTES, 'UTF-8');
    }

    /**
     * Validate email format
     */
    public static function validateEmail(string $email): bool
    {
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new \InvalidArgumentException('Invalid email format');
        }
        return true;
    }

    /**
     * Validate array structure
     */
    public static function validateArrayStructure(array $data, array $requiredKeys): bool
    {
        foreach ($requiredKeys as $key) {
            if (!array_key_exists($key, $data)) {
                throw new \InvalidArgumentException("Required key '{$key}' is missing");
            }
        }
        return true;
    }

    /**
     * Validate JSON string
     */
    public static function validateJson(string $json): bool
    {
        json_decode($json);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON format: ' . json_last_error_msg());
        }
        return true;
    }

    /**
     * Validate positive integer
     */
    public static function validatePositiveInteger(int $value, string $fieldName = 'Value'): bool
    {
        if ($value <= 0) {
            throw new \InvalidArgumentException("{$fieldName} must be a positive integer");
        }
        return true;
    }
}
