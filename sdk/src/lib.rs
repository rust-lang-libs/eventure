// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

//! # Eventure
//!
//! Eventure is a library that supports event-driven messaging design pattern in Rust.
//!
//! Different message broker integrations are/will be implemented (In-Memory, Kafka, RabbitMQ, etc...),
//! supporting variety of different scenarios, both for monolith and microservice-based applications.

pub mod model;
pub mod in_memory;
pub mod kafka;
pub mod iggy;
mod common;

