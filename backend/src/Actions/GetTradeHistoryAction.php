<?php
namespace App\Actions;

class GetTradeHistoryAction
{
    public function execute(string $sessionId, int $page = 1, int $perPage = 10): array
    {
        // TODO: Implement trade history storage and retrieval
        // For now, return empty history
        $tradeHistory = [];
        $total = 0;
        return [
            'data' => $tradeHistory,
            'page' => $page,
            'perPage' => $perPage,
            'total' => $total
        ];
    }
}
