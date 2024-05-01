use crate::{create_registry_backbone};

create_registry_backbone!();

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, _message_channel: MessageChannel, event_handler: Box<dyn EventHandler + Send>) {
        println!("Sync in-memory event handler registered: {}", event_handler);
        self.handlers.push(event_handler);
    }

    fn emit(&self, event: &dyn Event) {
        println!("Sync event emitted: {}", event);
        for handler in self.handlers.iter() {
            handler.handle(event);
        }
    }
}

