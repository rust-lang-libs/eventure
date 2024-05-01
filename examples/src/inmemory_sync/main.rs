use eventure::inmemory_sync;
use eventure_examples::{order_canceled, order_created};

fn main() {
    let order_created = order_created::create();
    let order_canceled = order_canceled::create();

    inmemory_sync::register(order_created::handler());
    inmemory_sync::emit(&order_created);
    inmemory_sync::emit(&order_canceled);
}
