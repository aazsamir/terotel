use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::{Jaeger, Operations, RefType, Services, Span, Trace, Traces};
use ratatui::{prelude::*, widgets::*};
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, stdout, Error},
    rc::Rc,
};

pub mod jaeger;

/// State of the application
///
/// Each window has its data, state of list and selected item (selected is not the same as hovered).
struct State {
    services: Option<Services>,
    services_state: ListState,
    selected_service: Option<String>,
    operations: Option<Operations>,
    operations_state: ListState,
    selected_operation: Option<String>,
    traces: Option<Traces>,
    traces_state: ListState,
    selected_trace: Option<String>,
    spans: Option<Vec<Span>>,
    spans_state: ListState,
    selected_span: Option<String>,
    span_text_scroll: u16,
    selected_window: Window,
    is_search_state: bool,
    search_input: String,
    should_quit: bool,
}

impl State {
    fn new() -> Self {
        Self {
            services: None,
            services_state: ListState::default(),
            selected_service: None,
            operations: None,
            operations_state: ListState::default(),
            selected_operation: None,
            traces: None,
            traces_state: ListState::default(),
            selected_trace: None,
            spans: None,
            spans_state: ListState::default(),
            selected_span: None,
            selected_window: Window::Services,
            span_text_scroll: 0,
            is_search_state: false,
            search_input: String::new(),
            should_quit: false,
        }
    }

    fn handle_operation(mut self, operation: &Operation, jaeger: &Jaeger) -> Self {
        match operation {
            Operation::Exit => self.handle_exit(),
            Operation::MoveDown => self.handle_move_vertical(true),
            Operation::MoveUp => self.handle_move_vertical(false),
            Operation::MoveRight => self.handle_move_horizontal(true),
            Operation::MoveLeft => self.handle_move_horizontal(false),
            Operation::Select => self.handle_select(jaeger),
            Operation::Nothing => {}
            Operation::Search => self.handle_search(),
            Operation::SearchInput(c) => self.handle_search_input(c),
            Operation::SearchEnter => self.handle_search_enter(),
        };
        self
    }

    fn handle_exit(&mut self) {
        if let Window::Spans = self.selected_window {
            self.selected_window = Window::Traces;
            // we exitted trace state, so we need to reset selected trace
            self.selected_trace = None;
        } else if let Window::Span = self.selected_window {
            self.selected_window = Window::Traces;
            // we exitted trace state, so we need to reset selected trace
            self.selected_trace = None;
        } else {
            self.should_quit = true;
        }
    }

    fn handle_move_vertical(&mut self, up: bool) {
        let dif = if up { -1 } else { 1 };

        match self.selected_window {
            Window::Services => {
                if let Some(services) = self.services.as_ref() {
                    handle_list_scroll(&mut self.services_state, services.data.len(), dif);
                }
            }
            Window::Operations => {
                if let Some(operations) = self.operations.as_ref() {
                    handle_list_scroll(&mut self.operations_state, operations.data.len(), dif)
                }
            }
            Window::Traces => {
                if let Some(traces) = self.traces.as_ref() {
                    handle_list_scroll(&mut self.traces_state, traces.data.len(), dif)
                }
            }
            Window::Spans => {
                if let Some(spans) = self.spans.as_ref() {
                    handle_list_scroll(&mut self.spans_state, spans.len(), dif)
                }
            }
            Window::Span => {
                if self.selected_span.is_some() {
                    if dif == -1 {
                        if self.span_text_scroll > 0 {
                            self.span_text_scroll -= 1;
                        }
                    } else {
                        self.span_text_scroll += 1;
                    }
                }
            }
        }
    }

    fn handle_move_horizontal(&mut self, right: bool) {
        let selected = self.selected_window as usize;

        if right {
            if selected == Window::Services as usize {
                self.selected_window = Window::Operations;
            } else if selected == Window::Operations as usize {
                self.selected_window = Window::Traces;
            } else if selected == Window::Traces as usize {
                self.selected_window = Window::Services;
            } else if selected == Window::Spans as usize {
                self.selected_window = Window::Span;
            } else if selected == Window::Span as usize {
                self.selected_window = Window::Spans;
            }
        } else if selected == Window::Services as usize {
            self.selected_window = Window::Traces;
        } else if selected == Window::Operations as usize {
            self.selected_window = Window::Services;
        } else if selected == Window::Traces as usize {
            self.selected_window = Window::Operations;
        } else if selected == Window::Spans as usize {
            self.selected_window = Window::Span;
        } else if selected == Window::Span as usize {
            self.selected_window = Window::Spans;
        }
    }

    fn handle_select(&mut self, jaeger: &Jaeger) {
        match self.selected_window {
            Window::Services => {
                if let Some(services) = self.services.as_mut() {
                    match handle_list_select(
                        Some(services.data.clone()),
                        &mut self.services_state,
                        &mut self.selected_service,
                    ) {
                        ListSelectResult::Selected => {
                            let operations = jaeger
                                .get_operations(self.selected_service.as_ref().unwrap())
                                .unwrap();
                            self.operations = Some(operations);
                            self.selected_window = Window::Operations;
                            // todo: set traces_state and spans_state etc to new state?
                            self.selected_operation = None;
                            self.traces = None;
                            self.selected_trace = None;
                            self.spans = None;
                            self.selected_span = None;
                        }
                        ListSelectResult::Unselected => {
                            self.operations = None;
                            self.selected_operation = None;
                            self.traces = None;
                            self.selected_trace = None;
                            self.spans = None;
                            self.selected_span = None;
                        }
                        ListSelectResult::None => {}
                    }
                    self.operations_state = ListState::default();
                }
            }
            Window::Operations => {
                match handle_list_select(
                    self.operations.as_ref().map(|o| o.data.clone()),
                    &mut self.operations_state,
                    &mut self.selected_operation,
                ) {
                    ListSelectResult::Selected => {
                        let mut request = jaeger::TracesRequest::new(
                            self.selected_service.as_ref().unwrap().clone(),
                        );

                        if let Some(to_select) = self.selected_operation.as_ref() {
                            if !to_select.eq("*") {
                                request.operation = Some(to_select.clone());
                            }
                        }

                        let traces = jaeger.get_traces(&request);

                        if traces.is_err() {
                            panic!("Error getting traces: {:?}", traces.err().unwrap());
                        }

                        self.traces = Some(traces.unwrap());
                        self.selected_window = Window::Traces;
                        self.selected_trace = None;
                        self.spans = None;
                        self.selected_span = None;
                    }
                    ListSelectResult::Unselected => {
                        self.traces = None;
                        self.selected_trace = None;
                        self.spans = None;
                        self.selected_span = None;
                    }
                    ListSelectResult::None => {}
                }
                self.traces_state = ListState::default();
            }
            Window::Traces => {
                match handle_list_select(
                    self.traces.as_ref().map(|t| t.data.clone()),
                    &mut self.traces_state,
                    &mut self.selected_trace,
                ) {
                    ListSelectResult::Selected => {
                        let trace = self
                            .traces
                            .as_ref()
                            .unwrap()
                            .data
                            .iter()
                            .find(|t| {
                                t.to_string() == self.selected_trace.as_ref().unwrap().to_string()
                            })
                            .expect("On trace select, trace should be found.");

                        let mut spans = trace.spans.clone();
                        spans.sort_by(|a, b| a.start_time.cmp(&b.start_time));
                        self.spans = Some(spans);
                        self.selected_window = Window::Spans;
                        self.selected_span = None;
                    }
                    ListSelectResult::Unselected => {
                        self.spans = None;
                        self.selected_span = None;
                    }
                    ListSelectResult::None => {}
                }
            }
            Window::Spans => {
                match handle_list_select(
                    self.spans.clone(),
                    &mut self.spans_state,
                    &mut self.selected_span,
                ) {
                    ListSelectResult::None => {}
                    ListSelectResult::Selected => {
                        self.selected_window = Window::Span;
                        self.span_text_scroll = 0;
                    }
                    ListSelectResult::Unselected => {}
                }
            }
            Window::Span => {}
        }
    }

    fn handle_search(&mut self) {
        self.is_search_state = !self.is_search_state;
        self.search_input = String::new();
    }

    fn handle_search_input(&mut self, c: &char) {
        if c == &'\u{8}' {
            self.search_input.pop();
        } else {
            self.search_input.push(*c);
        }
    }

    fn handle_search_enter(&mut self) {
        {
            match self.selected_window {
                Window::Services => {
                    if let Some(services) = &mut self.services {
                        let search = self.search_input.clone();
                        let filtered_services: Vec<String> = services
                            .data
                            .clone()
                            .into_iter()
                            .filter(|s| s.contains(&search))
                            .collect();

                        services.data = filtered_services;
                        self.services_state = ListState::default();
                    }
                }
                Window::Operations => {
                    if let Some(operations) = &mut self.operations {
                        let search = self.search_input.clone();
                        let filtered_operations: Vec<String> = operations
                            .data
                            .clone()
                            .into_iter()
                            .filter(|s| s.contains(&search))
                            .collect();

                        operations.data = filtered_operations;
                        self.operations_state = ListState::default();
                    }
                }
                Window::Traces => {
                    if let Some(traces) = &mut self.traces {
                        let search = self.search_input.clone();
                        let filtered_traces: Vec<Trace> = traces
                            .data
                            .clone()
                            .into_iter()
                            .filter(|s| s.to_string().contains(&search))
                            .collect();
                        let len = filtered_traces.len() as i32;
                        self.traces = Some(Traces {
                            data: filtered_traces,
                            total: len,
                        });
                        self.traces_state = ListState::default();
                    }
                }
                Window::Spans => {
                    if let Some(spans) = &mut self.spans {
                        let search = self.search_input.clone();
                        let filtered_spans: Vec<Span> = spans
                            .clone()
                            .into_iter()
                            .filter(|s| s.operation_name.contains(&search))
                            .collect();
                        self.spans = Some(filtered_spans);
                        self.spans_state = ListState::default();
                    }
                }
                Window::Span => {}
            }
            self.is_search_state = false;
        }
    }
}

#[derive(Clone, Copy)]
enum Window {
    Services = 0,
    Operations = 1,
    Traces = 2,
    Spans = 3,
    Span = 4,
}

fn main() -> io::Result<()> {
    let jaeger = Jaeger::new("http://localhost:16686");
    let mut state = State::new();
    let services = jaeger.get_services().unwrap();
    state.services = Some(services);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut operation: Result<Operation, Error>;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| {
            ui(f, &state);
        })?;
        operation = handle_events(&state);

        if let Ok(op) = operation {
            state = state.handle_operation(&op, &jaeger);
        }

        should_quit = state.should_quit;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_list_scroll(list_state: &mut ListState, max: usize, dif: i32) {
    if let Some(selected) = list_state.selected() {
        let selected = selected as i32 + dif;

        if selected >= 0 && selected < (max as i32) {
            list_state.select(Some(selected as usize))
        }

        // overflow
        if selected >= (max as i32) {
            list_state.select(Some(0));
        } else if selected < 0 && max > 0 {
            list_state.select(Some(max - 1));
        }
    } else {
        list_state.select(Some(0));
    }
}

fn handle_list_select<T: Display>(
    list_option: Option<Vec<T>>,
    list_state: &mut ListState,
    selected: &mut Option<String>,
) -> ListSelectResult
where
    T: Display + Clone,
{
    if let Some(list) = list_option {
        let hover = list_state.selected();

        if let Some(hovered) = hover {
            let to_select = list[hovered].to_string();

            return if selected.is_some() && selected.as_ref().unwrap() == &to_select {
                *selected = None;
                ListSelectResult::Unselected
            } else {
                *selected = Some(to_select);
                ListSelectResult::Selected
            };
        }
    }

    ListSelectResult::None
}

enum ListSelectResult {
    None,
    Selected,
    Unselected,
}

#[derive(Debug, PartialEq)]
enum Operation {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Select,
    Exit,
    Nothing,
    Search,
    SearchInput(char),
    SearchEnter,
}

fn handle_events(state: &State) -> io::Result<Operation> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read().unwrap() {
            if state.is_search_state {
                if is_keycode_pressed(key, KeyCode::Esc) {
                    return Ok(Operation::Search);
                }
                if let KeyCode::Char(c) = key.code {
                    return Ok(Operation::SearchInput(c));
                }
                if is_keycode_pressed(key, KeyCode::Backspace) {
                    return Ok(Operation::SearchInput('\u{8}'));
                }
                if is_keycode_pressed(key, KeyCode::Enter) {
                    return Ok(Operation::SearchEnter);
                }
            }
            if is_char_pressed(key, 'q') {
                return Ok(Operation::Exit);
            }
            if is_char_pressed(key, 'j') {
                return Ok(Operation::MoveUp);
            }
            if is_char_pressed(key, 'k') {
                return Ok(Operation::MoveDown);
            }
            if is_char_pressed(key, 'h') {
                return Ok(Operation::MoveLeft);
            }
            if is_char_pressed(key, 'l') {
                return Ok(Operation::MoveRight);
            }
            if is_char_pressed(key, 'e') {
                return Ok(Operation::Select);
            }
            if is_char_pressed(key, '/') {
                return Ok(Operation::Search);
            }
            if is_keycode_pressed(key, KeyCode::Enter) {
                return Ok(Operation::Select);
            }
        }
    }
    Ok(Operation::Nothing)
}

fn is_char_pressed(key: KeyEvent, key_char: char) -> bool {
    is_keycode_pressed(key, KeyCode::Char(key_char))
}

fn is_keycode_pressed(key: KeyEvent, key_code: KeyCode) -> bool {
    if key.kind == event::KeyEventKind::Press && key.code == key_code {
        return true;
    }
    false
}

fn ui(frame: &mut Frame, state: &State) {
    if let Window::Spans = state.selected_window {
        ui_trace(frame, state);
    } else if let Window::Span = state.selected_window {
        ui_trace(frame, state);
    } else {
        ui_main(frame, state)
    }
}

fn ui_main(frame: &mut Frame, state: &State) {
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

fn ui_topbar(frame: &mut Frame, layout: &Rc<[Rect]>, _state: &State) {
    let text = "Terotel - Terminal OTEL Viewer";
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, layout[0]);
}

fn ui_statusbar(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

fn ui_services(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

fn ui_operations(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

fn ui_traces(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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

fn ui_trace(frame: &mut Frame, state: &State) {
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

fn ui_spans(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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
        let mut line_text = format!("{}|{}", span.span_id, span.operation_name);
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

fn ui_span(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
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
        .find(|s| s.to_string() == state.selected_span.as_ref().unwrap().to_string())
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
    frame.render_stateful_widget(scrollbar, layout[1].inner(&margin), &mut scrollbar_state);
}
