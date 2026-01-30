use crate::model::Model;
use ratatui::{
    Frame,
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};

pub fn render(model: &Model, frame: &mut Frame) {
    frame.render_widget(render_main(model), frame.area());
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

    let tx_cnt = model.transaction_count();
    let counter_text = Text::from(vec![Line::from(vec![
        tx_cnt.to_string().into(),
        " Transactions".into(),
    ])]);

    Paragraph::new(counter_text).centered().block(block)
}
