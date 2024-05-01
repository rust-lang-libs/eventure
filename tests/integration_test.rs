use eventure::{inmemory_async, inmemory_sync};

mod order_created;
mod order_canceled;

#[test]
fn basic_scenario() {
    let order_created = order_created::create_order_created();
    let order_canceled = order_canceled::create_order_canceled();

    inmemory_sync::register(order_created::OrderEventHandler);
    inmemory_sync::emit(&order_created);
    inmemory_sync::emit(&order_canceled);

    inmemory_async::register(order_created::OrderEventHandler);
    inmemory_async::emit(&order_created);
    inmemory_async::emit(&order_canceled);
}
