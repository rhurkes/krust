extern crate structopt;

use rdkafka::client::ClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext, DefaultConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use std::io::{self, Write};
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Cli {
    // URI of Kafka broker(s) to read messages from
    #[structopt(short = "b", long = "brokers", default_value = "127.0.0.1:9092")]
    brokers: String,

    // Number of messages to display
    #[structopt(short = "n", long = "number", default_value = "0")]
    number: usize,

    // Kafka topic to retrieve messages from
    #[structopt(short = "t", long = "topic", default_value = "mytopic")]
    topic: String,
}

// TODO how to log things? Probably should write to stderr
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(
        &self,
        result: KafkaResult<()>,
        _offsets: *mut rdkafka_sys::RDKafkaTopicPartitionList,
    ) {
        println!("Committing offsets: {:?}", result);
    }
}

fn main() {
    do_stuff()
}

fn do_stuff() {
    let args = Cli::from_args();
    let topics: Vec<&str> = vec![&args.topic];
    let group_id = "test";

    let mut config = ClientConfig::new();
    config.set("group.id", group_id);
    config.set("bootstrap.servers", &args.brokers);
    config.set("enable.partition.eof", "false");
    config.set("session.timeout.ms", "6000");
    config.set("enable.auto.commit", "false");
    config.set("auto.offset.reset", "earliest");

    let consumer: BaseConsumer<DefaultConsumerContext> =
        config.create_with_context(DefaultConsumerContext).unwrap();
    consumer.subscribe(&topics).unwrap();

    loop {
        let message_option = consumer.poll(Duration::from_millis(100));
        match message_option {
            None => {
                // TODO what is error here, how to report?
            }
            Some(message_result) => match message_result {
                Ok(m) => {
                    let payload = match m.payload() {
                        None => &[],
                        Some(bytes) => bytes,
                    };

                    let payload = m.payload().unwrap();
                    write_console(payload);
                }
                Err(e) => println!("Kafka Error: {:?}", e),
            },
        }
    }
}

fn write_console(payload: &[u8]) {
    let output = [payload, b"\n"].concat();
    io::stdout().write_all(&output).unwrap();
}
