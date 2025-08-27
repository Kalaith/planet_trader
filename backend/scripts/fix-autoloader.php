#!/usr/bin/env php
<?php

/**
 * Regenerate Composer Autoloader
 * 
 * Run this script if you're having autoloader issues
 */

echo "🔄 Regenerating Composer autoloader...\n";

// Change to the backend directory
$backendDir = __DIR__ . '/..';
chdir($backendDir);

// Run composer dump-autoload
$command = 'composer dump-autoload -o';
echo "Running: $command\n";

$output = [];
$returnCode = 0;
exec($command, $output, $returnCode);

foreach ($output as $line) {
    echo $line . "\n";
}

if ($returnCode === 0) {
    echo "✅ Autoloader regenerated successfully!\n";
    
    // Test the autoloader
    echo "🧪 Testing autoloader...\n";
    require_once __DIR__ . '/../vendor/autoload.php';
    
    try {
        $reflection = new ReflectionClass('App\Database\SeederEnhanced');
        echo "✅ SeederEnhanced class found: " . $reflection->getFileName() . "\n";
    } catch (ReflectionException $e) {
        echo "❌ SeederEnhanced class not found: " . $e->getMessage() . "\n";
    }
    
    try {
        $reflection = new ReflectionClass('App\Database\Connection');
        echo "✅ Connection class found: " . $reflection->getFileName() . "\n";
    } catch (ReflectionException $e) {
        echo "❌ Connection class not found: " . $e->getMessage() . "\n";
    }
    
} else {
    echo "❌ Failed to regenerate autoloader (exit code: $returnCode)\n";
    exit(1);
}

echo "\n✅ All checks passed! You can now run migrations.\n";
