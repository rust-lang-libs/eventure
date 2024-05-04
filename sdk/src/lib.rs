// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

//! # Eventure
//! 'Eventure' a library that supports event-driven messaging design pattern in Rust.
//!
//! Different message broker integrations are/will be implemented (in-memory, Kafka, RabbitMQ, etc...),
//! supporting variety of different scenarios, both for modular monoliths and microservices.

pub mod model;
pub mod in_memory;
pub mod kafka;
