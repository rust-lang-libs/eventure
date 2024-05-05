// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::thread;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use eventure::in_memory;
use eventure::in_memory::ChannelType::TOPIC;

use examples::shared::order_created;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Main function
// -----------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    println!();
    init_logger();

    let configuration = in_memory::configuration(TOPIC, ".*", false);
    in_memory::setup(configuration);

    let order_created_handler = order_created::handler();
    let handler_topic_channel = in_memory::message_channel(TOPIC, "Order");
    in_memory::register(handler_topic_channel, order_created_handler);

    let builder = thread::Builder::new().name(String::from("sub-thread1"));
    let handle1 = builder.spawn(|| {
        let order_created = order_created::create();
        in_memory::emit(&order_created);
    });

    let builder = thread::Builder::new().name(String::from("sub-thread2"));
    let handle2 = builder.spawn(|| {
        let order_created = order_created::create();
        in_memory::emit(&order_created);
    });

    handle1.expect("thread expected").join().unwrap();
    handle2.expect("thread expected").join().unwrap();
    println!();
}

fn init_logger() {
    SimpleLogger::new()
        .with_colors(false)
        .with_threads(true)
        .with_level(LevelFilter::Debug)
        .init().unwrap();
}
