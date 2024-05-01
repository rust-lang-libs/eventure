use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_async;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    in_memory_async::register(order_created_handler);
    in_memory_async::emit(&order_created);
    in_memory_async::emit(&order_canceled);
}
