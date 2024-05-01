use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::{in_memory_async, kafka};
use eventure::model::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();
    let message_channel = in_memory_async::message_channel(ChannelType::TOPIC, String::from("Orders"));

    kafka::register(message_channel, order_created_handler);
    kafka::emit(&order_created);
    kafka::emit(&order_canceled);
}
