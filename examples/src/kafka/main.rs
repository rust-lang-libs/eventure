use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::kafka;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    kafka::register(order_created_handler);
    kafka::emit(&order_created);
    kafka::emit(&order_canceled);
}
