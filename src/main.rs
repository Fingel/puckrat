use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};

use crate::logparse::{LogDB, ParseError};

pub mod logparse;

#[derive(PartialEq)]
enum Message {
    MoveUp,
    MoveDown,
    Quit,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Model {
    running_state: RunningState,
    logs: LogDB,
}

impl Model {
    pub fn new(content: &str) -> Result<Self, ParseError> {
        Ok(Self {
            logs: LogDB::new(content)?,
            ..Default::default()
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let path = "/var/log/pacman.log";
    let log_content = std::fs::read_to_string(path)?;
    let model = Model::new(&log_content)?;
    ratatui::run(|terminal| run(model, terminal))?;
    Ok(())
}

fn run(mut model: Model, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&model, frame))?;

        if let Some(msg) = handle_event(&model)? {
            process_message(&mut model, msg);
        }
    }
    Ok(())
}

fn process_message(model: &mut Model, msg: Message) {
    let mut current_msg = Some(msg);
    while let Some(msg) = current_msg {
        current_msg = update(model, msg);
    }
}

fn view(model: &Model, frame: &mut Frame) {
    frame.render_widget(render_main(model), frame.area());
}

fn handle_event(_: &Model) -> io::Result<Option<Message>> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(handle_key(key_event)),
        _ => Ok(None),
    }
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::MoveDown),
        KeyCode::Char('k') => Some(Message::MoveUp),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::MoveUp => {
            dbg!("Move UP");
        }
        Message::MoveDown => {
            dbg!("Move DOWN");
        }
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
    };
    None
}

fn render_main(model: &Model) -> Paragraph<'_> {
    let title = Line::from(" Puckman ".bold());
    let instructions = Line::from(vec![
        " Down ".into(),
        "<j>".blue().bold(),
        " Up ".into(),
        "<k>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let tx_cnt = model.logs.transactions.len();
    let counter_text = Text::from(vec![Line::from(vec![
        tx_cnt.to_string().into(),
        " Transactions".into(),
    ])]);

    Paragraph::new(counter_text).centered().block(block)
}
