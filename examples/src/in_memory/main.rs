// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use log::LevelFilter;
use simple_logger::SimpleLogger;
use examples::shared::order_created;
use examples::shared::order_canceled;
use eventure::in_memory;
use eventure::in_memory::{MessageChannel};
use eventure::in_memory::ChannelType::{QUEUE, TOPIC};

// -----------------------------------------------------------------------------------------------------------------------------------------
// Main function
// -----------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    println!();
    init_logger();

    let order_created = order_created::create();
    let order_canceled = order_canceled::create();

    let configuration = in_memory::configuration(TOPIC, ".*", false);
    in_memory::setup(configuration);

    let handler_topic_channel = in_memory::message_channel(TOPIC, "Order");
    let order_created_handler = order_created::handler();
    in_memory::register(handler_topic_channel, order_created_handler);

    let handler_queue_channel = in_memory::message_channel(QUEUE, "Order");
    let order_created_handler = order_created::handler();
    in_memory::register(handler_queue_channel, order_created_handler);

    in_memory::emit(&order_created);
    in_memory::emit(&order_canceled);
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "Account.*" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "*" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "Orders" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: QUEUE, name: "Orders" });

    let order_created_handler = order_created::handler();
    in_memory::unregister(order_created_handler);
    let order_created_handler = order_created::handler();
    in_memory::unregister(order_created_handler);

    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: QUEUE, name: "Orders" });

    println!();
}

fn init_logger() {
    SimpleLogger::new()
        .with_colors(false)
        .with_level(LevelFilter::Debug)
        .init().unwrap();
}
