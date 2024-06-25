use crate::{
    jaeger::{self, Jaeger, Trace},
    Args,
};

pub struct CliHandler<T>
where
    T: Jaeger,
{
    jaeger: T,
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

impl<T> CliHandler<T>
where
    T: Jaeger,
{
    pub fn new(jaeger: T) -> CliHandler<T> {
        CliHandler { jaeger }
    }

    pub fn display(self, data: Result<Vec<String>, String>, formatter: &Box<dyn CliFormatter>) {
        let output = formatter.format(data);
        println!("{}", output);
    }

    pub fn display_traces(
        self,
        data: Result<Vec<Trace>, String>,
        formatter: &Box<dyn CliFormatter>,
    ) {
        let output = formatter.format_traces(data);
        println!("{}", output);
    }
}

pub trait CliHandlerrer<T>
where
    T: Jaeger,
{
    fn handle(self, args: Args);
}

impl<T> CliHandlerrer<T> for CliHandler<T>
where
    T: Jaeger,
{
    fn handle(self, args: Args) {
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

                    self.display_traces(Ok(traces.data), &formatter);

                    return;
                } else {
                    error = Some(
                        "Service name is required to list traces. Supply it with --service flag.",
                    );
                }
            }
        }

        if let Some(error) = error {
            self.display(Err(error.to_string()), &formatter);
        } else {
            self.display(Ok(data), &formatter);
        }
    }
}

pub trait CliFormatter {
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
            line.push_str(&format!(
                "\tTags:\n{}\n",
                s.tags
                    .iter()
                    .map(|t| format!("\t\t{}\n", t.value.to_string()))
                    .collect::<Vec<String>>()
                    .join("")
            ));
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

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::jaeger::{Jaeger, Operations, Services, Traces, TracesRequest, Trace}; use jaeger::LookbackUnit;

    pub struct MockJaeger;
    impl Jaeger for MockJaeger {
        fn get_services(&self) -> Result<Services, reqwest::Error> {
            Ok(Services {
                data: vec!["service1".to_string(), "service2".to_string()],
                total: 2,
                limit: 0,
                offset: 0,
            })
        }

        fn get_operations(&self, _service: &str) -> Result<Operations, reqwest::Error> {
            Ok(Operations {
                data: vec!["operation1".to_string(), "operation2".to_string()],
                total: 2,
                limit: 0,
                offset: 0,
            })
        }

        fn get_traces(&self, _request: &TracesRequest) -> Result<Traces, reqwest::Error> {
            Ok(Traces {
                data: vec![Trace::default(), Trace::default()],
                total: 2,
            })
        }

        fn get_trace(&self, _trace_id: &str) -> Result<Trace, reqwest::Error> {
            Ok(Trace::default())
        }
    }

    #[test]
    fn test_cli_handler_handle_list_operations() {
        let jaeger = MockJaeger;
        let handler = CliHandler::new(jaeger);
        let args = Args {
            list_operations: true,
            list_services: false,
            service: Some("service1".to_string()),
            operation: None,
            format: "json".to_string(),
            lookback: None,
            lookback_unit: None,
            limit: None,
            min_duration: None,
            max_duration: None,
            url: "http://localhost:16686".to_string(),
        };
        handler.handle(args);
    }

    #[test]
    fn test_cli_handler_handle_list_services() {
        let jaeger = MockJaeger;
        let handler = CliHandler::new(jaeger);
        let args = Args {
            list_operations: false,
            list_services: true,
            service: None,
            operation: None,
            format: "json".to_string(),
            lookback: None,
            lookback_unit: None,
            limit: None,
            min_duration: None,
            max_duration: None,
            url: "http://localhost:16686".to_string(),
        };
        handler.handle(args);
    }

    #[test]
    fn test_cli_handler_handle_get_traces() {
        let jaeger = MockJaeger;
        let handler = CliHandler::new(jaeger);
        let args = Args {
            list_operations: false,
            list_services: false,
            service: Some("service1".to_string()),
            operation: Some("operation1".to_string()),
            format: "json".to_string(),
            lookback: Some(1),
            lookback_unit: Some(LookbackUnit::Minutes),
            limit: Some(10),
            min_duration: Some(1),
            max_duration: Some(100),
            url: "http://localhost:16686".to_string(),
        };
        handler.handle(args);
    }

    #[test]
    fn test_cli_formatter_json_format() {
        let formatter = JsonFormatter;
        let data = Ok(vec!["service1".to_string(), "service2".to_string()]);
        let output = formatter.format(data);
        assert_eq!(output, "[\"service1\",\"service2\"]");
    }

    #[test]
    fn test_cli_formatter_plain_format() {
        let formatter = PlainFormatter;
        let data = Ok(vec!["service1".to_string(), "service2".to_string()]);
        let output = formatter.format(data);
        assert_eq!(output, "service1\nservice2");
    }
}