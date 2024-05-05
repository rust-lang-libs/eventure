// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::model::{Event, EventHandler};
use std::fmt::{Display, Formatter};
use log::info;

/// Kafka message channel definition.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let message_channel = kafka::MessageChannel {
///     topic: "Orders",
///     partition: 0
/// };
/// ```
pub struct MessageChannel {
    pub topic: &'static str,
    pub partition: u16,
}

/// Kafka message broker configuration.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let message_channel = kafka::MessageChannel {
///     topic: "Orders",
///     partition: 0
/// };
///
/// let configuration = kafka::MessageBrokerConfiguration {
///     message_channel,
///     topic_auto_create_enabled: false,
///     timeout: 10000,
/// };
///
/// ```
pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub topic_auto_create_enabled: bool,
    pub timeout: u16,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Creates Kafka message channel.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let handler_channel = kafka::message_channel("Orders", 0);
/// ```
pub fn message_channel(topic: &'static str, partition: u16) -> MessageChannel {
    MessageChannel {
        topic,
        partition,
    }
}

/// Creates Kafka message broker configuration.
///
/// # Examples
///
/// ```
/// use eventure::kafka;
///
/// let configuration = kafka::configuration("Orders", 0);
/// ```
pub fn configuration(topic: &'static str, partition: u16) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(topic, partition),
        topic_auto_create_enabled: false,
        timeout: 10000,
    }
}

/// Sets up Kafka message broker configuration by passing MessageBrokerConfiguration instance.
///
///  # Examples
/// ```
/// use eventure::kafka;
///
/// let configuration = kafka::configuration("Orders", 0);
/// kafka::setup(configuration);
/// ```
pub fn setup(configuration: MessageBrokerConfiguration) {
    info!(target: "MessageBrokerConfiguration", "setting up: {}",configuration);
    // TODO: implement
}

/// Registers Kafka event handler.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use serde::{Deserialize, Serialize};
/// use eventure::{kafka, model};
///
/// let handler_channel = kafka::message_channel("Orders", 0);
///
/// struct OrderCreatedEventHandler;
///
/// #[derive(Serialize, Deserialize)]
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
///     fn to_json(&self) -> String {
///         todo!()
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
///         String::from("OrderCreatedEventHandler")
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
/// kafka::register(handler_channel, order_created_handler);
/// ```
pub fn register(_message_channel: MessageChannel, _event_handler: impl EventHandler + Send + 'static) {
    // TODO: implement
}

/// Unregisters Kafka event handler.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use serde::{Deserialize, Serialize};
/// use eventure::{kafka, model};
///
/// let handler_channel = kafka::message_channel("Orders", 0);
///
/// struct OrderCreatedEventHandler;
///
/// #[derive(Serialize, Deserialize)]
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
///     fn to_json(&self) -> String {
///         todo!()
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
///         String::from("OrderCreatedEventHandler")
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
/// kafka::register(handler_channel, order_created_handler);
///
/// let order_created_handler = OrderCreatedEventHandler;
/// kafka::unregister(order_created_handler);
/// ```
pub fn unregister(_event_handler: impl EventHandler + Send + 'static) {
    // TODO: implement
}

/// Emits Kafka event without specifying message channel.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use serde::{Deserialize, Serialize};
/// use eventure::{kafka, model};
///
/// #[derive(Serialize, Deserialize)]
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
///     fn to_json(&self) -> String {
///         todo!()
///     }
/// }
///
/// let order_created = OrderCreated{
///     event_id: String::from("event_id"),
///     customer_id: String::from("customer_id"),
/// };
///
/// kafka::emit(&order_created);
/// ```
pub fn emit(_event: &dyn Event) {
    // TODO: implement
}

/// Emits Kafka event to specific message channel.
///
/// # Examples
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use serde::{Deserialize, Serialize};
/// use eventure::{kafka, model};
///
/// #[derive(Serialize, Deserialize)]
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
///     fn to_json(&self) -> String {
///         todo!()
///     }
/// }
///
/// let order_created = OrderCreated{
///     event_id: String::from("event_id"),
///     customer_id: String::from("customer_id"),
/// };
/// kafka::emit_to_channel(&order_created, kafka::MessageChannel { topic: "Orders", partition: 0 });
/// ```
pub fn emit_to_channel(_event: &dyn Event, _channel: MessageChannel) {
    // TODO: implement
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------------------------------------------------------------------

impl Display for MessageChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.topic, self.partition)
    }
}

impl Display for MessageBrokerConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[default-channel:{},topic-auto-create:{},timeout:{}]",
               self.message_channel, self.topic_auto_create_enabled, self.timeout)
    }
}
