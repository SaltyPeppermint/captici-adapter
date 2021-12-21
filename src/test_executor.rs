use crate::structs::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;

pub fn test(env_vars: &TestCase) -> TestResult {
    let output = execute_test(&env_vars.test_command);
    let file_contents = collect_result_file(&env_vars.result_path);
    let stdout = String::from_utf8(output.stdout).expect("Couldn't convert stdout to String");
    let stderr = String::from_utf8(output.stderr).expect("Couldn't convert stderr to String");
    TestResult {
        test_id: env_vars.test_id,
        test_group_id: env_vars.test_group_id,
        stdout: stdout,
        stderr: stderr,
        result: file_contents,
    }
}

fn execute_test(test_command: &String) -> Output {
    Command::new(test_command)
        .output()
        .expect("failed to execute process")
}

fn collect_result_file(filename: &String) -> String {
    let path = Path::new(filename);
    fs::read_to_string(path).expect("Unable to read result file")
}
