use uuid::Uuid;
use eventure::model;
use eventure::inmemory_sync;

#[derive(Debug)]
struct OrderCreated {
    event_id: String,
    customer_id: String,
}

impl OrderCreated {
    pub fn new(customer_id: String) -> OrderCreated {
        let event_id = Uuid::new_v4().to_string();
        OrderCreated { event_id, customer_id }
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

struct OrderEventHandler;

impl model::EventHandler for OrderEventHandler {
    fn handle(&self, _event: &dyn model::Event) {
        println!("event handled");
    }
}

fn main() {
    let customer_id = Uuid::new_v4().to_string();
    let order_created = OrderCreated::new(customer_id);
    let event_handler = OrderEventHandler;

    inmemory_sync::register(event_handler);
    inmemory_sync::emit(&order_created);
}