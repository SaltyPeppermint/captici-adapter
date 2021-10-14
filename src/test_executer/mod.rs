mod dummy;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Output;

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

pub fn test(test_case: TestCase) -> TestResult {
    let output = match test_case.test_type {
        TestType::Dummy => dummy::execute_test(test_case.content),
        _ => panic!("NOT IMPLEMENTED!"),
    };
    let file_contents = test_case
        .files_to_collect
        .iter()
        .map(collect_result_file)
        .collect::<Vec<String>>();
    TestResult {
        name: test_case.name,
        test_type: test_case.test_type,
        content: to_test_result_content(output, file_contents),
    }
}

fn collect_result_file(filename: &String) -> String {
    let path = Path::new(filename);
    fs::read_to_string(path).expect("Unable to read result file")
}

fn to_test_result_content(output: Output, file_contents: Vec<String>) -> TestResultContent {
    TestResultContent {
        stdout: String::from_utf8(output.stdout).expect("Couldn't convert stdout to String"),
        stderr: String::from_utf8(output.stderr).expect("Couldn't convert stderr to String"),
        file_contents,
    }
}
