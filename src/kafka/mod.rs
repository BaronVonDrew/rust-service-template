use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use std::env;

pub mod consumer;
pub mod producer;

pub fn get_config() -> ClientConfig {
    dotenv::dotenv().ok();
    let group_id = env::var("GROUP_ID").expect("GROUP_ID is required");
    let brokers = env::var("BROKERS").expect("GROUP_ID is required");
    let mut config = ClientConfig::new();

    config
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .to_owned()
}
