use rdkafka::message::BorrowedMessage;
use rdkafka::error::KafkaResult;
use futures::Stream;
use clap::{value_t, App, Arg};
use futures::stream::FuturesUnordered;
use futures::{StreamExt};

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer};
use std::future::Future;

use crate::example_utils::setup_logger;

mod example_utils;

fn run_async_processor<'s, S>(stream: S) -> impl Future<Output=()> + Send + 's
where
    S: Stream<Item = KafkaResult<BorrowedMessage<'s>>> + 's + Send,
{
    stream.map(|borrowed_message_result| {
        async { borrowed_message_result.expect("kafka consumer error").detach() }
    }).buffered(100).for_each(|_| {
        async { }
    })
}

#[tokio::main]
async fn main() {
    let matches = App::new("Async spawn example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Asynchronous spawn example")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .short("g")
                .long("group-id")
                .help("Consumer group id")
                .takes_value(true)
                .default_value("example_consumer_group_id"),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input-topic")
                .long("input-topic")
                .help("Input topic")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("num-workers")
                .long("num-workers")
                .help("Number of workers")
                .takes_value(true)
                .default_value("1"),
        )
        .get_matches();

    setup_logger(true, matches.value_of("log-conf"));

    let brokers = matches.value_of("brokers").unwrap().to_string();
    let group_id = matches.value_of("group-id").unwrap().to_string();
    let input_topic = matches.value_of("input-topic").unwrap().to_string();
    let num_workers = value_t!(matches, "num-workers", usize).unwrap();

    (0..num_workers)
        .map(|_| {
            let group_id = group_id.clone();
            let brokers = brokers.clone();
            let input_topic = input_topic.clone();
            tokio::spawn(async move {
                let consumer: StreamConsumer = ClientConfig::new()
                    .set("group.id", &group_id)
                    .set("bootstrap.servers", &brokers)
                    .set("enable.partition.eof", "false")
                    .set("session.timeout.ms", "6000")
                    .set("enable.auto.commit", "false")
                    .create()
                    .expect("Consumer creation failed");

                consumer
                    .subscribe(&[&input_topic])
                    .expect("Can't subscribe to specified topic");

                let stream = consumer.start();

                run_async_processor(stream).await;
            })
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { })
        .await;
}
