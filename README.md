# Rust Service Template

A template rust service using actix-web and rdkafka

## Setup

To get started you'll need to add a .env file with the following keys:

```
# location of the kafka broker
BROKERS
# group id for the consumer
GROUP_ID
# if using docker, the network running kafka to attach to
KAFKA_NETWORK
```

you can then run using `cargo run`

## Docker

To run using the docker-compose you'll need to also add a .docker.env file which contains overrides for values in the .env file to be used when running docker (for example the host name of kafka if both are running in docker)
