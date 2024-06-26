<?php

namespace Samir\JaegerMock;

class Traces
{
    public const JAEGER_TRACES = [
        "data" => [
            [
                "traceID" => "57a506b688ac22d8",
                "spans" => [
                    [
                        "traceID" => "57a506b688ac22d8",
                        "spanID" => "57a506b688ac22d8",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431236723967,
                        "duration" => 251,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "61f55ef4286dea9f",
                "spans" => [
                    [
                        "traceID" => "61f55ef4286dea9f",
                        "spanID" => "61f55ef4286dea9f",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431238866059,
                        "duration" => 113,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "7dfe067153d103cd",
                "spans" => [
                    [
                        "traceID" => "7dfe067153d103cd",
                        "spanID" => "7dfe067153d103cd",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431604694787,
                        "duration" => 29,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "65bff849476a5c1b",
                "spans" => [
                    [
                        "traceID" => "65bff849476a5c1b",
                        "spanID" => "65bff849476a5c1b",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431734469332,
                        "duration" => 42,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "40261b58e839854b",
                "spans" => [
                    [
                        "traceID" => "40261b58e839854b",
                        "spanID" => "40261b58e839854b",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431789067590,
                        "duration" => 31,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "190da9f557bebe6f",
                "spans" => [
                    [
                        "traceID" => "190da9f557bebe6f",
                        "spanID" => "190da9f557bebe6f",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719432316443168,
                        "duration" => 333,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "6943624ea73f8b37",
                "spans" => [
                    [
                        "traceID" => "6943624ea73f8b37",
                        "spanID" => "6943624ea73f8b37",
                        "flags" => 1,
                        "operationName" => "/api/traces",
                        "references" => [],
                        "startTime" => 1719431239435406,
                        "duration" => 321,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/traces?end=1719431239419000&limit=20&lookback=1h&maxDuration&minDuration&service=pushtest&start=1719427639419000"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "7d327631710ee733",
                "spans" => [
                    [
                        "traceID" => "7d327631710ee733",
                        "spanID" => "7d327631710ee733",
                        "flags" => 1,
                        "operationName" => "/api/traces",
                        "references" => [],
                        "startTime" => 1719431242988335,
                        "duration" => 226,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/traces?end=1719431239419000&limit=20&lookback=1h&maxDuration&minDuration&service=pushtest&start=1719427639419000"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "5f7c7a04e0ff03ca",
                "spans" => [
                    [
                        "traceID" => "5f7c7a04e0ff03ca",
                        "spanID" => "5f7c7a04e0ff03ca",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431242988565,
                        "duration" => 150,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "358d40d5932ab74e",
                "spans" => [
                    [
                        "traceID" => "358d40d5932ab74e",
                        "spanID" => "358d40d5932ab74e",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431318910134,
                        "duration" => 37,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "7b59ce66ffacff83",
                "spans" => [
                    [
                        "traceID" => "7b59ce66ffacff83",
                        "spanID" => "7b59ce66ffacff83",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431606178612,
                        "duration" => 33,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "206928da97266e5f",
                "spans" => [
                    [
                        "traceID" => "206928da97266e5f",
                        "spanID" => "206928da97266e5f",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719432162877263,
                        "duration" => 254,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "03d8fbb09b4b7047",
                "spans" => [
                    [
                        "traceID" => "03d8fbb09b4b7047",
                        "spanID" => "03d8fbb09b4b7047",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719432316443227,
                        "duration" => 621,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "20b6a5c8c90dd0c9",
                "spans" => [
                    [
                        "traceID" => "20b6a5c8c90dd0c9",
                        "spanID" => "20b6a5c8c90dd0c9",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431241700668,
                        "duration" => 138,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "75924e28fff7d221",
                "spans" => [
                    [
                        "traceID" => "75924e28fff7d221",
                        "spanID" => "75924e28fff7d221",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719431242988719,
                        "duration" => 159,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/pushtest/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "41bb0d9b1082ef94",
                "spans" => [
                    [
                        "traceID" => "41bb0d9b1082ef94",
                        "spanID" => "41bb0d9b1082ef94",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431317438474,
                        "duration" => 19,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "44e25b7f5d456b50",
                "spans" => [
                    [
                        "traceID" => "44e25b7f5d456b50",
                        "spanID" => "44e25b7f5d456b50",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431236723941,
                        "duration" => 188,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "0c11529da2f3a829",
                "spans" => [
                    [
                        "traceID" => "0c11529da2f3a829",
                        "spanID" => "0c11529da2f3a829",
                        "flags" => 1,
                        "operationName" => "/api/traces",
                        "references" => [],
                        "startTime" => 1719431319419878,
                        "duration" => 121,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/traces?service=pushtest&limit=10"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "4c6c3a95bfa27d20",
                "spans" => [
                    [
                        "traceID" => "4c6c3a95bfa27d20",
                        "spanID" => "4c6c3a95bfa27d20",
                        "flags" => 1,
                        "operationName" => "/api/traces",
                        "references" => [],
                        "startTime" => 1719431606674498,
                        "duration" => 134,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/traces?service=pushtest&limit=10"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "6e82dbab5cc6a97b",
                "spans" => [
                    [
                        "traceID" => "6e82dbab5cc6a97b",
                        "spanID" => "6e82dbab5cc6a97b",
                        "flags" => 1,
                        "operationName" => "/api/services",
                        "references" => [],
                        "startTime" => 1719431733031432,
                        "duration" => 25,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "2fc70cb103d63afc",
                "spans" => [
                    [
                        "traceID" => "2fc70cb103d63afc",
                        "spanID" => "2fc70cb103d63afc",
                        "flags" => 1,
                        "operationName" => "/api/traces",
                        "references" => [],
                        "startTime" => 1719431735006032,
                        "duration" => 118,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/traces?service=pushtest&limit=10"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ],
            [
                "traceID" => "31900cad0dc58d64",
                "spans" => [
                    [
                        "traceID" => "31900cad0dc58d64",
                        "spanID" => "31900cad0dc58d64",
                        "flags" => 1,
                        "operationName" => "/api/services/{service}/operations",
                        "references" => [],
                        "startTime" => 1719432320535696,
                        "duration" => 125,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => true
                            ],
                            [
                                "key" => "span.kind",
                                "type" => "string",
                                "value" => "server"
                            ],
                            [
                                "key" => "http.method",
                                "type" => "string",
                                "value" => "GET"
                            ],
                            [
                                "key" => "http.url",
                                "type" => "string",
                                "value" => "/api/services/jaeger-query/operations"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "net/http"
                            ],
                            [
                                "key" => "http.status_code",
                                "type" => "int64",
                                "value" => 200
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "jaeger-query",
                        "tags" => [
                            [
                                "key" => "client-uuid",
                                "type" => "string",
                                "value" => "3f96592973072204"
                            ],
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "df59e8898d14"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.2"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "Go-2.29.1"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ]
        ],
        "total" => 0,
        "limit" => 0,
        "offset" => 0,
        "errors" => null
    ];

    public const PUSHTEST_TRACES = [
        "data" => [
            [
                "traceID" => "061a6c492b34a093",
                "spans" => [
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "50b3db022a4b33d3",
                        "operationName" => "subsub",
                        "references" => [
                            [
                                "refType" => "CHILD_OF",
                                "traceID" => "061a6c492b34a093",
                                "spanID" => "6ac7a1417f04b7c5"
                            ]
                        ],
                        "startTime" => 1719431228721544,
                        "duration" => 3000096,
                        "tags" => [
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ],
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "74760ba0548b4969",
                        "operationName" => "sub",
                        "references" => [
                            [
                                "refType" => "CHILD_OF",
                                "traceID" => "061a6c492b34a093",
                                "spanID" => "061a6c492b34a093"
                            ]
                        ],
                        "startTime" => 1719431228721524,
                        "duration" => 6,
                        "tags" => [
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ],
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "6ac7a1417f04b7c5",
                        "operationName" => "sub",
                        "references" => [
                            [
                                "refType" => "CHILD_OF",
                                "traceID" => "061a6c492b34a093",
                                "spanID" => "061a6c492b34a093"
                            ]
                        ],
                        "startTime" => 1719431228721539,
                        "duration" => 3000114,
                        "tags" => [
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ],
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "4aeee3e3eb30e39a",
                        "operationName" => "sub",
                        "references" => [
                            [
                                "refType" => "CHILD_OF",
                                "traceID" => "061a6c492b34a093",
                                "spanID" => "061a6c492b34a093"
                            ]
                        ],
                        "startTime" => 1719431231721667,
                        "duration" => 9,
                        "tags" => [
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ],
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "061a6c492b34a093",
                        "operationName" => "ogMVirVF0W",
                        "references" => [],
                        "startTime" => 1719431228721101,
                        "duration" => 3000587,
                        "tags" => [
                            [
                                "key" => "sampler.type",
                                "type" => "string",
                                "value" => "const"
                            ],
                            [
                                "key" => "sampler.param",
                                "type" => "bool",
                                "value" => false
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ],
                    [
                        "traceID" => "061a6c492b34a093",
                        "spanID" => "045eefdae5a0a8ee",
                        "operationName" => "longtext",
                        "references" => [
                            [
                                "refType" => "CHILD_OF",
                                "traceID" => "061a6c492b34a093",
                                "spanID" => "061a6c492b34a093"
                            ]
                        ],
                        "startTime" => 1719431231721680,
                        "duration" => 6,
                        "tags" => [
                            [
                                "key" => "app.longtext",
                                "type" => "string",
                                "value" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                            ],
                            [
                                "key" => "component",
                                "type" => "string",
                                "value" => "pushtest"
                            ],
                            [
                                "key" => "internal.span.format",
                                "type" => "string",
                                "value" => "proto"
                            ]
                        ],
                        "logs" => [],
                        "processID" => "p1",
                        "warnings" => null
                    ]
                ],
                "processes" => [
                    "p1" => [
                        "serviceName" => "pushtest",
                        "tags" => [
                            [
                                "key" => "hostname",
                                "type" => "string",
                                "value" => "fc348dd346ca"
                            ],
                            [
                                "key" => "ip",
                                "type" => "string",
                                "value" => "192.168.96.3"
                            ],
                            [
                                "key" => "jaeger.version",
                                "type" => "string",
                                "value" => "PHP-8.3.8"
                            ]
                        ]
                    ]
                ],
                "warnings" => null
            ]
        ],
        "total" => 0,
        "limit" => 0,
        "offset" => 0,
        "errors" => null
    ];
}
