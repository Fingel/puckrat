use ratatui::{Frame, prelude::*};

use crate::{
    model::Model,
    view::{pkginfo::render_package, transactions::render_transactions},
};

pub fn render(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(frame.area());
    let tx_widget = render_transactions(&model.logs);
    frame.render_stateful_widget(tx_widget, layout[0], &mut model.list_state);
    let pkg_widget = render_package(model);
    frame.render_widget(pkg_widget, layout[1]);
}
