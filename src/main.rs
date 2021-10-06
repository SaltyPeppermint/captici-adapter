use lapin::Channel;
use log::info;
use serde_json;
use simple_logger;
mod rabbitmq;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    simple_logger::init_with_env().unwrap();

    let channel: Channel = rabbitmq::get_channel().await;
    info!("CONNECTED");
    while let Some(message) = rabbitmq::get_message(&channel, "mychannel").await {
        let data = rabbitmq::ack_message(message).await;
        let json = serde_json::de::from_slice(&data).expect("Deserializing failed");
    }

    // Rest of your program
    Ok(())
}
