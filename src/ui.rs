
use ratatui::{prelude::*, widgets::*};
use std::{
    collections::HashMap,
    rc::Rc,
};

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
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    ui_topbar(frame, &main_layout, state);
    ui_statusbar(frame, &main_layout, state);

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
        inner_layout[Window::Services as usize],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Operations"),
        inner_layout[Window::Operations as usize],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Traces"),
        inner_layout[Window::Traces as usize],
    );

    ui_services(frame, &inner_layout, state);
    ui_operations(frame, &inner_layout, state);
    ui_traces(frame, &inner_layout, state);
}

pub fn ui_topbar(frame: &mut Frame, layout: &Rc<[Rect]>, _state: &State) {
    let text = "Terotel - Terminal OTEL Viewer";
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, layout[0]);
}

pub fn ui_statusbar(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
    let text = if state.is_search_state {
        let mut search_input = "(ESC?)> ".to_string();
        search_input.push_str(&state.search_input);
        search_input
    } else {
        "q - Quit | hjkl - Move | e - Select | / - Search".to_string()
    };
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, layout[2]);
}

pub fn ui_services(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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
    frame.render_stateful_widget(services, layout[0], &mut state);
}

pub fn ui_operations(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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
    frame.render_stateful_widget(operations, layout[1], &mut state);
}

pub fn ui_traces(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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
    frame.render_stateful_widget(traces, layout[2], &mut state);
}

pub fn ui_trace(frame: &mut Frame, state: &State) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    ui_topbar(frame, &main_layout, state);
    ui_statusbar(frame, &main_layout, state);

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);

    ui_spans(frame, &inner_layout, state);
    ui_span(frame, &inner_layout, state);
}

pub fn ui_spans(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

    frame.render_stateful_widget(spans, layout[0], &mut state.spans_state.clone());
}

pub fn ui_span(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

    frame.render_widget(paragraph, layout[1]);
    let margin = &Margin {
        horizontal: 0,
        vertical: 1,
    };
    frame.render_stateful_widget(scrollbar, layout[1].inner(margin), &mut scrollbar_state);
}
