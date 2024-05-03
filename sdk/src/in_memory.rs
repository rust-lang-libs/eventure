use std::sync::Mutex;
use crate::model::{Event, EventHandler};

static HANDLER_REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());
static BROKER_CONFIGURATION: Mutex<MessageBrokerConfiguration> = Mutex::new(MessageBrokerConfiguration::new());

pub trait EventHandlerRegistry {
    fn register(&mut self, message_channel: MessageChannel, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event, channel: Option<MessageChannel>);
}

pub fn message_channel(channel_type: ChannelType, channel_name: &'static str) -> MessageChannel {
    MessageChannel {
        channel_type,
        name: channel_name,
    }
}

pub fn configuration(channel_type: ChannelType, channel_name: &'static str, is_async: bool) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(channel_type, channel_name),
        is_async,
    }
}

pub struct MessageChannel {
    pub channel_type: ChannelType,
    pub name: &'static str,
}

#[derive(PartialEq, Eq)]
pub enum ChannelType {
    TOPIC,
    QUEUE,
}

pub struct MessageBrokerConfiguration {
    message_channel: MessageChannel,
    is_async: bool,
}

pub fn setup(configuration: MessageBrokerConfiguration) {
    BROKER_CONFIGURATION.lock().unwrap().update(configuration);
}

pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
    HANDLER_REGISTRY.lock().unwrap().register(message_channel, Box::new(event_handler));
}

pub fn emit(event: &dyn Event) {
    HANDLER_REGISTRY.lock().unwrap().emit(event, None);
}

pub fn emit_to_channel(event: &dyn Event, channel: MessageChannel) {
    HANDLER_REGISTRY.lock().unwrap().emit(event, Some(channel));
}

struct EventHandlerRegistryImpl {
    handler_configs: Vec<HandlerConfiguration>,
}

impl EventHandlerRegistryImpl {
    pub const fn new() -> Self {
        EventHandlerRegistryImpl { handler_configs: Vec::new() }
    }
}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, channel: MessageChannel, handler: Box<dyn EventHandler + Send>) {
        println!("In-memory event handler registered: {}", handler);
        self.handler_configs.push(HandlerConfiguration { handler, channel });
    }

    fn emit(&self, event: &dyn Event, channel: Option<MessageChannel>) {
        println!("In-memory event emitted: {}", event);

        match channel {
            Some(channel) =>
                for config in self.handler_configs.iter() {
                    if config.channel.matches(&channel) {
                        println!("Channel matched");
                        config.handler.handle(event);
                    } else {
                        println!("Channel not matched");
                    }
                }
            None =>
                for config in self.handler_configs.iter() {
                    println!("Channel matched");
                    config.handler.handle(event);
                }
        }
    }
}

impl MessageChannel {
    pub const fn new() -> Self {
        MessageChannel {
            channel_type: ChannelType::TOPIC,
            name: "*",
        }
    }

    pub fn update(&mut self, message_channel: MessageChannel) {
        self.channel_type = message_channel.channel_type;
        self.name = message_channel.name;
    }

    pub fn matches(&self, channel: &MessageChannel) -> bool {
        self.channel_type == channel.channel_type
            && (self.name == "*" || channel.name == "*" || self.name == channel.name)
    }
}

impl MessageBrokerConfiguration {
    pub const fn new() -> Self {
        MessageBrokerConfiguration {
            message_channel: MessageChannel::new(),
            is_async: false,
        }
    }

    pub fn update(&mut self, configuration: MessageBrokerConfiguration) {
        self.message_channel = configuration.message_channel;
        self.is_async = configuration.is_async;
    }
}

struct HandlerConfiguration {
    handler: Box<dyn EventHandler + Send>,
    channel: MessageChannel,
}
