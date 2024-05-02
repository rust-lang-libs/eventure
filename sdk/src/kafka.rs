use std::sync::Mutex;
use crate::model::{Event, EventHandler};

static HANDLER_REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());
static BROKER_CONFIGURATION: Mutex<MessageBrokerConfiguration> = Mutex::new(MessageBrokerConfiguration::new());

pub trait EventHandlerRegistry {
    fn register(&mut self, message_channel: MessageChannel, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event);
}

struct EventHandlerRegistryImpl {
    handlers: Vec<Box<dyn EventHandler + Send>>,
}

impl EventHandlerRegistryImpl {
    pub const fn new() -> Self {
        EventHandlerRegistryImpl { handlers: Vec::new() }
    }
}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, _message_channel: MessageChannel, event_handler: Box<dyn EventHandler + Send>) {
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

pub fn message_channel(channel_name: &'static str, partition: i32) -> MessageChannel {
    MessageChannel {
        name: channel_name,
        partition,
    }
}

pub fn configuration(channel_name: &'static str, partition: i32) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(channel_name, partition)
    }
}

pub struct MessageChannel {
    pub name: &'static str,
    pub partition: i32,
}

impl MessageChannel {
    pub const fn new() -> Self {
        MessageChannel {
            name: "*",
            partition: 1,
        }
    }

    pub fn update(&mut self, message_channel: MessageChannel) {
        self.name = message_channel.name;
    }
}

pub struct MessageBrokerConfiguration {
    message_channel: MessageChannel,
}

impl MessageBrokerConfiguration {
    pub const fn new() -> Self {
        MessageBrokerConfiguration {
            message_channel: MessageChannel::new()
        }
    }

    pub fn update(&mut self, configuration: MessageBrokerConfiguration) {
        self.message_channel = configuration.message_channel;
    }
}

pub fn setup(configuration: MessageBrokerConfiguration) {
    BROKER_CONFIGURATION.lock().unwrap().update(configuration);
}

pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
    HANDLER_REGISTRY.lock().unwrap().register(message_channel, Box::new(event_handler));
}

pub fn emit(event: &dyn Event) {
    HANDLER_REGISTRY.lock().unwrap().emit(event)
}
