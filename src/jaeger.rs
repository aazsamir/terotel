use core::str;
use std::{collections::HashMap, fmt::Display};

use anyhow::{Error, Ok, Result};
use prost_types::{Duration, Timestamp};
use query::TraceQueryParameters;
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

#[async_trait::async_trait]
pub trait Jaeger {
    async fn get_operations(&mut self, service: &str) -> Result<Operations>;
    async fn get_services(&mut self) -> Result<Services>;
    async fn get_traces(&mut self, request: &TracesRequest) -> Result<Traces>;
    async fn get_trace(&mut self, trace_id: &str) -> Result<Trace>;
}

#[async_trait::async_trait]
impl Jaeger for JaegerService {
    async fn get_services(&mut self) -> Result<Services> {
        let url = format!("{}/api/services", self.host);
        let response = reqwest::get(url).await?.json::<Services>().await?;

        Ok(response)
    }

    async fn get_operations(&mut self, service: &str) -> Result<Operations> {
        let url = format!("{}/api/services/{}/operations", self.host, service);
        let mut res = reqwest::get(url).await?.json::<Operations>().await?;
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

        let response = reqwest::get(url).await?.json::<Traces>().await?;

        Ok(response)
    }

    async fn get_trace(&mut self, trace_id: &str) -> Result<Trace> {
        let url = format!("{}/api/traces/{}", self.host, trace_id);
        let response = reqwest::get(url).await?.json::<Trace>().await?;

        Ok(response)
    }
}

pub struct ProtoService {
    pub client: query::query_service_client::QueryServiceClient<tonic::transport::Channel>,
}

impl ProtoService {
    pub async fn new(url: &str) -> Result<ProtoService> {        
        let client = query::query_service_client::QueryServiceClient::connect(url.to_string()).await?;

        Ok(ProtoService { client })
    }
}

#[async_trait::async_trait]
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
        let data = operations
            .operations
            .into_iter()
            .map(|op| op.name)
            .collect();
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
            query: Some(
                TraceQueryParameters {
                    duration_max: if let Some(max_duration) = request.max_duration{
                        Some(Duration {seconds: max_duration as i64, nanos: 0})
                    } else {
                        None
                    },
                    duration_min: if let Some(min_duration) = request.min_duration{
                        Some(Duration {seconds: min_duration as i64, nanos: 0})
                    } else {
                        None
                    },
                    operation_name: request.operation.clone().unwrap_or_default(),
                    service_name: request.service.clone(),
                    start_time_min: if let Some(start) = request.start{
                        Some(Timestamp {seconds: start, nanos: 0})
                    } else {
                        None
                    },
                    start_time_max: if let Some(end) = request.end{
                        Some(Timestamp {seconds: end, nanos: 0})
                    } else {
                        None
                    },
                    tags: HashMap::new(),
                    search_depth: 20,
                }
            )
        };

        let response = self.client.find_traces(proto_request).await?;
        // let mut data = vec![];
        let mut data = HashMap::new();
        let mut response = response.into_inner();

        while let Some(spans_chunk) = response.message().await? {
            for span in &spans_chunk.spans {
                let trace_id = vec_u8_to_hex_string(span.trace_id.clone());
                // let trace_id = vec_u8_to_hex_string(span.span_id);
                // data.push(Trace {
                //     trace_id: trace_id.to_string(),
                //     spans: vec![],
                //     processes: Map::new(),
                // });
                let mut spans = vec![];
                for span in spans_chunk.spans.iter() {
                    let span_trace_id = vec_u8_to_hex_string(span.trace_id.clone());
                    let span_id = vec_u8_to_hex_string(span.span_id.clone());
                    let operation_name = span.operation_name.clone();
                    let start_time = span.start_time;
                    let duration = span.duration;
                    let process_id = span.process_id.clone();
                    let mut tags = vec![];

                    spans.push(
                        Span {
                            trace_id: span_trace_id.clone(),
                            span_id: span_id.clone(),
                            flags: None,
                            operation_name,
                            references: None,
                            start_time: 0,
                            duration: 0,
                            tags,
                            process_id,
                        }
                    );
                }

                data.insert(trace_id.clone(), Trace {
                    trace_id: trace_id.to_string(),
                    spans,
                    processes: Map::new(),
                });
            }
        }

        let data = data.into_iter().map(|(_, v)| v).collect();

        Ok(Traces { data, total: 0 })
    }

    async fn get_trace(&mut self, _trace_id: &str) -> Result<Trace> {
        todo!()
    }
}

fn vec_u8_to_hex_string(vec: Vec<u8>) -> String {
    let mut s = String::new();
    for byte in vec {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}
