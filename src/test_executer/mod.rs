mod dummy;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub test_type: TestType,
    pub command: String,
    pub args: Vec<String>,
    pub files_to_collect: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    pub name: String,
    pub test_type: TestType,
    pub content: TestResultContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResultContent {
    pub stdout: String,
    pub stderr: String,
    pub file_contents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TestType {
    Dummy,
    Benchbase,
    FFmpeg,
}

pub fn test(test_case: TestCase) -> TestResult {
    let test_type = &test_case.test_type;
    match test_type {
        TestType::Dummy => dummy::test(test_case),
        _ => panic!("NOT IMPLEMENTED!"),
    }
}

// fn collect_test_data()
