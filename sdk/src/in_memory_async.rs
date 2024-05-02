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

pub fn message_channel(channel_type: ChannelType, channel_name: &'static str) -> MessageChannel {
    MessageChannel {
        channel_type,
        name: channel_name,
    }
}

pub fn configuration(channel_type: ChannelType, channel_name: &'static str, durable: bool) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(channel_type, channel_name),
        durable,
    }
}

pub struct MessageChannel {
    pub channel_type: ChannelType,
    pub name: &'static str,
}

pub enum ChannelType {
    TOPIC,
    QUEUE,
}

impl MessageChannel {
    pub const fn new() -> Self {
        MessageChannel {
            channel_type: ChannelType::TOPIC,
            name: "*"
        }
    }

    pub fn update(&mut self, message_channel: MessageChannel) {
        self.channel_type = message_channel.channel_type;
        self.name = message_channel.name;
    }
}

pub struct MessageBrokerConfiguration {
    message_channel: MessageChannel,
    durable: bool,
}

impl MessageBrokerConfiguration {
    pub const fn new() -> Self {
        MessageBrokerConfiguration {
            message_channel: MessageChannel::new(),
            durable: false,
        }
    }

    pub fn update(&mut self, configuration: MessageBrokerConfiguration) {
        self.message_channel = configuration.message_channel;
        self.durable = configuration.durable;
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
