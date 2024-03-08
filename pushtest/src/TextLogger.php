<?php

declare(strict_types=1);

namespace Samir\Pushtest;

use Psr\Log\LoggerInterface;
use Stringable;

class TextLogger implements LoggerInterface
{
    public function emergency(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function alert(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function critical(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function error(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function warning(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function notice(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function info(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function debug(string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    public function log($level, string|Stringable $message, array $context = []): void
    {
        $this->toText($message, $context);
    }

    private function toText(string|Stringable $message, array $context = []): string
    {
        return (string) $message . ' ' . json_encode($context);
    }
}
