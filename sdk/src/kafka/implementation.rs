// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

use crate::model::{Event, EventHandler};
use std::fmt::{Display, Formatter};
use log::info;

pub struct MessageChannel {
    pub topic: &'static str,
    pub partition: u16,
}

pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub topic_auto_create_enabled: bool,
    pub timeout: u16,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub fn message_channel(topic: &'static str, partition: u16) -> MessageChannel {
    MessageChannel {
        topic,
        partition,
    }
}

#[allow(dead_code)]
pub fn configuration(topic: &'static str, partition: u16) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel: message_channel(topic, partition),
        topic_auto_create_enabled: false,
        timeout: 10000,
    }
}

#[allow(dead_code)]
pub fn setup(configuration: MessageBrokerConfiguration) {
    info!(target: "MessageBrokerConfiguration", "setting up: {}",configuration);
    // TODO: implement
}

#[allow(dead_code)]
pub fn register(_message_channel: MessageChannel, _event_handler: impl EventHandler + Send + 'static) {
    // TODO: implement
}

pub fn unregister(_event_handler: impl EventHandler + Send + 'static) {
    // TODO: implement
}

#[allow(dead_code)]
pub fn emit(_event: &dyn Event) {
    // TODO: implement
}

#[allow(dead_code)]
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