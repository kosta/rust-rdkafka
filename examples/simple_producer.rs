#[macro_use] extern crate log;
extern crate clap;
extern crate futures;
extern crate rdkafka;

use clap::{App, Arg};
use futures::*;

use rdkafka::config::Config;
use rdkafka::producer::Producer;
use rdkafka::util::get_rdkafka_version;

mod example_utils;
use example_utils::setup_logger;


fn produce(brokers: &str, topic: &str) {
    let producer = Config::new()
        .set("bootstrap.servers", brokers)
        .create::<Producer>()
        .unwrap();

    let _producer_thread = producer.start_polling_thread();

    let topic = producer.get_topic(topic)
        .set("produce.offset.report", "true")
        .create()
        .expect("Topic creation error");

    let futures = (0..10)
        .map(|i| {
            let value = format!("Message {}", i);
            producer.send_copy(&topic, Some(&value), Some(&"key"))
                .expect("Production failed")
                .map(move |m| {
                    info!("Delivery status for message {} received", i);
                    m
                })
        })
        .collect::<Vec<_>>();

    for future in futures {
        info!("Future completed. Result: {:?}", future.wait());
    }
}

fn main() {
    let matches = App::new("producer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line producer")
        .arg(Arg::with_name("brokers")
             .short("b")
             .long("brokers")
             .help("Broker list in kafka format")
             .takes_value(true)
             .default_value("localhost:9092"))
        .arg(Arg::with_name("log-conf")
             .long("log-conf")
             .help("Configure the logging format")
             .default_value("rdkafka=trace"))
        .arg(Arg::with_name("topic")
             .short("t")
             .long("topic")
             .help("Destination topic")
             .takes_value(true)
             .required(true))
        .get_matches();

    setup_logger(true, matches.value_of("log-conf"));

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = matches.value_of("topic").unwrap();
    let brokers = matches.value_of("brokers").unwrap();

    produce(brokers, topic);
}

