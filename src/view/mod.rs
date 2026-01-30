mod main_screen;

use crate::model::Model;
use ratatui::Frame;

pub fn view(model: &Model, frame: &mut Frame) {
    // For now, just render main screen
    // Future: match on model.current_screen or similar
    main_screen::render(model, frame);
}
