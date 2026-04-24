pub mod test_case;
pub mod test_discovery;
pub mod test_suite;

#[derive(Debug, Clone, Default)]
pub enum State {
    Executed,
    #[default]
    NotExecuted,
    Disabled,
}

#[derive(Debug, Clone, Default)]
pub enum Status {
    Pass,
    Failed,
    Skipped,
    Crashed,
    #[default]
    Initialized,
}
