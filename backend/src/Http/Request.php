<?php

namespace App\Http;

class Request
{
    private array $headers;
    private array $queryParams;
    private array $parsedBody;
    private array $cookies;
    private array $attributes = [];
    private array $serverParams;
    private string $method;
    private string $uri;
    private Body $body;

    public function __construct(
        array $headers,
        array $queryParams,
        array $parsedBody,
        array $cookies,
        array $serverParams,
        string $method,
        string $uri,
        Body $body
    ) {
        $this->headers = $headers;
        $this->queryParams = $queryParams;
        $this->parsedBody = $parsedBody;
        $this->cookies = $cookies;
        $this->serverParams = $serverParams;
        $this->method = $method;
        $this->uri = $uri;
        $this->body = $body;
    }

    public function getHeaderLine(string $name): string
    {
        $key = strtolower($name);
        if (isset($this->headers[$key])) {
            return $this->headers[$key];
        }

        if ($key === 'authorization') {
            return $this->serverParams['HTTP_AUTHORIZATION']
                ?? $this->serverParams['REDIRECT_HTTP_AUTHORIZATION']
                ?? '';
        }

        return '';
    }

    public function getQueryParams(): array
    {
        return $this->queryParams;
    }

    public function getParsedBody(): array
    {
        return $this->parsedBody;
    }

    public function getCookieParams(): array
    {
        return $this->cookies;
    }

    public function getServerParams(): array
    {
        return $this->serverParams;
    }

    public function getMethod(): string
    {
        return $this->method;
    }

    public function getUri(): string
    {
        return $this->uri;
    }

    public function getBody(): Body
    {
        return $this->body;
    }

    public function withAttribute(string $name, mixed $value): self
    {
        $clone = clone $this;
        $clone->attributes[$name] = $value;
        return $clone;
    }

    public function getAttribute(string $name, mixed $default = null): mixed
    {
        return $this->attributes[$name] ?? $default;
    }
}
