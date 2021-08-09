#[derive(Clone, PartialEq, Debug)]
pub enum ActivationState {
    ToBeScheduled,
    Scheduled(i64),
    ReadyToRun,
}
