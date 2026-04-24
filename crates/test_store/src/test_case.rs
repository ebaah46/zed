use crate::{State, Status};

#[derive(Debug, Clone)]
pub struct ErrorInfo {}

#[derive(Debug, Clone, Default)]
pub struct TestCase {
    // Tells the state that the test case is in
    pub state: State,
    // Tells the status of the test case after an execution round
    pub status: Status,
    // Tells the number of times the test case has been executed in this session
    pub runs: u16,
    // Tells the error information provided after the execution of the test case
    pub error_info: Option<ErrorInfo>,
}

impl TestCase {}
