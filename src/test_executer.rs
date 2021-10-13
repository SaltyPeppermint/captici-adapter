use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCaseData {
    pub name: String,
    pub test_type: TestType,
    pub command: String,
    pub args: Vec<String>,
    pub files_to_collect: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TestType {
    Dummy,
    Benchbase,
    FFmpeg,
}

pub fn test(test_case_data: TestCaseData) -> Vec<String> {
    vec!["asdf".to_string()]
}

// fn collect_test_data()
