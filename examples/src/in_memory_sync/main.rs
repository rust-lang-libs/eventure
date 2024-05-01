use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::{in_memory_async, in_memory_sync};
use eventure::model::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();
    let message_channel = in_memory_async::message_channel(ChannelType::TOPIC, String::from("Orders"));

    in_memory_sync::register(message_channel, order_created_handler);
    in_memory_sync::emit(&order_created);
    in_memory_sync::emit(&order_canceled);
}
