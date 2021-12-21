use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TestResult {
    pub test_id: i32,
    pub test_group_id: i32,
    pub stdout: String,
    pub stderr: String,
    pub result: String,
}

#[derive(Debug)]
pub struct TestCase {
    pub test_id: i32,
    pub test_group_id: i32,
    pub namespace: String,
    pub test_command: String,
    pub result_path: String,
}
