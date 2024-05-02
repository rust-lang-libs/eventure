# Event-Driven messaging library for Rust

It's an early alpha version, not ready for production yet.

The main goal of the project is to create a library with one model abstraction and different implementations 
for a variety of message brokers, like Kafka, RabbitMQ, Iggy, in-memory, etc... Ideally, the library should
be able to handle all possible event-driven oriented scenarios, both for modular monolith and distributed
applications (microservices, typically).

[*Release notes*](https://github.com/rust-lang-libs/eventure/releases)

## Building project
`cargo build`

## Testing project
`cargo build`

## Running examples
* In-Memory synchronous messages
`cargo run --example inmemory-sync`

* In-Memory asynchronous messages (dummy implementation at the moment)
`cargo run --example inmemory-async`

* Kafka messages (dummy implementation at the moment)
`cargo run --example inmemory-async`
