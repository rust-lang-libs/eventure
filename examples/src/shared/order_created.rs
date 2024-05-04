// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::shared::common;
use eventure::model;
use std::any::Any;
use std::fmt::{Display, Formatter};
use log::info;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub struct OrderCreated {
    event_id: String,
    customer_id: String,
}

pub struct OrderCreatedEventHandler {
    id: String
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

pub fn create() -> OrderCreated {
    let customer_id = common::generate_id();
    OrderCreated::new(customer_id)
}

pub fn handler() -> OrderCreatedEventHandler {
    OrderCreatedEventHandler {
        id: String::from("OrderCreatedEventHandler")
    }
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl OrderCreated {
    fn new(customer_id: String) -> OrderCreated {
        let event_id = common::generate_id();
        OrderCreated { event_id, customer_id }
    }
}

impl Display for OrderCreated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} event, id {}",
               common::color_string("OrderCreated"),
               self.event_id)
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
        write!(f, "{}", common::color_string("OrderEventHandler"))
    }
}

impl model::EventHandler for OrderCreatedEventHandler {
    fn handle(&self, event: &(dyn model::Event + '_)) {
        match event.as_any().downcast_ref::<OrderCreated>() {
            Some(order_create) => self.handle(order_create),
            None => info!(target: "OrderCreatedEventHandler", "not handling (type mismatch) event: {}", event)
        }
    }

    fn id(&self) -> String {
        String::from(&self.id)
    }
}

impl OrderCreatedEventHandler {
    fn handle(&self, event: &OrderCreated) {
        info!(target: "OrderCreatedEventHandler", "handling {}", event)
    }
}
