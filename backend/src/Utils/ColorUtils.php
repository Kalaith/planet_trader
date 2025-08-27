<?php

namespace App\Utils;

use App\Models\Planet;

class ColorUtils
{
    /**
     * Convert hex color to RGB array
     */
    public static function hexToRgb(string $hex): array
    {
        $hex = ltrim($hex, '#');
        
        if (strlen($hex) === 6) {
            return [
                'r' => hexdec(substr($hex, 0, 2)),
                'g' => hexdec(substr($hex, 2, 2)),
                'b' => hexdec(substr($hex, 4, 2))
            ];
        }
        
        return ['r' => 0, 'g' => 0, 'b' => 0];
    }

    /**
     * Convert RGB to HSL
     */
    public static function rgbToHsl(int $r, int $g, int $b): array
    {
        $r /= 255;
        $g /= 255;
        $b /= 255;
        
        $max = max($r, $g, $b);
        $min = min($r, $g, $b);
        $diff = $max - $min;
        
        $l = ($max + $min) / 2;
        
        if ($diff === 0) {
            $h = $s = 0;
        } else {
            $s = $l > 0.5 ? $diff / (2 - $max - $min) : $diff / ($max + $min);
            
            switch ($max) {
                case $r:
                    $h = (($g - $b) / $diff) + ($g < $b ? 6 : 0);
                    break;
                case $g:
                    $h = ($b - $r) / $diff + 2;
                    break;
                case $b:
                    $h = ($r - $g) / $diff + 4;
                    break;
                default:
                    $h = 0;
            }
            $h /= 6;
        }
        
        return [
            'h' => $h * 360,
            's' => $s,
            'l' => $l
        ];
    }

    /**
     * Convert HSL to RGB
     */
    public static function hslToRgb(float $h, float $s, float $l): array
    {
        $h /= 360;
        
        if ($s === 0) {
            $r = $g = $b = $l;
        } else {
            $hue2rgb = function($p, $q, $t) {
                if ($t < 0) $t += 1;
                if ($t > 1) $t -= 1;
                if ($t < 1/6) return $p + ($q - $p) * 6 * $t;
                if ($t < 1/2) return $q;
                if ($t < 2/3) return $p + ($q - $p) * (2/3 - $t) * 6;
                return $p;
            };
            
            $q = $l < 0.5 ? $l * (1 + $s) : $l + $s - $l * $s;
            $p = 2 * $l - $q;
            
            $r = $hue2rgb($p, $q, $h + 1/3);
            $g = $hue2rgb($p, $q, $h);
            $b = $hue2rgb($p, $q, $h - 1/3);
        }
        
        return [
            'r' => $r,
            'g' => $g,
            'b' => $b
        ];
    }

    /**
     * Convert RGB to hex
     */
    public static function rgbToHex(int $r, int $g, int $b): string
    {
        return sprintf("#%02x%02x%02x", $r, $g, $b);
    }

    
    /**
     * Generate a random color
     */
    public static function randomColor(): string
    {
        return sprintf("#%02x%02x%02x", rand(0, 255), rand(0, 255), rand(0, 255));
    }

    /**
     * Darken a color by a percentage
     */
    public static function darken(string $hex, float $percent): string
    {
        $rgb = self::hexToRgb($hex);
        $hsl = self::rgbToHsl($rgb['r'], $rgb['g'], $rgb['b']);
        
        $hsl['l'] = max(0, $hsl['l'] - $percent);
        
        $newRgb = self::hslToRgb($hsl['h'], $hsl['s'], $hsl['l']);
        
        return self::rgbToHex(
            round($newRgb['r'] * 255),
            round($newRgb['g'] * 255),
            round($newRgb['b'] * 255)
        );
    }

    /**
     * Lighten a color by a percentage
     */
    public static function lighten(string $hex, float $percent): string
    {
        $rgb = self::hexToRgb($hex);
        $hsl = self::rgbToHsl($rgb['r'], $rgb['g'], $rgb['b']);
        
        $hsl['l'] = min(1, $hsl['l'] + $percent);
        
        $newRgb = self::hslToRgb($hsl['h'], $hsl['s'], $hsl['l']);
        
        return self::rgbToHex(
            round($newRgb['r'] * 255),
            round($newRgb['g'] * 255),
            round($newRgb['b'] * 255)
        );
    }
}
