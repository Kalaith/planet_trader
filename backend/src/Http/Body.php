<?php

namespace App\Http;

class Body
{
    private string $contents;

    public function __construct(string $contents = '')
    {
        $this->contents = $contents;
    }

    public function write(string $data): void
    {
        $this->contents .= $data;
    }

    public function getContents(): string
    {
        return $this->contents;
    }

    public function __toString(): string
    {
        return $this->contents;
    }
}
