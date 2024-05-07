// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::{thread, time};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use eventure::kafka;
use examples::shared::order_created;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Main function
// -----------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    println!();
    init_logger();

    let configuration = kafka::configuration("orders", 0);
    kafka::setup(configuration);

    let order_created = order_created::create();
    kafka::emit(&order_created);

    let message_channel = kafka::message_channel("orders", 0);
    let order_created_handler = order_created::handler();
    kafka::register(message_channel, order_created_handler);

    let duration = time::Duration::from_secs(5);
    thread::sleep(duration);

    let order_created = order_created::create();
    kafka::emit(&order_created);

    let duration = time::Duration::from_secs(5);
    thread::sleep(duration);

    let order_created = order_created::create();
    kafka::emit(&order_created);

    let duration = time::Duration::from_secs(5);
    thread::sleep(duration);

    println!();
}

fn init_logger() {
    SimpleLogger::new()
        .with_colors(false)
        .with_level(LevelFilter::Debug)
        .init().unwrap();
}
