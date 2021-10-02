use lapin::{Connection, ConnectionProperties, Result};
use amq_protocol::uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use log::info;
use simple_logger;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    simple_logger::init_with_env().unwrap();


    let amqp_user_info = AMQPUserInfo{
        username: "user".into(),
        password: "FvkwuL1Jac".into(),
    };
    let amqp_authority = AMQPAuthority {
        userinfo: amqp_user_info,
        host: "localhost".into(),
        port: 5672,
    };
    let amqp_uri = AMQPUri {
        scheme: Default::default(),
        authority: amqp_authority,
        vhost: "/".to_string(),
        query: Default::default(),
    };
    let conn_properties = ConnectionProperties::default().with_default_executor(2);
    let conn = Connection::connect_uri(amqp_uri, conn_properties).await?;
    let mq_channel = conn.create_channel().await?;

    info!("CONNECTED");

    // Rest of your program
    Ok(())
}
