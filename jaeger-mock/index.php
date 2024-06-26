<?php

declare(strict_types=1);

use Samir\JaegerMock\Kernel;

require __DIR__ . '/vendor/autoload.php';

$kernel = new Kernel();
$kernel->run();