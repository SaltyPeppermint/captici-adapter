mod envs;
mod structs;
mod test_executor;

use log::info;
use simple_logger::SimpleLogger;
// macro_rules! string_vec {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    SimpleLogger::new().init().unwrap();
    info!("I'm logging");

    let env_vars = envs::gather_envs();
    info!(
        "Gathered environnement vars:\n {:?} \n Now running test...",
        &env_vars
    );

    let result = test_executor::test(&env_vars);
    info!("Ran test. Gathered result:\n {:?}", &result);

    let json_result = serde_json::to_string(&result).unwrap();
    info!("Serialized result into json:\n {:?}", &result);

    let string_list = vec![
        "http://cdpb-test-orchestrator".to_owned(),
        env_vars.namespace,
        "svc.cluster.local/internal/result".to_owned(),
    ];
    let test_orchestrator = string_list.join(".");

    info!("Uploading to {:?} ...", &test_orchestrator);
    let client = reqwest::blocking::Client::new();
    let res = client.post(test_orchestrator).body(json_result).send();

    info!("Upload result: {:?}", res.unwrap().status())
}
