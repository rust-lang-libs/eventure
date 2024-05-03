use std::any::Any;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use eventure::model;
use colored::Colorize;

#[allow(dead_code)]
pub struct OrderCanceled {
    event_id: String,
    customer_id: String,
}

pub fn create() -> OrderCanceled {
    let customer_id = String::from(&Uuid::new_v4().to_string()[..6]);
    OrderCanceled::new(customer_id)
}

pub struct OrderCanceledEventHandler;

pub fn handler() -> OrderCanceledEventHandler {
    OrderCanceledEventHandler
}

impl OrderCanceled {
    pub fn new(customer_id: String) -> OrderCanceled {
        let event_id = String::from(&Uuid::new_v4().to_string()[..6]);
        OrderCanceled { event_id, customer_id }
    }
}

impl Display for OrderCanceled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} event with id {}",
               "OrderCanceled".bold().purple(),
               self.event_id)
    }
}

impl model::Event for OrderCanceled {
    fn id(&self) -> &str {
        &self.event_id[..]
    }
    fn name(&self) -> &str {
        "OrderCanceled"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

