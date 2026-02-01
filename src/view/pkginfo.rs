use crate::{alpmutil::PackageInfo, model::Model};
use ratatui::{
    prelude::*,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

pub fn render_package(model: &Model) -> Paragraph<'_> {
    let event = model.selected_event();
    let package_name = if let Some(event) = event {
        event.1.package()
    } else {
        "Puckman"
    };
    let title = Line::from(format!(" {} ", package_name).bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .padding(Padding::new(1, 1, 1, 1))
        .border_set(border::THICK);

    let package_info = model.alpm_service.package_info(package_name).ok();
    let package_text = render_pkg_info(package_info);
    Paragraph::new(package_text)
        .wrap(Wrap { trim: false })
        .block(block)
}

fn render_pkg_info(package_info: Option<PackageInfo>) -> Text<'static> {
    if let Some(info) = package_info {
        Text::from(vec![
            i_line("Version", &info.version),
            i_line("Description", &info.description),
            i_option("Architecture", &info.arch),
            i_option("URL", &info.url),
            i_list("Licenses", &info.licenses),
        ])
    } else {
        Text::from(Line::from("No package selected".to_string()))
    }
}

fn i_option(label: &str, value: &Option<String>) -> Line<'static> {
    i_line(label, value.as_ref().unwrap_or(&"None".to_string()))
}

fn i_list(label: &str, items: &[String]) -> Line<'static> {
    i_line(label, &items.join(", "))
}

fn i_line(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:<16} : ", label), Style::default().bold()),
        Span::raw(value.to_string()),
    ])
}
