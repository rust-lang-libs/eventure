use std::any::Any;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use eventure::model;

#[allow(dead_code)]
pub struct OrderCreated {
    event_id: String,
    customer_id: String,
}

pub fn create() -> OrderCreated {
    let customer_id = Uuid::new_v4().to_string();
    OrderCreated::new(customer_id)
}

pub struct OrderCreatedEventHandler;

pub fn handler() -> OrderCreatedEventHandler {
    OrderCreatedEventHandler
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

impl model::Event for OrderCreated {
    fn id(&self) -> &str {
        &self.event_id[..]
    }
    fn name(&self) -> &str {
        "OrderCreated"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for OrderCreatedEventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderEventHandler")
    }
}

impl model::EventHandler for OrderCreatedEventHandler {
    fn handle(&self, event: &(dyn model::Event + '_)) {
        match event.as_any().downcast_ref::<OrderCreated>() {
            Some(order_create) => self.handle(order_create),
            None => println!("skipping...")
        }
    }
}

impl OrderCreatedEventHandler {
    fn handle(&self, event: &OrderCreated) {
        println!("handling {}", event)
    }
}
