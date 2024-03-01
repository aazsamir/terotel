use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// type Errors = Option<Vec<String>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Services {
    pub data: Vec<String>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Operations {
    pub data: Vec<String>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Traces {
    pub data: Vec<Trace>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
    // pub errors: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
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
        if let Some(span) = self.spans.first() {
            s = format!("{}|{}", s, span.operation_name);
        }

        // add span count
        s = format!("{}|{} spans", s, self.spans.len());
        
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Span {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    #[serde(rename = "spanID")]
    pub span_id: String,
    pub flags: i32,
    #[serde(rename = "operationName")]
    pub operation_name: String,
    // pub references: Vec<String>,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub tags: Vec<Tag>,
    // pub logs: Vec<Log>,
    #[serde(rename = "processID")]
    pub process_id: String,
    // pub warnings: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub key: String,
    #[serde(rename = "type")]
    pub tag_type: String, // todo: enum
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TagValue {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f64),
}

// #[derive(Debug, Deserialize, Serialize)]
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
    pub min_duration: Option<i64>,
    pub max_duration: Option<i64>,
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

    pub fn min_duration(mut self, min_duration: i64) -> TracesRequest {
        self.min_duration = Some(min_duration);
        self
    }

    pub fn max_duration(mut self, max_duration: i64) -> TracesRequest {
        self.max_duration = Some(max_duration);
        self
    }

    pub fn lookback(mut self, lookback: Lookback) -> TracesRequest {
        self.lookback = Some(lookback);
        self
    }
}

#[derive(Debug)]
pub struct Lookback {
    pub value: i32,
    pub unit: LookbackUnit,
}

#[derive(Debug)]
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

pub struct Jaeger {
    pub host: String,
}

impl Jaeger {
    pub fn new(host: &str) -> Jaeger {
        Jaeger {
            host: host.to_string(),
        }
    }

    pub fn get_services(&self) -> Result<Services, reqwest::Error> {
        let url = format!("{}/api/services", self.host);
        reqwest::blocking::get(url)?.json::<Services>()
    }

    pub fn get_operations(&self, service: &str) -> Result<Operations, reqwest::Error> {
        let url = format!("{}/api/services/{}/operations", self.host, service);
        let mut res = reqwest::blocking::get(url)?.json::<Operations>();
        // add asterisk operation, to match them all
        if let Ok(res) = &mut res {
            res.data.insert(0, "*".to_string());
        }
        res
    }

    pub fn get_traces(&self, request: &TracesRequest) -> Result<Traces, reqwest::Error> {
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
            url = format!("{}&minDuration={}", url, min_duration);
        }

        if let Some(max_duration) = request.max_duration {
            url = format!("{}&maxDuration={}", url, max_duration);
        }

        if let Some(lookback) = &request.lookback {
            url = format!("{}&lookback={}{}", url, lookback.value, lookback.unit);
        }
        
        reqwest::blocking::get(url)?.json::<Traces>()
    }

    pub fn get_trace(&self, trace_id: &str) -> Result<Trace, reqwest::Error> {
        let url = format!("{}/api/traces/{}", self.host, trace_id);
        reqwest::blocking::get(url)?.json::<Trace>()
    }
}
