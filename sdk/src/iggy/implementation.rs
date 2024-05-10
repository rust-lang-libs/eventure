// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public structs
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Iggy message channel definition.
pub struct MessageChannel {
    pub stream_id: u32,
    pub topic_id: u32,
    pub partition_id: u32,
}

/// Iggy message broker configuration.
pub struct MessageBrokerConfiguration {
    pub message_channel: MessageChannel,
    pub server: &'static str,
}

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Creates Iggy message channel.
pub fn message_channel(stream_id: u32, topic_id: u32, partition_id: u32) -> MessageChannel {
    MessageChannel {
        stream_id,
        topic_id,
        partition_id
    }
}

/// Creates Iggy message broker configuration.
pub fn configuration(server: &'static str, message_channel: MessageChannel) -> MessageBrokerConfiguration {
    MessageBrokerConfiguration {
        message_channel,
        server,
    }
}
