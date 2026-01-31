use ratatui::{
    prelude::*,
    style::palette::tailwind::SLATE,
    symbols::border,
    widgets::{Block, List, ListItem},
};
use time::macros::format_description;

use crate::logparse::{LogDB, LogEvent};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c600).add_modifier(Modifier::BOLD);

pub fn render_transactions(db: &LogDB) -> List<'_> {
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
    let items: Vec<ListItem> = db
        .events
        .iter()
        .map(|(logkey, event)| {
            let event_summary = Line::from(render_event_summary(event));
            // First item in transaction has a date above it
            let items = if logkey.offset == 0 {
                vec![
                    Line::from(render_timestamp(logkey.timestamp)).bold(),
                    event_summary,
                ]
            } else {
                vec![event_summary]
            };
            ListItem::from(items)
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
