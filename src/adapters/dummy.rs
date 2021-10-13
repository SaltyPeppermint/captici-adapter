use std::process::Command;

pub fn execute_test(test_command: String, test_args: Vec<String>) -> String {
    Command::new("test_command")
        .args(test_args)
        .output()
        .expect("failed to execute process")
}
