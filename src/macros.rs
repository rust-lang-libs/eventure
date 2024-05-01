#[macro_export]
macro_rules! create_registry_backbone {
    () => {
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

        pub fn register(event_handler: impl EventHandler + Send + 'static) {
            EVENT_HANDLER_REGISTRY.lock().unwrap().register(Box::new(event_handler));
        }

        pub fn emit(event: &dyn Event) {
            EVENT_HANDLER_REGISTRY.lock().unwrap().emit(event)
        }
    };
}
