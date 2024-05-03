// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use eventure::model;
use std::any::Any;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use colored::Colorize;
use log::info;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub struct OrderCreated {
    event_id: String,
    customer_id: String,
}

pub struct OrderCreatedEventHandler;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

pub fn create() -> OrderCreated {
    let customer_id = String::from(&Uuid::new_v4().to_string()[..6]);
    OrderCreated::new(customer_id)
}

pub fn handler() -> OrderCreatedEventHandler {
    OrderCreatedEventHandler
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl OrderCreated {
    fn new(customer_id: String) -> OrderCreated {
        let event_id = String::from(&Uuid::new_v4().to_string()[..6]);
        OrderCreated { event_id, customer_id }
    }
}

impl Display for OrderCreated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} event with id {}",
               "OrderCreated".bold().yellow().italic().underline(),
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
        write!(f, "{}", "OrderEventHandler".bold().yellow().italic().underline())
    }
}

impl model::EventHandler for OrderCreatedEventHandler {
    fn handle(&self, event: &(dyn model::Event + '_)) {
        match event.as_any().downcast_ref::<OrderCreated>() {
            Some(order_create) => self.handle(order_create),
            None => info!(target: "OrderCreatedEventHandler", "not handling {}", event)
        }
    }
}

impl OrderCreatedEventHandler {
    fn handle(&self, event: &OrderCreated) {
        info!(target: "OrderCreatedEventHandler", "handling {}", event)
    }
}
