// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::model::{Event, EventHandler};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use regex::Regex;
use colored::Colorize;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

pub struct MessageChannel {
    pub channel_type: ChannelType,
    pub name: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChannelType {
    TOPIC,
    QUEUE,
}

pub struct MessageBrokerConfiguration {
    message_channel: MessageChannel,
    is_async: bool,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

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

pub fn setup(configuration: MessageBrokerConfiguration) {
    BROKER_CONFIGURATION.lock().unwrap().update(MessageBrokerConfigurationInternal::from(configuration));
}

pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
    HANDLER_REGISTRY.lock().unwrap().register(
        MessageChannelInternal::from(message_channel),
        Box::new(event_handler));
}

pub fn emit(event: &dyn Event) {
    HANDLER_REGISTRY.lock().unwrap().emit(event, None);
}

pub fn emit_to_channel(event: &dyn Event, channel: MessageChannel) {
    HANDLER_REGISTRY.lock().unwrap().emit(event, Some(channel));
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Private statics
// -----------------------------------------------------------------------------------------------------------------------------------------

static HANDLER_REGISTRY: Mutex<EventHandlerRegistryImpl> = Mutex::new(EventHandlerRegistryImpl::new());
static BROKER_CONFIGURATION: Mutex<MessageBrokerConfigurationInternal> = Mutex::new(MessageBrokerConfigurationInternal::new());

// -----------------------------------------------------------------------------------------------------------------------------------------
// Private structs
// -----------------------------------------------------------------------------------------------------------------------------------------

struct MessageChannelInternal {
    channel_type: ChannelType,
    name_regex: Option<Regex>,
}

struct MessageBrokerConfigurationInternal {
    message_channel: MessageChannelInternal,
    is_async: bool,
}

struct EventHandlerRegistryImpl {
    handler_configs: Vec<HandlerConfiguration>,
}

struct HandlerConfiguration {
    handler: Box<dyn EventHandler + Send>,
    channel: MessageChannelInternal,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Private traits
// -----------------------------------------------------------------------------------------------------------------------------------------

trait EventHandlerRegistry {
    fn register(&mut self, message_channel: MessageChannelInternal, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event, channel: Option<MessageChannel>);
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl MessageChannelInternal {
    pub const fn new() -> Self {
        MessageChannelInternal {
            channel_type: ChannelType::TOPIC,
            name_regex: None,
        }
    }

    pub fn from(message_channel: MessageChannel) -> Self {
        MessageChannelInternal {
            channel_type: message_channel.channel_type,
            name_regex: Some(Regex::new(message_channel.name).unwrap()),
        }
    }

    pub fn matches(&self, channel: &MessageChannel) -> bool {
        match &self.name_regex {
            Some(regex) => self.channel_type == channel.channel_type && (regex.captures(channel.name).is_some()),
            None => false
        }
    }
}

impl MessageBrokerConfigurationInternal {
    pub const fn new() -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::new(),
            is_async: false,
        }
    }

    fn from(configuration: MessageBrokerConfiguration) -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::from(configuration.message_channel),
            is_async: configuration.is_async,
        }
    }

    fn update(&mut self, configuration: MessageBrokerConfigurationInternal) {
        self.message_channel = configuration.message_channel;
        self.is_async = configuration.is_async;
    }
}

impl EventHandlerRegistryImpl {
    pub const fn new() -> Self {
        EventHandlerRegistryImpl { handler_configs: Vec::new() }
    }
}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, channel: MessageChannelInternal, handler: Box<dyn EventHandler + Send>) {
        println!("{}: in-memory event handler registered: {}",
                 "EventHandlerRegistry".bold().green(),
                 handler);
        self.handler_configs.push(HandlerConfiguration { handler, channel });
    }

    fn emit(&self, event: &dyn Event, channel_option: Option<MessageChannel>) {
        println!("{}: in-memory event emitted: {}",
                 "EventHandlerRegistry".bold().green(),
                 event);

        match channel_option {
            Some(channel) =>
                for config in self.handler_configs.iter() {
                    if config.channel.matches(&channel) {
                        println!("{}: channel matched (handler: {}, channel: {})",
                                 "EventHandlerRegistry".bold().green(),
                                 config.handler,
                                 channel);
                        config.handler.handle(event);
                    } else {
                        println!("{}: channel not matched (handler: {}, channel: {})",
                                 "EventHandlerRegistry".bold().green(),
                                 config.handler,
                                 channel);
                    }
                }
            None =>
                for config in self.handler_configs.iter() {
                    println!("{}: not-specified channel matched by default (handler: {})",
                             "EventHandlerRegistry".bold().green(),
                             config.handler);
                    config.handler.handle(event);
                }
        }
    }
}

impl MessageChannel {
    pub const fn new() -> Self {
        MessageChannel {
            channel_type: ChannelType::TOPIC,
            name: ".*",
        }
    }

    pub fn update(&mut self, message_channel: MessageChannel) {
        self.channel_type = message_channel.channel_type;
        self.name = message_channel.name;
    }
}

impl Display for MessageChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.channel_type, self.name)
    }
}

impl MessageBrokerConfiguration {
    pub const fn new() -> Self {
        MessageBrokerConfiguration {
            message_channel: MessageChannel::new(),
            is_async: false,
        }
    }
}

