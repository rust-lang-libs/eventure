// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::{process, thread};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use futures::future::{self, FutureExt};
use futures::StreamExt;
use log::info;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::AsyncRuntime;

use crate::common;
use crate::model::{Event, EventHandler};

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions

/// Kafka message channel definition.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let message_channel = kafka::MessageChannel {
///     topic: "Orders",
///     partition: 0,
///     group_id: "default"
/// };
/// ```
pub struct MessageChannel {
    pub topic: &'static str,
    pub partition: u16,
    pub group_id: &'static str
}

/// Kafka message broker configuration.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let message_channel = kafka::MessageChannel {
///     topic: "Orders",
///     partition: 0,
///     group_id: "consumer_group"
/// };
///
/// let configuration = kafka::MessageBrokerConfiguration {
///     message_channel,
///     bootstrap_servers: "localhost:9092",
///     topic_auto_create_enabled: false,
///     auto_commit_enabled: false,
///     timeout: 10000,
/// };
///
/// ```
pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub bootstrap_servers: &'static str,
    pub topic_auto_create_enabled: bool,
    pub auto_commit_enabled: bool,
    pub timeout: u32,
}

// -----------------------------------------------------------------------------------------------------------------------------------------

/// Creates Kafka message channel.
///
/// # Examples
/// ```
/// use eventure::kafka;
///
/// let handler_channel = kafka::message_channel("Orders", 0, "consumer_group");
/// ```
pub fn message_channel(topic: &'static str, partition: u16, group_id: &'static str) -> MessageChannel {
    MessageChannel {
        topic,
        partition,
        group_id
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
        message_channel: message_channel(topic, partition, "default"),
        bootstrap_servers: "localhost:9092",
        topic_auto_create_enabled: false,
        auto_commit_enabled: true,
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
/// let handler_channel = kafka::message_channel("orders", 0, "consumer_group");
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
/// #[typetag::serde]
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
pub fn register(message_channel: MessageChannel, event_handler: impl EventHandler + Send + 'static) {
    thread::spawn(move || {
        let configuration = BROKER_CONFIGURATION.lock().unwrap();
        smol::block_on(async {
            let topic = message_channel.topic;
            let consumer: StreamConsumer<_, SmolRuntime> = ClientConfig::new()
                .set("bootstrap.servers", configuration.bootstrap_servers)
                .set("session.timeout.ms", configuration.timeout.to_string())
                .set("enable.auto.commit", configuration.auto_commit_enabled.to_string())
                .set("group.id", message_channel.group_id)
                .set("auto.offset.reset", "earliest")
                .create().expect("Consumer creation failed");
            consumer.subscribe(&[&topic]).unwrap();

            drop(configuration);

            loop {
                let mut stream = consumer.stream();
                let message = stream.next().await;
                match message {
                    Some(Ok(message)) => {
                        let message_str = match message.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(_)) => "<invalid utf-8>",
                        };

                        let event: Box<dyn Event> = serde_json::from_str(message_str).unwrap();
                        event_handler.handle(&*event);
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message: {}", e);
                        process::exit(1);
                    }
                    None => {
                        eprintln!("Consumer unexpectedly returned no messages");
                        process::exit(1);
                    }
                }
            }
        });
    });
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
/// let handler_channel = kafka::message_channel("orders", 0, "consumer_group");
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
/// #[typetag::serde]
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
/// #[typetag::serde]
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
        let configuration = BROKER_CONFIGURATION.lock().unwrap();
        let topic = configuration.message_channel.topic;
        let producer: FutureProducer<_, SmolRuntime> = ClientConfig::new()
            .set("bootstrap.servers", configuration.bootstrap_servers)
            .set("message.timeout.ms", configuration.timeout.to_string())
            .create().expect("Producer creation error");

        drop(configuration);

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
/// #[typetag::serde]
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
/// kafka::emit_to_channel(&order_created, kafka::MessageChannel { topic: "Orders", partition: 0, group_id: "consumer_group" });
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
    pub group_id: &'static str
}

struct MessageBrokerConfigurationInternal {
    message_channel: MessageChannelInternal,
    bootstrap_servers: &'static str,
    topic_auto_create_enabled: bool,
    auto_commit_enabled: bool,
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
            group_id: "default"
        }
    }

    fn from(message_channel: MessageChannel) -> Self {
        MessageChannelInternal {
            topic: message_channel.topic,
            partition: message_channel.partition,
            group_id: message_channel.group_id
        }
    }
}

impl MessageBrokerConfigurationInternal {
    const fn new() -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::new(),
            bootstrap_servers: "localhost:9092",
            topic_auto_create_enabled: false,
            auto_commit_enabled: true,
            timeout: 0,
        }
    }

    fn from(configuration: MessageBrokerConfiguration) -> Self {
        MessageBrokerConfigurationInternal {
            message_channel: MessageChannelInternal::from(configuration.message_channel),
            bootstrap_servers: configuration.bootstrap_servers,
            topic_auto_create_enabled: configuration.topic_auto_create_enabled,
            auto_commit_enabled: configuration.auto_commit_enabled,
            timeout: configuration.timeout,
        }
    }

    fn update(&mut self, configuration: MessageBrokerConfigurationInternal) {
        self.message_channel = configuration.message_channel;
        self.bootstrap_servers = configuration.bootstrap_servers;
        self.topic_auto_create_enabled = configuration.topic_auto_create_enabled;
        self.timeout = configuration.timeout;
    }
}
