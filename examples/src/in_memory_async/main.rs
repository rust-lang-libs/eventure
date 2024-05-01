use std::sync::mpsc::channel;
use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_async;
use eventure::model::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();
    let message_channel = in_memory_async::message_channel(ChannelType::TOPIC, String::from("Orders"));

    in_memory_async::register(message_channel, order_created_handler);
    in_memory_async::emit(&order_created);
    in_memory_async::emit(&order_canceled);
}
