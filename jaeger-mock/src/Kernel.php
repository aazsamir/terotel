<?php

declare(strict_types=1);

namespace Samir\JaegerMock;

class Kernel
{
    public function run(): void
    {
        $response = $this->routing();

        if (isset($response['error'])) {
            header('HTTP/1.1 400 Not Found');
            echo $response['error'];

            return;
        }

        header('Content-Type: application/json');
        echo \json_encode($response);
    }

    public function routing(): array
    {
        $path = $_SERVER['REQUEST_URI'];

        $services = [
            'jaeger-query',
            'pushtest',
        ];

        $operations = [
            $services[0] => [
                '/api/services',
                '/api/services/{service}/operations',
                '/api/traces',
            ],
            $services[1] => [
                'subsub',
                'sub',
                'longtext',
                'ogMVirVF0W',
            ],
        ];

        $traces = [
            $services[0] => Traces::JAEGER_TRACES,
            $services[1] => Traces::PUSHTEST_TRACES,
        ];

        if ($this->route('/api/services')) {
            return [
                'data' => $services,
                'total' => 2,
                'limit' => 0,
                'offset' => 0,
                'errors' => null,
            ];
        }

        if ($service = $this->route('/api/services/([\w\-]*)/operations')) {
            $service = $service[1];

            if (isset($operations[$service])) {
                return [
                    'data' => $operations[$service],
                    'total' => \count($operations[$service]),
                    'limit' => 0,
                    'offset' => 0,
                    'errors' => null,
                ];
            }

            return ['error' => 'Not found'];
        }

        if ($this->route('/api/traces')) {
            $service = $_GET['service'] ?? null;

            if ($service && isset($traces[$service])) {
                $traces = $traces[$service];
                $data = $traces['data'];
                
                $operation = $_GET['operation'] ?? null;

                if ($operation) {
                    $data = \array_filter($data, function ($trace) use ($operation) {
                        // iterate over spans
                        foreach ($trace['spans'] as $span) {
                            if ($span['operationName'] === $operation) {
                                return true;
                            }
                        }
                    });
                }

                $minDuration = $_GET['minDuration'] ?? null;

                if ($minDuration) {
                    $minDuration = (int) $minDuration;
                    $minDuration = $minDuration * 1000; // convert to microseconds
                    $data = \array_filter($data, function ($trace) use ($minDuration) {
                        // iterate over spans
                        foreach ($trace['spans'] as $span) {
                            if ($span['duration'] >= $minDuration) {
                                return true;
                            }
                        }
                    });
                }

                $maxDuration = $_GET['maxDuration'] ?? null;

                if ($maxDuration) {
                    $maxDuration = (int) $maxDuration;
                    $maxDuration = $maxDuration * 1000; // convert to microseconds
                    $data = \array_filter($data, function ($trace) use ($maxDuration) {
                        // iterate over spans
                        foreach ($trace['spans'] as $span) {
                            if ($span['duration'] <= $maxDuration) {
                                return true;
                            }
                        }
                    });
                }

                $limit = $_GET['limit'] ?? null;

                if ($limit) {
                    $limit = (int) $limit;
                    $data = \array_slice($data, 0, $limit);
                }

                $data = \array_values($data);

                $traces['data'] = $data;

                return $traces;
            }

            return ['error' => 'Not found'];
        }

        return ['error' => 'Not found'];
    }

    private function route(string $regexp): array
    {
        $res = [];
        $regexp = '.^' . $regexp . '$.';
        $uri = $_SERVER['REQUEST_URI'];
        // trim query string
        $uri = \explode('?', $uri)[0];
        // trim trailing slash
        $uri = \rtrim($uri, '/');

        \preg_match($regexp, $uri, $res);

        return $res;
    }
}
