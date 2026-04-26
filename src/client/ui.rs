use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::client::ApiClient;

pub fn run() {
    let api_key = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: pasm_client <API_KEY>");
        std::process::exit(1);
    });

    let mut client = ApiClient::new(api_key.clone());
    let mut selected = 0usize;
    let mut input_mode = false;
    let mut current_input = 0usize;
    let mut inputs = [String::new(), String::new()];

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).ok();
    enable_raw_mode().ok();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    loop {
        terminal
            .draw(|f| draw(f, &client, selected, input_mode, &inputs, current_input))
            .ok();
        if let Event::Key(key) = event::read().expect("Failed to read event") {
            if key.kind == KeyEventKind::Press {
                if input_mode {
                    match key.code {
                        KeyCode::Char(c) => {
                            inputs[current_input].push(c);
                        }
                        KeyCode::Backspace => {
                            inputs[current_input].pop();
                        }
                        KeyCode::Tab => {
                            current_input = if current_input == 0 { 1 } else { 0 };
                        }
                        KeyCode::Enter => {
                            match selected {
                                0 => client.create(&inputs[0], &inputs[1]),
                                4 => client.amend(&inputs[0], &inputs[1]),
                                _ => {}
                            }
                            inputs = [String::new(), String::new()];
                            current_input = 0;
                            input_mode = false;
                        }
                        KeyCode::Esc => {
                            input_mode = false;
                            inputs = [String::new(), String::new()];
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Up | KeyCode::Char('k') => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if selected < 8 {
                                selected += 1;
                            }
                        }
                        KeyCode::Enter => match selected {
                            0 | 4 => input_mode = true,
                            1 => client.find(&inputs[0]),
                            2 => client.list(),
                            3 => client.delete(&inputs[0]),
                            5 => client.register_auth(),
                            6 => client.update_auth(),
                            7 => client.remove_auth(),
                            8 => client.list_users(),
                            _ => {client.result = format!("wrong index : {selected}")}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();
}

fn draw(
    f: &mut Frame,
    client: &ApiClient,
    selected: usize,
    input_mode: bool,
    inputs: &[String; 2],
    _current_input: usize,
) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ])
        .split(area);

    let items = vec![
        "Create Entry",
        "Find Entry",
        "List Entries",
        "Delete Entry",
        "Amend Entry",
        "Register Auth",
        "Update Auth",
        "Remove Auth",
        "List Users",
    ];

    let list = List::new(items.iter().map(|s| ListItem::new(*s)).collect::<Vec<_>>())
        .block(Block::bordered().title("API Actions").borders(Borders::ALL))
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(selected));
    f.render_stateful_widget(list, chunks[0], &mut state);

    let input_text = if input_mode {
        format!(
            "Key: {}  Value: {}  (Tab: switch, Enter: submit, Esc: cancel)",
            inputs[0], inputs[1]
        )
    } else {
        "Press Enter to select, j/k to move".to_string()
    };
    let input_block =
        Paragraph::new(input_text).block(Block::bordered().title("Input").borders(Borders::ALL));

    f.render_widget(input_block, chunks[1]);

    let result_text = if client.result.is_empty() {
        "Result will appear here...".to_string()
    } else {
        client.result.clone()
    };
    let result = Paragraph::new(result_text)
        .block(Block::bordered().title("Result").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(result, chunks[2]);

    let help = Paragraph::new("j/k: move  |  Enter: select  |  q/Esc: quit")
        .block(Block::bordered().title("Help").borders(Borders::ALL));

    f.render_widget(help, chunks[3]);
}
