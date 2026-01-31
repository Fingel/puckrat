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
        Ok(Self {
            logs: LogDB::new(content)?,
            ..Default::default()
        })
    }

    pub fn transaction_count(&self) -> usize {
        self.logs.transactions.len()
    }
}
