// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

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

    let order_created = order_created::create();
    let configuration = in_memory::configuration(TOPIC, ".*", false);
    in_memory::setup(configuration);

    let handler_topic_channel = in_memory::message_channel(TOPIC, "Order");
    let order_created_handler = order_created::handler();
    in_memory::register(handler_topic_channel, order_created_handler);

    in_memory::emit(&order_created);

    println!();
}

fn init_logger() {
    SimpleLogger::new()
        .with_colors(false)
        .with_level(LevelFilter::Debug)
        .init().unwrap();
}
