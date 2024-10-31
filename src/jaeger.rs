use std::{error::Error, fmt::Display};

use anyhow::{Error, Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub mod query {
    tonic::include_proto!("jaeger.api_v2");
}

// type Errors = Option<Vec<String>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Services {
    pub data: Vec<String>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Operations {
    pub data: Vec<String>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Traces {
    pub data: Vec<Trace>,
    pub total: i32,
    // pub limit: i32,
    // pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Trace {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    pub spans: Vec<Span>,
    pub processes: Map<String, Value>,
    // pub warnings: Errors,
}

impl Display for Trace {
    /// Returns a string representation of the trace.
    ///
    /// `{trace_id}|{duration}ms|{first operation_name}|{span_count} spans`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.trace_id.to_string();
        let mut start = None;
        let mut end = None;
        // look for start and end
        for span in &self.spans {
            start = match start {
                Some(start) => Some(std::cmp::min(start, span.start_time)),
                None => Some(span.start_time),
            };
            end = match end {
                Some(end) => Some(std::cmp::max(end, span.start_time + span.duration)),
                None => Some(span.start_time + span.duration),
            };
        }
        // calc elapsed time
        if start.is_some() && end.is_some() {
            let elapsed = (end.unwrap() - start.unwrap()) / 1000;
            s = format!("{}|{}ms", s, elapsed);
        }
        // look for first operation name

        let mut first_span: Option<&Span> = None;

        for span in &self.spans {
            if first_span.is_none() || span.start_time < first_span.unwrap().start_time {
                first_span = Some(span);
            }
        }

        if let Some(span) = first_span {
            s = format!("{}|{}", s, span.operation_name);
        }

        // add span count
        s = format!("{}|{} spans", s, self.spans.len());

        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Span {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    #[serde(rename = "spanID")]
    pub span_id: String,
    pub flags: Option<i32>,
    #[serde(rename = "operationName")]
    pub operation_name: String,
    pub references: Option<Vec<Reference>>,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub tags: Vec<Tag>,
    // pub logs: Vec<Log>,
    #[serde(rename = "processID")]
    pub process_id: String,
    // pub warnings: Errors,
}

impl Display for Span {
    /// Returns a string representation of the span.
    ///
    /// `{operation_name}|{duration}ms|{span_id}`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "{}|{}ms|{}",
            self.operation_name, self.duration, self.span_id
        );
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    #[serde(rename = "refType")]
    pub ref_type: RefType,
    #[serde(rename = "traceID")]
    pub trace_id: String,
    #[serde(rename = "spanID")]
    pub span_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RefType {
    #[serde(rename = "CHILD_OF")]
    ChildOf,
    #[serde(rename = "FOLLOWS_FROM")]
    FollowsFrom,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    pub key: String,
    #[serde(rename = "type")]
    pub tag_type: String, // todo: enum
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TagValue {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f64),
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Process {
//     pub serviceName: String,
//     pub tags: Vec<Tag>,
//     pub warnings: Errors,
// }

#[derive(Debug)]
pub struct TracesRequest {
    pub service: String,
    pub operation: Option<String>,
    pub limit: Option<i32>,
    pub start: Option<i64>,
    pub end: Option<i64>, // todo: duration?
    pub min_duration: Option<u64>,
    pub max_duration: Option<u64>,
    pub lookback: Option<Lookback>,
}

impl TracesRequest {
    pub fn new(service: String) -> TracesRequest {
        TracesRequest {
            service,
            operation: None,
            limit: None,
            start: None,
            end: None,
            min_duration: None,
            max_duration: None,
            lookback: None,
        }
    }

    pub fn limit(mut self, limit: i32) -> TracesRequest {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start: i64) -> TracesRequest {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: i64) -> TracesRequest {
        self.end = Some(end);
        self
    }

    pub fn min_duration(mut self, min_duration: u64) -> TracesRequest {
        self.min_duration = Some(min_duration);
        self
    }

    pub fn max_duration(mut self, max_duration: u64) -> TracesRequest {
        self.max_duration = Some(max_duration);
        self
    }

    pub fn lookback(mut self, lookback: Lookback) -> TracesRequest {
        self.lookback = Some(lookback);
        self
    }

    pub fn operation(mut self, operation: String) -> TracesRequest {
        self.operation = Some(operation);
        self
    }
}

#[derive(Debug)]
pub struct Lookback {
    pub value: i32,
    pub unit: LookbackUnit,
}

#[derive(Debug, Clone)]
pub enum LookbackUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

// brings to_string method
impl Display for LookbackUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookbackUnit::Seconds => write!(f, "s"),
            LookbackUnit::Minutes => write!(f, "m"),
            LookbackUnit::Hours => write!(f, "h"),
            LookbackUnit::Days => write!(f, "d"),
        }
    }
}

pub struct JaegerService {
    pub host: String,
}

impl JaegerService {
    pub fn new(host: &str) -> JaegerService {
        JaegerService {
            host: host.to_string(),
        }
    }
}

pub trait Jaeger {
    async fn get_operations(&mut self, service: &str) -> Result<Operations>;
    async fn get_services(&mut self) -> Result<Services>;
    async fn get_traces(&mut self, request: &TracesRequest) -> Result<Traces>;
    async fn get_trace(&mut self, trace_id: &str) -> Result<Trace>;
}

impl Jaeger for JaegerService {
    async fn get_services(&mut self) -> Result<Services> {
        let url = format!("{}/api/services", self.host);
        let response = reqwest::blocking::get(url)?.json::<Services>()?;

        Ok(response)
    }

    async fn get_operations(&mut self, service: &str) -> Result<Operations> {
        let url = format!("{}/api/services/{}/operations", self.host, service);
        let mut res = reqwest::blocking::get(url)?.json::<Operations>()?;
        // add asterisk operation, to match them all
        res.data.insert(0, "*".to_string());

        Ok(res)
    }

    async fn get_traces(&mut self, request: &TracesRequest) -> Result<Traces> {
        let mut url = format!("{}/api/traces?service={}", self.host, request.service);

        if let Some(operation) = request.operation.as_ref() {
            url = format!("{}&operation={}", url, operation);
        }

        if let Some(limit) = request.limit {
            url = format!("{}&limit={}", url, limit);
        }

        if let Some(start) = request.start {
            url = format!("{}&start={}", url, start);
        }

        if let Some(end) = request.end {
            url = format!("{}&end={}", url, end);
        }

        if let Some(min_duration) = request.min_duration {
            url = format!("{}&minDuration={}ms", url, min_duration);
        }

        if let Some(max_duration) = request.max_duration {
            url = format!("{}&maxDuration={}ms", url, max_duration);
        }

        if let Some(lookback) = &request.lookback {
            url = format!("{}&lookback={}{}", url, lookback.value, lookback.unit);
        }

        let response = reqwest::blocking::get(url)?.json::<Traces>()?;

        Ok(response)
    }

    async fn get_trace(&mut self, trace_id: &str) -> Result<Trace> {
        let url = format!("{}/api/traces/{}", self.host, trace_id);
        let response = reqwest::blocking::get(url)?.json::<Trace>()?;

        Ok(response)
    }
}

pub struct ProtoService {
    pub client: query::query_service_client::QueryServiceClient<tonic::transport::Channel>,
}


impl Jaeger for ProtoService {
    async fn get_services(&mut self) -> Result<Services> {
        let request = query::GetServicesRequest {};
        let response = self.client.get_services(request).await?;
        let services = response.into_inner();
        let data = services.services.into_iter().collect();
        let total = 0;
        let limit = 0;
        let offset = 0;

        Ok(Services {
            data,
            total,
            limit,
            offset,
        })
    }

    async fn get_operations(&mut self, service: &str) -> Result<Operations> {
        let request = query::GetOperationsRequest {
            service: service.to_string(),
            span_kind: "".to_string(),
        };
        let response = self.client.get_operations(request).await?;
        let operations = response.into_inner();
        let data = operations.operations.into_iter().map(|op| op.name).collect();
        let total = 0;
        let limit = 0;
        let offset = 0;

        Ok(Operations {
            data,
            total,
            limit,
            offset,
        })
    }

    async fn get_traces(&mut self, request: &TracesRequest) -> Result<Traces> {
        let proto_request = query::FindTracesRequest {
            query: None,
        };

        let response = self.client.find_traces(request).await?;
        let mut traces = response.into_inner();
        let mut data = vec![];

        while let Some(trace) = traces.message().await? {
            let trace_id = trace.trace_id;
            let spans = trace.spans.into_iter().map(|span| {
                let trace_id = span.trace_id;
                let span_id = span.span_id;
                let flags = span.flags;
                let operation_name = span.operation_name;
                let references = span.references.into_iter().map(|ref_| {
                    let ref_type = match ref_.ref_type {
                        query::SpanRefType::ChildOf => RefType::ChildOf,
                        query::SpanRefType::FollowsFrom => RefType::FollowsFrom,
                    };
                    let trace_id = ref_.
                }).collect();
                let start_time = span.start_time;
                let duration = span.duration;
                let tags = span.tags.into_iter().map(|tag| {
                    let key = tag.key;
                    let tag_type = tag.type_;
                    let value = tag.value;
                    Tag {
                        key,
                        tag_type,
                        value,
                    }
                }).collect();
                let process_id = span.process_id;
                Span {
                    trace_id,
                    span_id,
                    flags,
                    operation_name,
                    references,
                    start_time,
                    duration,
                    tags,
                    process_id,
                }
            }).collect();
            let processes = trace.processes.into_iter().map(|(key, value)| {
                (key, value)
            }).collect();
            data.push(
                Trace {
                    trace_id,
                    spans,
                    processes,
                }                
            );
        }
        let total = 0;
        
        Ok(Traces {
            data,
            total,
        })
    }

    async fn get_trace(&mut self, trace_id: &str) -> Result<Trace> {}
}
