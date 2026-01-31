use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};

use crate::{model::Model, view::transactions::render_transactions};

pub fn render(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    let tx_widget = render_transactions(&model.logs);
    frame.render_stateful_widget(tx_widget, layout[0], &mut model.list_state);
    frame.render_widget(render_main(model), layout[1]);
}

fn render_main(model: &Model) -> Paragraph<'_> {
    let title = Line::from(" Puckman ".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let event = model.selected_event();
    if let Some((_key, event)) = event {
        let package_text = Text::from(Line::from(format!("{:?}", event)));
        Paragraph::new(package_text).centered().block(block)
    } else {
        let package_text = Text::from(Line::from("No package selected"));
        Paragraph::new(package_text).centered().block(block)
    }
}
