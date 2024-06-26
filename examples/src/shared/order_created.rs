// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::any::Any;
use std::fmt::{Display, Formatter};
use log::info;
use serde::{Deserialize, Serialize};
use crate::shared::common;
use eventure::model;
use eventure::model::Event;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct OrderCreated {
    event_id: String,
    customer_id: String,
}

pub struct OrderCreatedEventHandler {
    id: String,
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
        id: String::from("OrderCreatedEventHandler-") + &common::generate_id()
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

#[typetag::serde]
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
    fn to_json(&self) -> String {
        let event = self as &dyn Event;
        serde_json::to_string(&event).unwrap()
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
        info!(target: &self.id, "handling {}", event)
    }
}
