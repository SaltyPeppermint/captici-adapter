// use bincode;
mod rabbitmq;
mod test_executer;

use lapin::Channel;
use log::info;
use serde_json;
use simple_logger;

macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    simple_logger::init_with_env().unwrap();

    let channel: Channel = rabbitmq::get_channel().await;
    info!("CONNECTED");

    let example_test_case = test_executer::TestCase {
        name: String::from("Test case"),
        test_type: test_executer::TestType::Dummy,
        command: String::from("sh"),
        args: string_vec!("-c", "bla"),
        files_to_collect: string_vec!("test.log", "result.xml"),
    };

    let example_queue = "example-queue";
    // let serialized_example = bincode::serialize(&example_test_case).expect("Couldn't serialize: ");
    let serialized_example = serde_json::to_vec(&example_test_case).expect("Couldn't serialize");

    rabbitmq::declare_queue(&channel, example_queue).await;
    info!("Decleared queue {}!", &example_queue);

    info!("Sending message: {:?}", &serialized_example);
    rabbitmq::publish_message(&channel, &example_queue, serialized_example).await;

    while let Some(message) = rabbitmq::get_message(&channel, &example_queue).await {
        let data = rabbitmq::ack_message(message).await;
        // let test_case: TestCaseData = bincode::deserialize(&data).expect("Deserializing failed");
        let test_case: test_executer::TestCase =
            serde_json::from_slice(&data).expect("Deserializing failed");
        info!("Recieved message: {:#?}", &test_case);
        let test_result = test_executer::test(test_case);
        info!("Recieved message: {:#?}", &test_result);
    }

    // Rest of your program
    Ok(())
}
