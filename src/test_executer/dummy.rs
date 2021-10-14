use crate::test_executer::structs::TestCaseContent;
use std::process::Command;
use std::process::Output;

pub fn execute_test(test_content: TestCaseContent) -> Output {
    Command::new(test_content.command)
        .args(test_content.args)
        .output()
        .expect("failed to execute process")
}
