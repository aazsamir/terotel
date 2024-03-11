use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
};
use jaeger::{Jaeger, Operations, Services, Span, Trace, Traces};
use ratatui::{widgets::*};
use std::{
    fmt::Display,
    io::{self},
};

use crate::jaeger::{self};

/// State of the application
///
/// Each window has its data, state of list and selected item (selected is not the same as hovered).
pub struct State {
    pub services: Option<Services>,
    pub services_state: ListState,
    pub selected_service: Option<String>,
    pub operations: Option<Operations>,
    pub operations_state: ListState,
    pub selected_operation: Option<String>,
    pub traces: Option<Traces>,
    pub traces_state: ListState,
    pub selected_trace: Option<String>,
    pub spans: Option<Vec<Span>>,
    pub spans_state: ListState,
    pub selected_span: Option<String>,
    pub span_text_scroll: u16,
    pub selected_window: Window,
    pub is_search_state: bool,
    pub search_input: String,
    pub should_quit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
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

    pub fn handle_operation(mut self, operation: &Operation, jaeger: &Jaeger) -> Self {
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

    pub fn handle_exit(&mut self) {
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

    pub fn handle_move_vertical(&mut self, up: bool) {
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

    pub fn handle_move_horizontal(&mut self, right: bool) {
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

    pub fn handle_select(&mut self, jaeger: &Jaeger) {
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
                            .find(|t| t.to_string() == *self.selected_trace.as_ref().unwrap())
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

    pub fn handle_search(&mut self) {
        self.is_search_state = !self.is_search_state;
        self.search_input = String::new();
    }

    pub fn handle_search_input(&mut self, c: &char) {
        if c == &'\u{8}' {
            self.search_input.pop();
        } else {
            self.search_input.push(*c);
        }
    }

    pub fn handle_search_enter(&mut self) {
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
pub enum Window {
    Services = 0,
    Operations = 1,
    Traces = 2,
    Spans = 3,
    Span = 4,
}

pub fn handle_list_scroll(list_state: &mut ListState, max: usize, dif: i32) {
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

pub fn handle_list_select<T: Display>(
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

pub enum ListSelectResult {
    None,
    Selected,
    Unselected,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
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

pub fn handle_events(state: &State) -> io::Result<Operation> {
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
