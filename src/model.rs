use ratatui::widgets::ListState;

use crate::logparse::{LogDB, ParseError};

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
}

impl Model {
    pub fn new(content: &str) -> Result<Self, ParseError> {
        let logs = LogDB::new(content)?;
        let mut list_state = ListState::default();
        // List state needs be initialized with a length in order to select an item on
        // initial render
        list_state.select(Some(logs.transactions.len().saturating_sub(1)));
        Ok(Self {
            logs,
            list_state,
            ..Default::default()
        })
    }

    pub fn transaction_count(&self) -> usize {
        self.logs.transactions.len()
    }
}
