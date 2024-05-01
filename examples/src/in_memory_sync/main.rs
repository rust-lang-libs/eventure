use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::in_memory_sync;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    in_memory_sync::register(order_created_handler);
    in_memory_sync::emit(&order_created);
    in_memory_sync::emit(&order_canceled);
}