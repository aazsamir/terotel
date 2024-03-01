use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::{Jaeger, Operations, Services, Traces};
use ratatui::{prelude::*, widgets::*};
use std::{
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
    selected_window: Window,
    is_search_state: bool,
    search_input: String,
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
            selected_window: Window::Services,
            is_search_state: false,
            search_input: String::new(),
        }
    }
}

enum Window {
    Services = 0,
    Operations = 1,
    Traces = 2,
}

fn main() -> io::Result<()> {
    let jaeger = Jaeger::new("http://localhost:16686");
    let mut state = State::new();
    let services = jaeger.get_services().unwrap();
    state.services = Some(services);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut operation: Result<Operation, Error>;

    while !should_quit {
        terminal.draw(|f| {
            ui(f, &state);
        })?;
        operation = handle_events(&state);

        if operation.is_ok() {
            match operation.as_ref().unwrap() {
                Operation::Exit => should_quit = true,
                Operation::MoveDown => match state.selected_window {
                    Window::Services => {
                        if let Some(services) = state.services.as_ref() {
                            handle_list_scroll(&mut state.services_state, services.data.len(), -1);
                        }
                    }
                    Window::Operations => {
                        if let Some(operations) = state.operations.as_ref() {
                            handle_list_scroll(
                                &mut state.operations_state,
                                operations.data.len(),
                                -1,
                            )
                        }
                    }
                    Window::Traces => {
                        if let Some(traces) = state.traces.as_ref() {
                            handle_list_scroll(&mut state.traces_state, traces.data.len(), -1)
                        }
                    }
                },
                Operation::MoveUp => match state.selected_window {
                    Window::Services => {
                        if let Some(services) = state.services.as_ref() {
                            handle_list_scroll(&mut state.services_state, services.data.len(), 1)
                        }
                    }
                    Window::Operations => {
                        if let Some(operations) = state.operations.as_ref() {
                            handle_list_scroll(
                                &mut state.operations_state,
                                operations.data.len(),
                                1,
                            )
                        }
                    }
                    Window::Traces => {
                        if let Some(traces) = state.traces.as_ref() {
                            handle_list_scroll(&mut state.traces_state, traces.data.len(), 1)
                        }
                    }
                },
                Operation::MoveRight => {
                    let selected = state.selected_window as usize;

                    if selected == Window::Services as usize {
                        state.selected_window = Window::Operations;
                    } else if selected == Window::Operations as usize {
                        state.selected_window = Window::Traces;
                    } else {
                        state.selected_window = Window::Services;
                    }
                }
                Operation::MoveLeft => {
                    let selected = state.selected_window as usize;

                    if selected == Window::Services as usize {
                        state.selected_window = Window::Traces;
                    } else if selected == Window::Operations as usize {
                        state.selected_window = Window::Services;
                    } else {
                        state.selected_window = Window::Operations;
                    }
                }
                Operation::Select => match state.selected_window {
                    Window::Services => {
                        if state.services.is_none() {
                            {}
                        }

                        // hovered element, from scrolling
                        let hover = state.services_state.selected();

                        // if any is hovered
                        if let Some(hovered) = hover {
                            // hovered is index, now we get the actual value
                            let to_select =
                                state.services.as_ref().unwrap().data[hovered].to_string();

                            // if to select element is the same as hovered, unselect
                            if state.selected_service.is_some()
                                && state.selected_service.as_ref().unwrap() == &to_select
                            {
                                // unselect
                                state.selected_service = None;
                                // purge operations
                                state.operations = None;
                            } else {
                                // otherwise, select it, and fetch operations of that service
                                state.selected_service = Some(to_select);
                                let operations = jaeger
                                    .get_operations(state.selected_service.as_ref().unwrap())
                                    .unwrap();

                                // save in state
                                state.operations = Some(operations);
                                // change window to next one
                                state.selected_window = Window::Operations;
                                // unselect any selected operation, as they are not valid anymore
                                state.selected_operation = None;
                            }

                            // after all, we need to reset list state
                            state.operations_state = ListState::default();
                        }
                    }
                    Window::Operations => {
                        // ensure that there are any operations that we can select
                        if state.operations.is_none() {
                            {}
                        }

                        // and that there is a selected service, to fetch traces
                        if state.selected_service.is_none() {
                            {}
                        }

                        let hover = state.operations_state.selected();
                        if let Some(hovered) = hover {
                            let to_select =
                                state.operations.as_ref().unwrap().data[hovered].to_string();

                            // unselect on same operation select
                            if state.selected_operation.is_some()
                                && state.selected_operation.as_ref().unwrap() == &to_select
                            {
                                state.selected_operation = None;
                                state.traces = None;
                            } else {
                                // otherwise select operation
                                state.selected_operation = Some(to_select);
                                // fetch traces for given service
                                let mut request = jaeger::TracesRequest::new(
                                    state.selected_service.as_ref().unwrap().clone(),
                                );

                                // if selected operation is not *, add it to request
                                if let Some(to_select) = state.selected_operation.as_ref() {
                                    if !to_select.eq("*") {
                                        request.operation = Some(to_select.clone());
                                    }
                                }

                                let traces = jaeger.get_traces(&request);

                                // todo: proper error handling
                                if traces.is_err() {
                                    panic!("Error getting traces: {:?}", traces.err().unwrap());
                                }

                                state.traces = Some(traces.unwrap());
                                state.selected_window = Window::Traces;
                                state.selected_trace = None;
                            }

                            // after all, reset list state
                            state.traces_state = ListState::default();
                        }
                    }
                    _ => {}
                },
                Operation::Nothing => {}
                Operation::Search => {
                    state.is_search_state = !state.is_search_state;
                    state.search_input = String::new();
                }
                Operation::SearchInput(c) => {
                    if c == &'\u{8}' {
                        state.search_input.pop();
                    } else {
                        state.search_input.push(*c);
                    }
                }
                Operation::SearchEnter => {
                    match state.selected_window {
                        Window::Services => {
                            // if let Some(services) = state.services.as_ref() {
                            //     let search = state.search_input.clone();
                            //     state.services.unwrap().data = services
                            //         .data
                            //         .clone()
                            //         .into_iter()
                            //         .filter(|s| s.contains(&search))
                            //         .collect();
                            //     state.services_state = ListState::default();
                            // }
                        }
                        Window::Operations => {
                            // if let Some(operations) = state.operations.as_ref() {
                            //     let search = state.search_input.clone();
                            //     state.operations.unwrap().data = operations
                            //         .data
                            //         .clone()
                            //         .into_iter()
                            //         .filter(|s| s.contains(&search))
                            //         .collect();
                            //     state.operations_state = ListState::default();
                            // }
                        }
                        Window::Traces => {
                            // if let Some(traces) = state.traces.as_ref() {
                            //     let search = state.search_input.clone();
                            //     state.traces.unwrap().data = traces
                            //         .data
                            //         .into_iter()
                            //         .filter(|s| s.to_string().contains(&search))
                            //         .collect();
                            //     state.traces_state = ListState::default();
                            // }
                        }
                    }
                }
            }
        }
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
        }
    } else {
        list_state.select(Some(0));
    }
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
                if is_keycode_pressed(key, KeyCode::Esc){
                    return Ok(Operation::Search);
                }
                if let KeyCode::Char(c) = key.code {
                    return Ok(Operation::SearchInput(c));
                }
                if is_keycode_pressed(key, KeyCode::Backspace) {
                    return Ok(Operation::SearchInput('\u{8}'));
                }
                if is_keycode_pressed(key, KeyCode::Enter) {
                    return Ok(Operation::Search);
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
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("Terotel - Terminal OTEL Viewer"),
        main_layout[0],
    );
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

fn ui_statusbar(frame: &mut Frame, layout: &Rc<[Rect]>, state: &State) {
    let text: String;

    if state.is_search_state {
        let mut search_input = "(ESC?)> ".to_string();
        search_input.push_str(&state.search_input);
        text = search_input;
    } else {
        text = "q - Quit | hjkl - Move | e - Select | / - Search".to_string();
    }
    let paragraph = Paragraph::new(text);
    frame.render_widget(
        paragraph,
        // Block::new()
        //     .borders(Borders::TOP)
        //     .title("q - Quit | hjkl - Move | e - Select| / - Search"),
        layout[2],
    );
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
