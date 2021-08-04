#[derive(Copy, Clone, PartialEq, Default)]
pub struct Schedule {
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub repeat: bool,
}