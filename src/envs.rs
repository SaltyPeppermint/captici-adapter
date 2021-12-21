use crate::structs::TestCase;
use std::env;

pub fn gather_envs() -> TestCase {
    //const ENV_KEYS = vec!["TEST_ID", "TEST_GROUP_ID", "TEST_COMMAND", "RESULT_PATH"];
    TestCase {
        test_id: get_single_env("TEST_ID").parse().unwrap(),
        test_group_id: get_single_env("TEST_GROUP_ID").parse().unwrap(),
        namespace: get_single_env("NAMESPACE"),
        test_command: get_single_env("TEST_COMMAND"),
        result_path: get_single_env("RESULT_PATH"),
    }
}

fn get_single_env(key: &str) -> String {
    env::var(key).unwrap()
}
