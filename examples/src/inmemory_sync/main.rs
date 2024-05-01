use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::inmemory_sync;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    inmemory_sync::register(order_created_handler);
    inmemory_sync::emit(&order_created);
    inmemory_sync::emit(&order_canceled);
}
