use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub test_type: TestType,
    pub files_to_collect: Vec<String>,
    pub content: TestCaseContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCaseContent {
    pub command: String,
    pub args: Vec<String>,
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
