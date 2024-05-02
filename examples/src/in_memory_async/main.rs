use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_async;
use eventure::in_memory_async::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let configuration = in_memory_async::configuration(ChannelType::TOPIC, "*", false);
    in_memory_async::setup(configuration);

    let handler_channel = in_memory_async::message_channel(ChannelType::TOPIC, "Orders");
    in_memory_async::register(handler_channel, order_created_handler);

    in_memory_async::emit(&order_created);
    in_memory_async::emit(&order_canceled);
}
