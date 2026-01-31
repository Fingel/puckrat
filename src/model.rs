use ratatui::widgets::ListState;

use crate::logparse::{LogDB, LogEvent, LogKey, ParseError};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub logs: LogDB,
    pub list_state: ListState,
    selected_event: usize,
}

impl Model {
    pub fn new(content: &str) -> Result<Self, ParseError> {
        let logs = LogDB::new(content)?;
        let mut list_state = ListState::default();
        let initial_event = logs.events.len().saturating_sub(1);
        // List state needs be initialized with a length in order to select an item on
        // initial render
        list_state.select(Some(initial_event));
        Ok(Self {
            logs,
            list_state,
            selected_event: initial_event,
            ..Default::default()
        })
    }

    pub fn event_count(&self) -> usize {
        self.logs.events.len()
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
        self.update_selected_event();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
        self.update_selected_event();
    }

    fn update_selected_event(&mut self) {
        if let Some(selected) = self.list_state.selected()
            && selected < self.event_count()
        {
            self.selected_event = selected;
        }
    }

    pub fn selected_event(&self) -> Option<(&LogKey, &LogEvent)> {
        self.logs.events.iter().nth(self.selected_event)
    }
}
