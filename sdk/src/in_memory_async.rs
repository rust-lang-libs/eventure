use crate::{create_registry_backbone};

create_registry_backbone!();

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, event_handler: Box<dyn EventHandler + Send>) {
        println!("Async in-memory event handler registered: {}", event_handler);
        self.handlers.push(event_handler);
    }

    fn emit(&self, event: &dyn Event) {
        println!("Async event emitted: {}", event);
        for handler in self.handlers.iter() {
            handler.handle(event);
        }
    }
}
