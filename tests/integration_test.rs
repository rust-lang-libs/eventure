use std::fmt::{Display, Formatter};
use uuid::Uuid;
use eventure::model;
use eventure::inmemory_sync;

#[allow(dead_code)]
struct OrderCreated {
    event_id: String,
    customer_id: String,
}

#[allow(dead_code)]
struct OrderCanceled {
    event_id: String,
    customer_id: String,
}

impl OrderCreated {
    pub fn new(customer_id: String) -> OrderCreated {
        let event_id = Uuid::new_v4().to_string();
        OrderCreated { event_id, customer_id }
    }
}

impl Display for OrderCreated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderCreated with id {}", self.event_id)
    }
}

impl OrderCanceled {
    pub fn new(customer_id: String) -> OrderCanceled {
        let event_id = Uuid::new_v4().to_string();
        OrderCanceled { event_id, customer_id }
    }
}

impl Display for OrderCanceled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderCanceled with id {}", self.event_id)
    }
}

impl model::Event for OrderCreated {
    fn id(&self) -> &str {
        &self.event_id[..]
    }
    fn name(&self) -> &str {
        "OrderCreated"
    }
}

impl model::Event for OrderCanceled {
    fn id(&self) -> &str {
        &self.event_id[..]
    }
    fn name(&self) -> &str {
        "OrderCanceled"
    }
}

struct OrderEventHandler;

impl Display for OrderEventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderEventHandler")
    }
}

impl model::EventHandler for OrderEventHandler {
    fn handle(&self, event: &dyn model::Event) {
        println!("event handled: {}", event);
    }
}

fn create_order_created() -> OrderCreated {
    let customer_id = Uuid::new_v4().to_string();
    OrderCreated::new(customer_id)
}

fn create_order_canceled() -> OrderCanceled {
    let customer_id = Uuid::new_v4().to_string();
    OrderCanceled::new(customer_id)
}

#[test]
fn basic_scenario() {
    let order_created = create_order_created();
    let order_canceled = create_order_canceled();

    inmemory_sync::register(OrderEventHandler);
    inmemory_sync::emit(&order_created);
    inmemory_sync::emit(&order_canceled);
}
