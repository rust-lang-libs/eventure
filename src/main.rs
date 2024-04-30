use std::fmt::{Display, Formatter};
use uuid::Uuid;
use eventure::model;
use eventure::inmemory_sync;
use eventure::model::Event;

struct OrderCreated {
    event_id: String,
    customer_id: String,
}

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
        &self.customer_id[..]
    }
}

impl model::Event for OrderCanceled {
    fn id(&self) -> &str {
        &self.event_id[..]
    }
    fn name(&self) -> &str {
        &self.customer_id[..]
    }
}

struct OrderEventHandler;

impl Display for OrderEventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderEventHandler")
    }
}

impl model::EventHandler for OrderEventHandler {
    fn handle(&self, event: &dyn Event) {
        println!("event handled: {}", event);
    }
}

fn main() {
    let customer_id = Uuid::new_v4().to_string();
    let order_created = OrderCreated::new(customer_id);
    let customer_id = Uuid::new_v4().to_string();
    let order_canceled = OrderCanceled::new(customer_id);
    let event_handler = OrderEventHandler;

    inmemory_sync::register(event_handler);
    inmemory_sync::emit(&order_created);
    inmemory_sync::emit(&order_canceled);
}