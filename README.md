# Event-Driven messaging library for Rust

[![crates.io](https://img.shields.io/crates/v/eventure.svg)](https://crates.io/crates/eventure)
[![crates.io](https://img.shields.io/crates/d/eventure.svg)](https://crates.io/crates/eventure)
[![docs](https://docs.rs/eventure/badge.svg)](https://docs.rs/eventure)
[![workflow](https://github.com/rust-lang-libs/eventure/actions/workflows/build.yml/badge.svg)](https://github.com/rust-lang-libs/eventure/actions/workflows/build.yml)
[![dependency](https://deps.rs/repo/github/rust-lang-libs/eventure/status.svg)](https://deps.rs/repo/github/rust-lang-libs/eventure)

It's an early alpha version, not ready for production yet.

The main goal of the project is to create a library with one model abstraction and different implementations 
for a variety of message brokers, like Kafka, RabbitMQ, Iggy, in-memory, etc... Ideally, the library should
be able to handle all possible event-driven oriented scenarios, both for modular monolith and distributed
applications (microservices, typically).

[*Release notes*](https://github.com/rust-lang-libs/eventure/releases)

## Building project
`cargo build`

## Testing project
`cargo test`

## Running examples
### In-Memory messages
`cargo run --example in-memory`

