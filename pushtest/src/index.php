<?php

declare(strict_types=1);

namespace Samir\Pushtest;

use Jaeger\Config;
use OpenTracing\GlobalTracer;

use const Jaeger\SAMPLER_TYPE_CONST;

require __DIR__ . '/../vendor/autoload.php';

class App
{
    public function run(): void
    {
        $config = new Config(
            [
                'sampler' => [
                    'type' => SAMPLER_TYPE_CONST,
                    'param' => true,
                ],
                'logging' => true,
                'local_agent' => [
                    'reporting_host' => '127.0.0.1',
                    'reporting_port' => '6831',
                ],
            ],
            // $this->getRandomString(10),
            'pushtest',
            new TextLogger()
        );
        $config->initializeTracer();
        
        $tracer = GlobalTracer::get();
        
        $scope = $tracer->startActiveSpan($this->getRandomString(10), []);
        {
            $span = $tracer->startActiveSpan('sub', []);
            $span->close();
        }
        {
            $span = $tracer->startActiveSpan('sub', []);
            {
                $span2 = $tracer->startActiveSpan('subsub', []);
                sleep(3);
                $span2->close();
            }
            $span->close();
            $span = $tracer->startActiveSpan('sub', []);
            $span->close();
        }
        {
            $span = $tracer->startActiveSpan('longtext', []);
            $span->getSpan()->setTag('app.longtext', str_repeat('a', 10000));
            $span->close();
        }
        $scope->close();

        $tracer->flush();
    }

    private function getRandomString(int $n)
    {
        $characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
        $randomString = '';

        for ($i = 0; $i < $n; $i++) {
            $index = rand(0, strlen($characters) - 1);
            $randomString .= $characters[$index];
        }

        return $randomString;
    }
}

$app = new App();
$app->run();