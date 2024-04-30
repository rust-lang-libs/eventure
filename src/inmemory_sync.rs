use std::sync::Mutex;
use crate::model::{Event, EventHandler, EventHandlerRegistry};

static EVENT_HANDLER_REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());

struct EventHandlerRegistryImpl {
    handlers: Vec<Box<dyn EventHandler + Send>>,
}

impl EventHandlerRegistryImpl {
    pub const fn new() -> Self {
        EventHandlerRegistryImpl { handlers: Vec::new() }
    }
}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, event_handler: Box<dyn EventHandler + Send>) {
        println!("Event handler registered: {}", event_handler);
        self.handlers.push(event_handler);
    }

    fn emit(&self, event: &dyn Event) {
        println!("Event emitted: {}", event);
        for handler in self.handlers.iter() {
            handler.handle(event);
        }
    }
}

pub fn register(event_handler: impl EventHandler + Send + 'static) {
    EVENT_HANDLER_REGISTRY.lock().unwrap().register(Box::new(event_handler));
}

pub fn emit(event: &dyn Event) {
    EVENT_HANDLER_REGISTRY.lock().unwrap().emit(event)
}
