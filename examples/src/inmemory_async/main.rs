use eventure_examples::shared::order_created;
use eventure_examples::shared::order_canceled;
use eventure::inmemory_async;

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();
    let order_created_handler = order_created::handler();

    inmemory_async::register(order_created_handler);
    inmemory_async::emit(&order_created);
    inmemory_async::emit(&order_canceled);
}
