// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::any::Any;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use eventure::model;
use crate::shared::common;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct OrderCanceled {
    event_id: String,
    customer_id: String,
}

pub struct OrderCanceledEventHandler;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

pub fn create() -> OrderCanceled {
    let customer_id = common::generate_id();
    OrderCanceled::new(customer_id)
}

pub fn handler() -> OrderCanceledEventHandler {
    OrderCanceledEventHandler
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl OrderCanceled {
    fn new(customer_id: String) -> OrderCanceled {
        let event_id = common::generate_id();
        OrderCanceled { event_id, customer_id }
    }
}

impl Display for OrderCanceled {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} event, id {}", common::color_string("OrderCanceled"), self.event_id)
    }
}

#[typetag::serde]
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
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
