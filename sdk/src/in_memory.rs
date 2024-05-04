// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::model::{Event, EventHandler};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use regex::Regex;
use log::{debug, info};

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Message channel definition.
///
/// # Examples
/// ```
/// use eventure::in_memory::{ChannelType, MessageChannel};
///
/// let message_channel = MessageChannel {
///         channel_type: ChannelType::TOPIC,
///         name: "Orders",
/// };
/// ```
pub struct MessageChannel {
    pub channel_type: ChannelType,
    pub name: &'static str,
}

/// Channel type
#[derive(Debug, PartialEq, Eq)]
pub enum ChannelType {
    TOPIC,
    QUEUE,
}

/// Message broken configuration.
///
/// # Examples
///
/// ```
/// use eventure::in_memory::{ChannelType, MessageBrokerConfiguration, MessageChannel};
///
/// let message_channel = MessageChannel {
///         channel_type: ChannelType::TOPIC,
///         name: "Orders",
/// };
///
/// let message_channel = MessageChannel {
///         channel_type: ChannelType::TOPIC,
///         name: "Orders",
/// };
///
/// let configuration = MessageBrokerConfiguration {
///     message_channel,
///     is_async: false,
/// };
/// ```
pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub is_async: bool,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Creates MessageChannel.
///
/// # Examples
/// ```
/// use eventure::in_memory;
/// use eventure::in_memory::ChannelType;
/// let handler_channel = in_memory::message_channel(ChannelType::TOPIC, "Order");
/// ```
pub fn message_channel(channel_type: ChannelType, channel_name: &'static str) -> MessageChannel {
    MessageChannel {
        channel_type,
        name: channel_name,
    }
}

/// Creates MessageBrokerConfiguration.
///
/// # Examples
///
/// ```
/// use eventure::in_memory;
/// use eventure::in_memory::ChannelType;
/// let configuration = in_memory::configuration(ChannelType::TOPIC, ".*", false);
/// ```
pub fn configuration(channel_type: ChannelType, channel_name: &'static str, is_async: bool) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(channel_type, channel_name),
        is_async,
    }
}

/// Sets up message broker configuration by passing MessageBrokerConfiguration instance.
///
///  # Examples
/// ```
/// use eventure::in_memory;
/// use eventure::in_memory::ChannelType;
/// let configuration = in_memory::configuration(ChannelType::TOPIC, ".*", false);
/// in_memory::setup(configuration);
/// ```
pub fn setup(configuration: MessageBrokerConfiguration) {
    info!(target: "MessageBrokerConfiguration", "setting up: {}",configuration);
    BROKER_CONFIGURATION.lock().unwrap().update(MessageBrokerConfigurationInternal::from(configuration));
}

/// Registers event handler.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use eventure::{in_memory, model};
/// use eventure::in_memory::ChannelType;
///
/// let handler_channel = in_memory::message_channel(ChannelType::TOPIC, "Order");
///
/// struct OrderCreatedEventHandler;
///
/// struct OrderCreated {
///     event_id: String,
///     customer_id: String,
/// }
///
/// impl Display for OrderCreated {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
///
/// impl model::Event for OrderCreated {
///     fn id(&self) -> &str {
///         &self.event_id[..]
///     }
///     fn name(&self) -> &str {
///         "OrderCreated"
///     }
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
///
/// let order_created = OrderCreated{
///     event_id: String::from("event_id"),
///     customer_id: String::from("customer_id"),
/// };
///
/// impl Display for OrderCreatedEventHandler {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", "OrderEventHandler")
///     }
/// }
///
/// impl model::EventHandler for OrderCreatedEventHandler {
///     fn handle(&self, event: &(dyn model::Event + '_)) {
///         match event.as_any().downcast_ref::<OrderCreated>() {
///             Some(order_create) => self.handle(order_create),
///             None => println!("{}: not handling {}", "OrderCreatedEventHandler", event)
///         }
///     }
///
///     fn id(&self) -> String {
///         todo!()
///     }
/// }
///
/// impl OrderCreatedEventHandler {
///     fn handle(&self, event: &OrderCreated) {
///         println!("{}: handling {}","OrderCreatedEventHandler", event)
///     }
/// }
///
/// let order_created_handler = OrderCreatedEventHandler;
/// in_memory::register(handler_channel, order_created_handler);
/// ```
pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
    HANDLER_REGISTRY.lock().unwrap().register(
        MessageChannelInternal::from(message_channel),
        Box::new(event_handler));
}

pub fn unregister(event_handler: impl EventHandler + Send + 'static) {
    HANDLER_REGISTRY.lock().unwrap().unregister(Box::new(event_handler));
}

/// Emits event without specifying message channel.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use eventure::{in_memory, model};
///
/// struct OrderCreated {
///     event_id: String,
///     customer_id: String,
/// }
///
/// impl Display for OrderCreated {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{} event with id {}",
///                "OrderCreated", self.event_id)
///     }
/// }
///
/// impl model::Event for OrderCreated {
///     fn id(&self) -> &str {
///         &self.event_id[..]
///     }
///     fn name(&self) -> &str {
///         "OrderCreated"
///     }
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
///
/// let order_created = OrderCreated{
///     event_id: String::from("event_id"),
///     customer_id: String::from("customer_id"),
/// };
///
/// in_memory::emit(&order_created);
/// ```
pub fn emit(event: &dyn Event) {
    HANDLER_REGISTRY.lock().unwrap().emit(event, None);
}

/// Emits event with specifying message channel.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use eventure::{in_memory, model};
/// use eventure::in_memory::ChannelType::QUEUE;
/// use eventure::in_memory::MessageChannel;
///
/// struct OrderCreated {
///     event_id: String,
///     customer_id: String,
/// }
///
/// impl Display for OrderCreated {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{} event with id {}",
///                "OrderCreated", self.event_id)
///     }
/// }
///
/// impl model::Event for OrderCreated {
///     fn id(&self) -> &str {
///         &self.event_id[..]
///     }
///     fn name(&self) -> &str {
///         "OrderCreated"
///     }
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
///
/// let order_created = OrderCreated{
///     event_id: String::from("event_id"),
///     customer_id: String::from("customer_id"),
/// };
/// in_memory::emit_to_channel(&order_created, MessageChannel { channel_type: QUEUE, name: ".*" });
/// ```
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
    fn unregister(&mut self, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event, channel: Option<MessageChannel>);
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl MessageChannelInternal {
    const fn new() -> Self {
        MessageChannelInternal {
            channel_type: ChannelType::TOPIC,
            name_regex: None,
        }
    }

    fn from(message_channel: MessageChannel) -> Self {
        MessageChannelInternal {
            channel_type: message_channel.channel_type,
            name_regex: Some(Regex::new(message_channel.name).unwrap()),
        }
    }

    fn matches(&self, channel: &MessageChannel) -> bool {
        match &self.name_regex {
            Some(regex) => self.channel_type == channel.channel_type && (regex.captures(channel.name).is_some()),
            None => false
        }
    }
}

impl MessageBrokerConfigurationInternal {
    const fn new() -> Self {
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
    const fn new() -> Self {
        EventHandlerRegistryImpl { handler_configs: Vec::new() }
    }
}

impl EventHandlerRegistry for EventHandlerRegistryImpl {
    fn register(&mut self, channel: MessageChannelInternal, handler: Box<dyn EventHandler + Send>) {
        info!(target: "EventHandlerRegistry", "in-memory event handler registered: {}",handler);
        self.handler_configs.push(HandlerConfiguration { handler, channel });
    }

    fn unregister(&mut self, event_handler: Box<dyn EventHandler + Send>) {
        let removed = self.handler_configs.iter()
            .position(|eh| *eh.handler.id() == event_handler.id())
            .map(|e| self.handler_configs.remove(e))
            .is_some();
        if removed {
            info!(target: "EventHandlerRegistry", "event handler unregistered: {}", event_handler);
        }
    }

    fn emit(&self, event: &dyn Event, channel_option: Option<MessageChannel>) {
        info!(target: "EventHandlerRegistry","in-memory event emitted: {}",event);
        match channel_option {
            Some(channel) =>
                for config in self.handler_configs.iter() {
                    if config.channel.matches(&channel) {
                        info!(target: "EventHandlerRegistry", "channel matched (handler: {}, channel: {})", config.handler, channel);
                        config.handler.handle(event);
                        if channel.channel_type == ChannelType::QUEUE {
                            debug!(target: "EventHandlerRegistry", "event handlers loop stopped for event {} in QUEUE", event);
                            break;
                        }
                    } else {
                        debug!(target: "EventHandlerRegistry", "channel not matched (handler: {}, channel: {})", config.handler, channel);
                    }
                }
            None =>
                for config in self.handler_configs.iter() {
                    info!(target: "EventHandlerRegistry", "not-specified channel matched by default (handler: {})", config.handler);
                    config.handler.handle(event);
                }
        }
    }
}

impl Display for MessageChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.channel_type, self.name)
    }
}

impl Display for MessageBrokerConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[default-channel:{},async:{}]", self.message_channel, self.is_async)
    }
}
