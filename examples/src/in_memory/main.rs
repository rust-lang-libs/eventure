use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory;
use eventure::in_memory::ChannelType;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let configuration = in_memory::configuration(ChannelType::TOPIC, "*", false);
    in_memory::setup(configuration);

    let handler_channel = in_memory::message_channel(ChannelType::TOPIC, "Orders");
    in_memory::register(handler_channel, order_created_handler);

    in_memory::emit(&order_created);
    in_memory::emit(&order_canceled);
}
