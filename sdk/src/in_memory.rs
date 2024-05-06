// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

//! In-Memory message broken implementation.
//!
//! At the moment only synchronous mode is supported (for queues and topics).

mod implementation;

pub use self::implementation::ChannelType;
pub use self::implementation::MessageChannel;
pub use self::implementation::MessageBrokerConfiguration;
pub use self::implementation::setup;
pub use self::implementation::register;
pub use self::implementation::unregister;
pub use self::implementation::emit;
pub use self::implementation::emit_to_channel;
pub use self::implementation::configuration;
pub use self::implementation::message_channel;
