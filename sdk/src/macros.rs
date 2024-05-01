#[macro_export]
macro_rules! create_registry_backbone {
    () => {
        use std::sync::Mutex;
        use crate::model::{Event, EventHandler, EventHandlerRegistry, MessageChannel, ChannelType};

        static EVENT_HANDLER_REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());

        struct EventHandlerRegistryImpl {
            handlers: Vec<Box<dyn EventHandler + Send>>,
        }

        impl EventHandlerRegistryImpl {
            pub const fn new() -> Self {
                EventHandlerRegistryImpl { handlers: Vec::new() }
            }
        }

        pub fn message_channel(channel_type: ChannelType, channel_name: String) -> MessageChannel {
            MessageChannel{
                channel_type: channel_type,
                name : channel_name,
            }
        }

        pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
            EVENT_HANDLER_REGISTRY.lock().unwrap().register(message_channel, Box::new(event_handler));
        }

        pub fn emit(event: &dyn Event) {
            EVENT_HANDLER_REGISTRY.lock().unwrap().emit(event)
        }
    };
}
