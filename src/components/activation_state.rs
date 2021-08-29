use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ActivationState {
    ToBeScheduled,
    Scheduled(i64),
    ReadyToRun,
}
