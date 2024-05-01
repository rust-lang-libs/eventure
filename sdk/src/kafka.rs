use crate::{create_registry_backbone};

create_registry_backbone!();

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, event_handler: Box<dyn EventHandler + Send>) {
        println!("Kafka event handler registered: {}", event_handler);
        self.handlers.push(event_handler);
    }

    fn emit(&self, event: &dyn Event) {
        println!("Kafka event emitted: {}", event);
        for handler in self.handlers.iter() {
            handler.handle(event);
        }
    }
}
