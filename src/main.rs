mod app;
mod event;
mod logparse;
mod message;
mod model;
mod view;

use model::Model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let path = "/var/log/pacman.log";
    let log_content = std::fs::read_to_string(path)?;
    let model = Model::new(&log_content)?;
    ratatui::run(|terminal| app::run(model, terminal))?;
    Ok(())
}
