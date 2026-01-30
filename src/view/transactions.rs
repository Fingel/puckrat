use crate::{logparse::LogEvent, model::Model};
use ratatui::{
    prelude::*,
    style::palette::tailwind::SLATE,
    symbols::border,
    widgets::{Block, List, ListItem},
};
use time::macros::format_description;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render_transactions(model: &Model) -> List<'_> {
    let title = Line::from(" Transactions ".bold());
    let instructions = Line::from(vec![
        " Down ".into(),
        "<j>".blue().bold(),
        " Up ".into(),
        "<k>".blue().bold(),
    ]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    let items: Vec<ListItem> = model
        .transactions()
        .flat_map(|(timestamp, events)| {
            let mut items = vec![ListItem::new(
                Line::from(render_timestamp(*timestamp)).bold(),
            )];
            items.extend(events.iter().map(|event| {
                ListItem::new(Line::from(format!("  {}", render_event_summary(event))))
            }));
            items
        })
        .collect();

    List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
}

fn render_event_summary(event: &LogEvent) -> String {
    match event {
        LogEvent::Downgraded {
            package,
            old_version,
            new_version,
        } => format!("↓ {} ({} → {})", package, old_version, new_version),
        LogEvent::Upgraded {
            package,
            old_version,
            new_version,
        } => format!("↑ {} ({} → {})", package, old_version, new_version),
        LogEvent::Installed { package, version } => {
            format!("+ {} ({})", package, version)
        }
        LogEvent::Removed { package, version } => {
            format!("- {} ({})", package, version)
        }
    }
}

fn render_timestamp(timestamp: i64) -> String {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let date = time::OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
    date.format(&format).unwrap()
}
