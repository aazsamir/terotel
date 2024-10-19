
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

use crate::app::{State, Window};

pub fn ui(frame: &mut Frame, state: &State) {
    if let Window::Spans = state.selected_window {
        ui_trace(frame, state);
    } else if let Window::Span = state.selected_window {
        ui_trace(frame, state);
    } else {
        ui_main(frame, state)
    }
}

pub fn ui_main(frame: &mut Frame, state: &State) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ],
    )
    .split(frame.area());
    ui_topbar(frame, &main_layout[0], state);
    ui_statusbar(frame, &main_layout[2], state);

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(60),
        ],
    )
    .split(main_layout[1]);
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Services"), // "default" title, if no services found
        inner_layout[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Operations"),
        inner_layout[1],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Traces"),
        inner_layout[2],
    );

    ui_services(frame, &inner_layout[0], state);
    ui_operations(frame, &inner_layout[1], state);
    ui_traces(frame, &inner_layout[2], state);
}

pub fn ui_topbar(frame: &mut Frame, layout: &Rect, _state: &State) {
    let text = "Terotel - Terminal OTEL Viewer";
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, *layout);
}

pub fn ui_statusbar(frame: &mut Frame, layout: &Rect, state: &State) {
    let mut action_text = if state.is_search_state {
        let mut search_input = "(ESC?)> ".to_string();
        search_input.push_str(&state.search_input);
        search_input
    } else {
        "q - Quit | hjkl - Move | e - Select | / - Search | PgUp - Page+ | PgDown - Page- | [] - Min Duration | {} - Max Duration".to_string()
    };

    if state.is_debug {
        action_text.push_str(" | DEBUG: ");
        action_text.push_str(&state.debug_text);
    }

    let state_text = format!("Page: {} | Duration: {}/{}", state.traces_page + 1, state.min_duration, state.max_duration);

    let lines: Vec<Line> = vec![
        Line::from(action_text),
        Line::from(state_text),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, *layout);
}

pub fn ui_services(frame: &mut Frame, layout: &Rect, state: &State) {
    if state.services.is_none() {
        return;
    }

    let mut block = Block::default().title("Services").borders(Borders::ALL);

    // if selected window is services, add asterisk to title
    if let Window::Services = state.selected_window {
        block = Block::default().title("*Services*").borders(Borders::ALL);
    };

    let services = List::new(
        state
            .services
            .as_ref()
            .unwrap()
            .data
            .iter()
            .map(|s| {
                let service = s.to_string();
                if state.selected_service.is_some()
                    && state.selected_service.as_ref().unwrap() == &service
                {
                    format!("*{}*", service)
                } else {
                    service
                }
            })
            .collect::<Vec<String>>(),
    )
    .block(block)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(" ");

    let mut state = state.services_state.clone();
    frame.render_stateful_widget(services, *layout, &mut state);
}

pub fn ui_operations(frame: &mut Frame, layout: &Rect, state: &State) {
    if state.operations.is_none() {
        return;
    }

    let mut block = Block::default().title("Operations").borders(Borders::ALL);

    if let Window::Operations = state.selected_window {
        block = Block::default().title("*Operations*").borders(Borders::ALL);
    };

    let operations = List::new(
        state
            .operations
            .as_ref()
            .unwrap()
            .data
            .iter()
            .map(|s| {
                let operation = s.to_string();
                if state.selected_operation.is_some()
                    && state.selected_operation.as_ref().unwrap() == &operation
                {
                    format!("*{}*", operation)
                } else {
                    operation
                }
            })
            .collect::<Vec<String>>(),
    )
    .block(block)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(" ");

    let mut state = state.operations_state.clone();
    frame.render_stateful_widget(operations, *layout, &mut state);
}

pub fn ui_traces(frame: &mut Frame, layout: &Rect, state: &State) {
    if state.traces.is_none() {
        return;
    }

    let mut block = Block::default().title("Traces").borders(Borders::ALL);

    if let Window::Traces = state.selected_window {
        block = Block::default().title("*Traces*").borders(Borders::ALL);
    };

    let traces = List::new(
        state
            .traces
            .as_ref()
            .unwrap()
            .data
            .iter()
            .map(|t| {
                let trace = t.to_string();
                if state.selected_trace.is_some()
                    && state.selected_trace.as_ref().unwrap() == &trace
                {
                    format!("*{}*", trace)
                } else {
                    trace.to_string()
                }
            })
            .collect::<Vec<String>>(),
    )
    .block(block)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(" ");

    let mut state = state.traces_state.clone();
    frame.render_stateful_widget(traces, *layout, &mut state);
}

pub fn ui_trace(frame: &mut Frame, state: &State) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ],
    )
    .split(frame.area());
    ui_topbar(frame, &main_layout[0], state);
    ui_statusbar(frame, &main_layout[2], state);

    let content_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ],
    ).split(main_layout[1]);

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(content_layout[0]);

    ui_spans(frame, &inner_layout[0], state);
    ui_span(frame, &inner_layout[1], state);
    ui_diagram(frame, &content_layout[1], state);
}

pub fn ui_spans(frame: &mut Frame, layout: &Rect, state: &State) {
    if state.spans.is_none() {
        return;
    }

    let mut lines: Vec<String> = vec![];
    let spans = state.spans.clone().unwrap();
    let parents = state.spans.clone().unwrap();

    // todo: don't do that here, as it happens every while loop

    let mut indentation = 0;
    let mut indetations: HashMap<String, i32> = HashMap::new();

    for span in spans {
        let mut line_text = format!("{}|{}ms", span.operation_name, span.duration / 1000);
        let span_string = span.to_string();

        // if span have references
        if let Some(reference) = span.references.unwrap().first() {
            let parent_span_id = reference.span_id.clone();

            // otherwise, figure out indendation
            for parent in &parents {
                if parent.span_id == parent_span_id {
                    // if parent was already indented, use that indentation
                    if indetations.contains_key(parent_span_id.as_str()) {
                        indentation = *indetations.get(parent_span_id.as_str()).unwrap();
                    } else {
                        // otherwise, add indent and save it
                        indentation += 1;
                        indetations.insert(parent_span_id, indentation);
                    }

                    let indent_string = "  ".repeat(indentation as usize);
                    line_text = format!("{}{}", indent_string, line_text);

                    break;
                }
            }
        }

        if let Some(selected_span) = &state.selected_span {
            if span_string.eq(selected_span) {
                line_text = format!("*{}*", line_text);
            }
        }

        lines.push(line_text);
    }

    let mut block = Block::default().title("Spans").borders(Borders::ALL);

    if let Window::Spans = state.selected_window {
        block = Block::default().title("*Spans*").borders(Borders::ALL);
    };

    let spans = List::new(lines)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(" ");

    frame.render_stateful_widget(spans, *layout, &mut state.spans_state.clone());
}

pub fn ui_span(frame: &mut Frame, layout: &Rect, state: &State) {
    if state.spans.is_none() {
        return;
    }

    if state.selected_span.is_none() {
        return;
    }

    // find selected span
    let selected_span = state
        .spans
        .as_ref()
        .unwrap()
        .iter()
        .find(|s| s.to_string() == *state.selected_span.as_ref().unwrap())
        .unwrap();

    let mut block = Block::default().title("Span Details").borders(Borders::ALL);

    if let Window::Spans = state.selected_window {
        block = Block::default()
            .title("*Span Details*")
            .borders(Borders::ALL);
    };

    let mut lines: Vec<String> = vec![];
    lines.push(format!("Operation Name: {}", selected_span.operation_name));
    lines.push(format!("Trace ID: {}", selected_span.trace_id));
    lines.push(format!("Span ID: {}", selected_span.span_id));
    lines.push(format!("Start Time: {}", selected_span.start_time));
    lines.push(format!("Duration: {}", selected_span.duration));
    lines.push(format!("Process ID: {}", selected_span.process_id));
    lines.push("Tags:".to_string());

    for tag in &selected_span.tags {
        lines.push(format!("  {} - {}", tag.key, tag.value));
    }

    let mut paragraph_items = vec![];
    for line in lines {
        paragraph_items.push(Line::from(line));
    }

    let paragraph_items_len = paragraph_items.len();

    let paragraph = Paragraph::new(paragraph_items)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll((state.span_text_scroll, 0))
        .block(block);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"));

    let mut scrollbar_state =
        ScrollbarState::new(paragraph_items_len).position(state.span_text_scroll as usize);

    frame.render_widget(paragraph, *layout);
    let margin = Margin {
        horizontal: 0,
        vertical: 1,
    };
    frame.render_stateful_widget(scrollbar, layout.inner(margin), &mut scrollbar_state);
}

pub fn ui_diagram(frame: &mut Frame, layout: &Rect, state: &State) {
    let block = Block::default().title("Diagram").borders(Borders::ALL);

    if state.selected_trace.is_none() {
        frame.render_widget(block, *layout);
        return;
    }

    let selected_trace = state
        .traces
        .as_ref()
        .unwrap()
        .data
        .iter()
        .find(|t| t.to_string() == *state.selected_trace.as_ref().unwrap())
        .unwrap();

    let mut spans = selected_trace.spans.clone();
    let mut min_starttime = i64::MAX;
    let mut max_endtime = 0;
    let spans_count = spans.len();
    let mut counter = spans.len();

    let mut data = vec![];

    spans.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    let mut labels = vec![];

    for span in spans {
        let start_time = span.start_time;
        let end_time = span.start_time + span.duration;

        if start_time < min_starttime {
            min_starttime = start_time;
        }

        if end_time > max_endtime {
            max_endtime = end_time;
        }

        data.push(vec![(start_time as f64, counter as f64), (end_time as f64, counter as f64)]);
        counter -= 1;

        labels.push(format!("{}", span.operation_name));
    }
    labels.push("".to_string());

    labels.reverse();

    let datasets: Vec<Dataset> = data.iter().map(|d| {
        Dataset::default()
            .data(d)
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
    }).collect();

    let chart = Chart::new(datasets)
        .block(block)
        .style(Style::default().fg(Color::White))
        .x_axis(Axis::default().style(Style::default().fg(Color::Green)).bounds([min_starttime as f64, max_endtime as f64]))
        .y_axis(Axis::default().style(Style::default().fg(Color::Red)).bounds([0.0, spans_count as f64]).labels(labels));

    frame.render_widget(chart, *layout);
}
