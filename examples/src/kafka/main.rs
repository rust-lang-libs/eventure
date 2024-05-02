use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::{in_memory_async, in_memory_sync, kafka};
use eventure::model::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let default_channel = kafka::message_channel(ChannelType::TOPIC, "*");
    kafka::setup(default_channel);

    let handler_channel = kafka::message_channel(ChannelType::TOPIC, "Orders");
    kafka::register(handler_channel, order_created_handler);

    kafka::emit(&order_created);
    kafka::emit(&order_canceled);
}
