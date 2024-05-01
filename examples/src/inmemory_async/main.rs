use eventure::inmemory_async;
use eventure_examples::{order_canceled, order_created};

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();

    inmemory_async::register(order_created::handler());
    inmemory_async::emit(&order_created);
    inmemory_async::emit(&order_canceled);
}
