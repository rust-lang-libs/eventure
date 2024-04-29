use std::sync::Mutex;
use crate::model::{Event, EventHandler, EventHandlerRegistry};

struct EventHandlerRegistryImpl {
    handlers: Vec<Box<dyn EventHandler + Send>>,
}

static REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());

impl EventHandlerRegistryImpl {
    pub const fn new() -> Self {
        EventHandlerRegistryImpl { handlers: Vec::new() }
    }
}

unsafe impl Sync for EventHandlerRegistryImpl {}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, _event_handler: Box<dyn EventHandler + Send>) {
        println!("event handler registered");
        self.handlers.push(_event_handler);
    }

    fn emit(&self, _event: &dyn Event) {
        println!("event emitted");
        for handler in self.handlers.iter() {
            handler.handle(_event);
        }
    }
}

pub fn register(_event_handler: impl EventHandler + Send + 'static) {
    REGISTRY.lock().unwrap().register(Box::new(_event_handler));
}

pub fn emit(_event:&dyn Event) {
    REGISTRY.lock().unwrap().emit(_event)
}
