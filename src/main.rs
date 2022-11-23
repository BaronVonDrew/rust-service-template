extern crate serde_derive;

use actix_health;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use kafka_lib;
use kafka_lib::producer::KafkaProducerService;
use kafka_message::instrument::NewInstrumentMessage;
use kafka_message::price::NewPriceMessage;
use log::info;
use rust_decimal_macros::dec;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
mod test;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Price Service running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,price_service=debug,rdkafka=debug",
    );
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let kafka_config = kafka_lib::get_config(None);

    let producer = kafka_lib::producer::KafkaProducerService::new(kafka_config);
    let producer_clone = producer.clone();

    test_producers_and_consumers(producer);

    info!("Configuring server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                producer: producer_clone.clone(),
            }))
            .configure(|cfg| {
                cfg.service(index);
            })
            .service(actix_health::add_health_endpoints())
            .service(web::scope("/test").configure(test::init))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[derive(Clone)]
pub struct AppState {
    pub producer: KafkaProducerService,
}

fn test_producers_and_consumers(producer: KafkaProducerService) {
    actix_web::rt::spawn(async move {
        producer
            .send_message(
                NewPriceMessage {
                    instrument: "TST".to_string(),
                    timestamp: get_unix_timestamp(),
                    value: dec!(13.1),
                },
                Some(|message| -> &String { &message.instrument }),
            )
            .await;
        producer
            .send_message(
                NewInstrumentMessage {
                    added_timestamp: get_unix_timestamp(),
                    instrument: "Test Instrument".to_string(),
                },
                Some(|message| -> &String { &message.instrument }),
            )
            .await;
    });
}

fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Unable to get Unix timestamp")
        .as_secs()
}
