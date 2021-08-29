use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Schedule {
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub weekdays: [bool; 7],
}
