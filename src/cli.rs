use crate::{
    jaeger::{self, Jaeger, Trace},
    Args,
};

pub struct CliHandler {
    jaeger: Jaeger,
}

enum CliOperation {
    ListOperations,
    ListServices,
    GetTraces,
}

enum CliFormat {
    Json,
    Plain,
}

impl CliFormat {
    fn from_string(format: &str) -> CliFormat {
        match format {
            "json" => CliFormat::Json,
            "plain" => CliFormat::Plain,
            _ => panic!(
                "Unsupported format: {}. Available formats are `json` and `plain`.",
                format
            ),
        }
    }
}

impl CliHandler {
    pub fn new(jaeger: Jaeger) -> CliHandler {
        CliHandler { jaeger }
    }

    pub fn handle(self, args: Args) {
        let operation = if args.list_operations {
            CliOperation::ListOperations
        } else if args.list_services {
            CliOperation::ListServices
        } else if args.service.is_some() {
            CliOperation::GetTraces
        } else {
            CliOperation::ListServices
        };

        let format = CliFormat::from_string(&args.format);
        let formatter: Box<dyn CliFormatter> = match format {
            CliFormat::Json => Box::new(JsonFormatter),
            CliFormat::Plain => Box::new(PlainFormatter),
        };
        let mut data: Vec<String> = Vec::new();
        let mut error = None;

        match operation {
            CliOperation::ListOperations => {
                if let Some(service) = args.service {
                    let operations = self.jaeger.get_operations(&service).unwrap();
                    data = operations.data.into_iter().map(|op| op).collect();
                } else {
                    error = Some("Service name is required to list operations. Supply it with --service flag.");
                }
            }
            CliOperation::ListServices => {
                let services = self.jaeger.get_services().unwrap();
                data = services.data.into_iter().map(|service| service).collect();
            }
            CliOperation::GetTraces => {
                if let Some(service) = args.service {
                    let mut request = jaeger::TracesRequest::new(service);

                    if let Some(operation) = args.operation {
                        request = request.operation(operation);
                    }

                    if let Some(lookback) = args.lookback {
                        if let Some(lookback_unit) = args.lookback_unit {
                            let lookback = jaeger::Lookback {
                                value: lookback,
                                unit: lookback_unit,
                            };
                            request = request.lookback(lookback);
                        }
                    }

                    if let Some(limit) = args.limit {
                        request = request.limit(limit);
                    }

                    if let Some(min_duration) = args.min_duration {
                        request = request.min_duration(min_duration);
                    }

                    if let Some(max_duration) = args.max_duration {
                        request = request.max_duration(max_duration);

                        // we need to add ANY min duration (not 0), if we define max
                        if args.min_duration.is_none() {
                            request = request.min_duration(1);
                        }
                    }

                    let traces = self.jaeger.get_traces(&request).unwrap();
                    
                    CliHandler::display_traces(Ok(traces.data), &formatter);

                    return;
                } else {
                    error = Some(
                        "Service name is required to list traces. Supply it with --service flag.",
                    );
                }
            }
        }

        if let Some(error) = error {
            CliHandler::display(Err(error.to_string()), &formatter);
        } else {
            CliHandler::display(Ok(data), &formatter);
        }
    }

    fn display(data: Result<Vec<String>, String>, formatter: &Box<dyn CliFormatter>) {
        let output = formatter.format(data);
        println!("{}", output);
    }

    fn display_traces(data: Result<Vec<Trace>, String>, formatter: &Box<dyn CliFormatter>) {
        let output = formatter.format_traces(data);
        println!("{}", output);
    }
}

trait CliFormatter {
    fn format(&self, data: Result<Vec<String>, String>) -> String;
    fn format_traces(&self, data: Result<Vec<Trace>, String>) -> String;
}

pub struct JsonFormatter;
impl CliFormatter for JsonFormatter {
    fn format(&self, data: Result<Vec<String>, String>) -> String {
        if let Ok(data) = data {
            return serde_json::to_string(&data).unwrap();
        } else {
            return format!("{{\"error\": \"{}\"}}", data.unwrap_err());
        }
    }

    fn format_traces(&self, data: Result<Vec<Trace>, String>) -> String {
        if let Ok(data) = data {
            return serde_json::to_string(&data).unwrap();
        } else {
            return format!("{{\"error\": \"{}\"}}", data.unwrap_err());
        }
    }
}

pub struct PlainFormatter;

impl PlainFormatter {
    pub fn trace_format(&self, trace: Trace) -> String {
        let mut line = String::new();
        line.push_str(&format!("Trace ID: {}\n", trace.to_string()));
        trace.spans.iter().for_each(|s| {
            line.push_str(&format!("\tSpan ID: {}\n", s.span_id));
            line.push_str(&format!("\tOperation: {}\n", s.operation_name));
            line.push_str(&format!("\tStart Time: {}\n", s.start_time));
            line.push_str(&format!("\tDuration: {}\n", s.duration));
            line.push_str(&format!("\tTags:\n{}\n", s.tags.iter().map(|t| format!("\t\t{}\n", t.value.to_string())).collect::<Vec<String>>().join("")));
        });

        line
    }
}
impl CliFormatter for PlainFormatter {
    fn format(&self, data: Result<Vec<String>, String>) -> String {
        if let Ok(data) = data {
            return data.join("\n");
        } else {
            return data.unwrap_err();
        }
    }

    fn format_traces(&self, data: Result<Vec<Trace>, String>) -> String {
        if let Ok(data) = data {
            return data
                .into_iter()
                .map(|trace| format!("{}", self.trace_format(trace)))
                .collect::<Vec<String>>()
                .join("\n");
        } else {
            return data.unwrap_err();
        }
    }
}
