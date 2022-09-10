#[macro_use]
extern crate serde_derive;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use kafka::producer::send;
use log::info;
use rdkafka::{consumer::StreamConsumer, producer::FutureProducer};
use std::env;

mod kafka;
mod test;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Price Service running")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(index).service(health);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,price_service=debug,rdkafka=debug",
    );
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let kafka_config = kafka::get_config();
    let consumer: StreamConsumer = kafka_config.create().expect("Consumer creation failed");
    let producer: FutureProducer = kafka_config.create().expect("Producer creation failed");
    let producer_clone: FutureProducer = producer.clone();

    actix_web::rt::spawn(async move { kafka::consumer::consume(consumer, &["test-price-topic"]).await });

    actix_web::rt::spawn(async move {
        send(&producer_clone, "hello", 1, "test-price-topic").await;
    });

    info!("Configuring server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                producer: producer.clone(),
            }))
            .configure(init)
            .service(web::scope("/test").configure(test::init))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[derive(Clone)]
pub struct AppState {
    producer: FutureProducer,
}
