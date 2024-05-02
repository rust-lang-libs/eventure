use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_sync;
use eventure::in_memory_sync::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let configuration = in_memory_sync::configuration(ChannelType::TOPIC, "*");
    in_memory_sync::setup(configuration);

    let handler_channel = in_memory_sync::message_channel(ChannelType::TOPIC, "Orders");
    in_memory_sync::register(handler_channel, order_created_handler);

    in_memory_sync::emit(&order_created);
    in_memory_sync::emit(&order_canceled);
}
