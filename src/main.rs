mod alpmutil;
mod app;
mod event;
mod logparse;
mod message;
mod model;
mod view;

use model::Model;

use crate::alpmutil::AlpmService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let alpm_service = AlpmService::new()?;
    let path = "/var/log/pacman.log";
    let log_content = std::fs::read_to_string(path)?;
    let model = Model::new(alpm_service, &log_content)?;
    ratatui::run(|terminal| app::run(model, terminal))?;
    Ok(())
}
