use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatuses {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestStatuses::Open => write!(f, "Open"),
            QuestStatuses::InJourney => write!(f, "In Journey"),
            QuestStatuses::Completed => write!(f, "Completed"),
            QuestStatuses::Failed => write!(f, "Failed"),
        }
    }
}
