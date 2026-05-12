<?php

declare(strict_types=1);

namespace Tests\Controllers;

use App\Controllers\GameController;
use App\Http\Body;
use App\Http\Request;
use App\Http\Response;
use PHPUnit\Framework\TestCase;
use Tests\Support\RecordingGameStateService;

final class GameControllerSecurityTest extends TestCase
{
    public function testStartGameIgnoresCallerSuppliedCredits(): void
    {
        $service = new RecordingGameStateService();
        $controller = new GameController($service);
        $request = $this->request(['startingCredits' => 999999])
            ->withAttribute('auth_user', ['id' => 'user_123']);

        $response = $controller->startGame($request, new Response());
        $payload = json_decode((string) $response->getBody(), true);

        self::assertSame(200, $response->getStatusCode());
        self::assertSame('user_123', $service->sessionId);
        self::assertSame(10000, $service->startingCredits);
        self::assertSame(10000, $payload['data']['starting_credits']);
    }

    private function request(array $body): Request
    {
        return new Request(
            ['content-type' => 'application/json'],
            [],
            $body,
            [],
            [],
            'POST',
            '/api/game/start',
            new Body(json_encode($body))
        );
    }
}
