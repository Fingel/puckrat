use crate::logparse::{LogDB, LogEvent, ParseError};

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

    pub fn transactions(&self) -> impl Iterator<Item = (&i64, &Vec<LogEvent>)> {
        self.logs.transactions.iter().rev()
    }
}
