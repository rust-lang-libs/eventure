// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::model::{Event, EventHandler};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::process;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use log::info;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::AsyncRuntime;
use futures::future::{self, FutureExt};
use crate::common;

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
    pub timeout: u32,
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
    info!(target: &common::format_target("MessageBrokerConfiguration"), "setting up: {}",configuration);
    BROKER_CONFIGURATION.lock().unwrap().update(MessageBrokerConfigurationInternal::from(configuration));
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
///         serde_json::to_string(&self).unwrap()
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
pub fn emit(event: &dyn Event) {
    smol::block_on(async {
        let topic = BROKER_CONFIGURATION.lock().unwrap().message_channel.topic;
        let producer: FutureProducer<_, SmolRuntime> = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create().expect("Producer creation error");

        let delivery_status = producer
            .send::<Vec<u8>, _, _>(
                FutureRecord::to(topic).payload(&event.to_json()),
                Duration::from_secs(0),
            )
            .await;
        if let Err((e, _)) = delivery_status {
            eprintln!("unable to send message: {}", e);
            process::exit(1);
        }

        info!(target: &common::format_target("KafkaEmitter"), "event {} sent to the topic: {}", event, topic);
    })
}

pub struct SmolRuntime;

impl AsyncRuntime for SmolRuntime {
    type Delay = future::Map<smol::Timer, fn(Instant)>;

    fn spawn<T>(task: T)
        where
            T: Future<Output=()> + Send + 'static,
    {
        smol::spawn(task).detach()
    }

    fn delay_for(duration: Duration) -> Self::Delay {
        FutureExt::map(smol::Timer::after(duration), |_| ())
    }
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
// Private statics
// -----------------------------------------------------------------------------------------------------------------------------------------

static BROKER_CONFIGURATION: Mutex<MessageBrokerConfigurationInternal> = Mutex::new(MessageBrokerConfigurationInternal::new());

// -----------------------------------------------------------------------------------------------------------------------------------------
// Private structs
// -----------------------------------------------------------------------------------------------------------------------------------------

pub struct MessageChannelInternal {
    pub topic: &'static str,
    pub partition: u16,
}

struct MessageBrokerConfigurationInternal {
    message_channel: MessageChannelInternal,
    topic_auto_create_enabled: bool,
    timeout: u32,
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

impl MessageChannelInternal {
    const fn new() -> Self {
        MessageChannelInternal {
            topic: "default",
            partition: 0,
        }
    }

    fn from(message_channel: MessageChannel) -> Self {
        MessageChannelInternal {
            topic: message_channel.topic,
            partition: message_channel.partition,
        }
    }
}

impl MessageBrokerConfigurationInternal {
    const fn new() -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::new(),
            topic_auto_create_enabled: false,
            timeout: 0,
        }
    }

    fn from(configuration: MessageBrokerConfiguration) -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::from(configuration.message_channel),
            topic_auto_create_enabled: configuration.topic_auto_create_enabled,
            timeout: configuration.timeout,
        }
    }

    fn update(&mut self, configuration: MessageBrokerConfigurationInternal) {
        self.message_channel = configuration.message_channel;
        self.topic_auto_create_enabled = configuration.topic_auto_create_enabled;
        self.timeout = configuration.timeout;
    }
}
