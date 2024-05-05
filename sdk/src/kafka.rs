// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

//! Apache Kafka integration. Work in progress, at the moment.

mod implementation;

pub use self::implementation::MessageChannel;
pub use self::implementation::MessageBrokerConfiguration;

pub use self::implementation::setup;
pub use self::implementation::register;
pub use self::implementation::unregister;
pub use self::implementation::emit;
pub use self::implementation::emit_to_channel;
pub use self::implementation::configuration;
pub use self::implementation::message_channel;
