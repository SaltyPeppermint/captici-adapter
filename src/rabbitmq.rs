use amq_protocol::uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use lapin::message::BasicGetMessage;
use lapin::options::*;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Queue};
use log::info;

pub async fn get_channel() -> Channel {
    let amqp_uri = create_amqp_uri();
    let conn_properties = ConnectionProperties::default().with_default_executor(2);
    let conn = match Connection::connect_uri(amqp_uri, conn_properties).await {
        Ok(c) => c,
        Err(error) => panic!("Problem creating a connection: {:?}", error),
    };
    conn.create_channel()
        .await
        .expect("Problem opening the channel")
}

fn create_amqp_uri() -> AMQPUri {
    let amqp_user_info = AMQPUserInfo {
        username: "user".into(),
        password: "7HPDa9usy4".into(),
    };
    let amqp_authority = AMQPAuthority {
        userinfo: amqp_user_info,
        host: "localhost".into(),
        port: 5672,
    };
    AMQPUri {
        scheme: Default::default(),
        authority: amqp_authority,
        vhost: "/".to_string(),
        query: Default::default(),
    }
}

pub async fn create_queue(channel: &Channel, queue_name: &str) -> Queue {
    let queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Problem declearing queue");
    info!("Declared queue {:?}", queue);
    queue
}

pub async fn publish_message(channel: &Channel, payload: Vec<u8>) {
    let confirm = channel
        .basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
        .expect("Problem publishing message");
    // let result = match confirm {
    //     Confirmation::NotRequested => Ok(()),
    //     _ => io:Err("AAAAAAAAAAAA"),
    // }
    // assert_eq!(confirm, Confirmation::NotRequested);
    info!("Published message {:?}", confirm)
}

pub async fn get_message(channel: &Channel, queue_name: &str) -> Option<BasicGetMessage> {
    let message = channel
        .basic_get(queue_name, BasicGetOptions::default())
        .await;
    let message = match message {
        Ok(m) => m,
        Err(error) => panic!("Problem getting message: {:?}", error),
    };
    info!("Got message {:?}", message);
    message
}

pub async fn ack_message(message: BasicGetMessage) -> Vec<u8> {
    let delivery = message.delivery;
    delivery.ack(BasicAckOptions::default()).await.expect("ack");
    delivery.data
}
