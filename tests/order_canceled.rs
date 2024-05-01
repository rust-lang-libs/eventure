use std::any::Any;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use eventure::model;

#[allow(dead_code)]
pub struct OrderCanceled {
    event_id: String,
    customer_id: String,
}

pub fn create_order_canceled() -> OrderCanceled {
    let customer_id = Uuid::new_v4().to_string();
    OrderCanceled::new(customer_id)
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

