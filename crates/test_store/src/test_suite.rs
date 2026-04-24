use crate::{State, Status, test_case::TestCase};

#[derive(Debug, Clone, Default)]
pub struct TestSuite {
    // Tells the state that the test case is in
    pub state: State,
    // Tells the status of the test case after an execution round
    pub status: Status,
    // Tells the number of times the test case has been executed in this session
    pub runs: u16,
    // Tells the number of test cases in the suite
    pub count: u32,
    // The test cases in the test suite
    pub cases: Vec<TestCase>,
}
impl TestSuite {}
