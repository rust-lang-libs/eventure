use examples::shared::order_created;
use examples::shared::order_canceled;
use eventure::in_memory;
use eventure::in_memory::{MessageChannel};
use eventure::in_memory::ChannelType::{QUEUE, TOPIC};

fn main() {
    println!();

    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let configuration = in_memory::configuration(TOPIC, ".*", false);
    in_memory::setup(configuration);

    let handler_channel = in_memory::message_channel(TOPIC, "Order");
    in_memory::register(handler_channel, order_created_handler);

    in_memory::emit(&order_created);
    in_memory::emit(&order_canceled);
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: QUEUE, name: ".*" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "Account.*" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "*" });
    in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: TOPIC, name: "Orders" });

    println!();
}
