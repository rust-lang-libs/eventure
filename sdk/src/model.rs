use std::any::Any;
use std::fmt::Display;

pub trait Event: Display {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub trait EventHandler: Display {
    fn handle(&self, event: &dyn Event);
}

pub trait EventHandlerRegistry {
    fn register(&mut self, message_channel: MessageChannel, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event);
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
