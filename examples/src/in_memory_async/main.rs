use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_async;
use eventure::model::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let default_channel = in_memory_async::message_channel(ChannelType::TOPIC, "*");
    in_memory_async::setup(default_channel);

    let handler_channel = in_memory_async::message_channel(ChannelType::TOPIC, "Orders");
    in_memory_async::register(handler_channel, order_created_handler);

    in_memory_async::emit(&order_created);
    in_memory_async::emit(&order_canceled);
}
