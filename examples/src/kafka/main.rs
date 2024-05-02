use examples::shared::order_created;
use examples::shared::order_canceled;
use eventure::kafka;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    let configuration = kafka::configuration("*", 1);
    kafka::setup(configuration);

    let handler_channel = kafka::message_channel("Orders", 1);
    kafka::register(handler_channel, order_created_handler);

    kafka::emit(&order_created);
    kafka::emit(&order_canceled);
}
