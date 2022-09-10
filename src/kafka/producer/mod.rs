use std::time::Duration;

use log::info;
use rdkafka::{
    message::OwnedHeaders,
    producer::{FutureProducer, FutureRecord},
};

pub async fn send<'a>(
    producer: &'a FutureProducer,
    payload: &str,
    key: i32,
    topic_name: &str,
) -> (i32, i64) {
    info!("Sending test message");
    let delivery_status = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("Message {}", payload))
                .key(&format!("Key {}", key))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            Duration::from_secs(0),
        )
        .await;

    // This will be executed when the result is received.
    info!("Delivery status for message {} received", key);
    delivery_status.expect("Error delivering message")
}
