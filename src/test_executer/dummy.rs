use crate::test_executer::TestCase;
use crate::test_executer::TestResult;
use crate::test_executer::TestResultContent;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;

pub fn test(test_case: TestCase) -> TestResult {
    let output = execute_test(test_case.command, test_case.args);
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

fn execute_test(test_command: String, test_args: Vec<String>) -> Output {
    Command::new(test_command)
        .args(test_args)
        .output()
        .expect("failed to execute process")
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
