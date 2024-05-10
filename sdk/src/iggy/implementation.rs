// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

pub struct MessageChannel {
    pub stream_id: u32,
    pub topic_id: u32,
    pub partition_id: u32,
}

pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub server: &'static str,
}

pub fn message_channel(stream_id: u32, topic_id: u32, partition_id: u32) -> MessageChannel {
    MessageChannel {
        stream_id,
        topic_id,
        partition_id
    }
}

pub fn configuration(server: &'static str, message_channel: MessageChannel) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel,
        server,
    }
}
