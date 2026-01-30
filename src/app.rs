use ratatui::DefaultTerminal;
use std::io;

use crate::event;
use crate::message::Message;
use crate::model::{Model, RunningState};
use crate::view;

pub fn run(mut model: Model, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view::view(&model, frame))?;

        if let Some(msg) = event::handle_event(&model)? {
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
