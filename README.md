# Event-Driven messaging library for Rust

[![crates.io](https://img.shields.io/crates/v/eventure.svg)](https://crates.io/crates/eventure)
[![crates.io](https://img.shields.io/crates/d/eventure.svg)](https://crates.io/crates/eventure)
[![docs](https://docs.rs/eventure/badge.svg)](https://docs.rs/eventure)
[![workflow](https://github.com/rust-lang-libs/eventure/actions/workflows/build.yml/badge.svg)](https://github.com/rust-lang-libs/eventure/actions/workflows/build.yml)
[![dependency](https://deps.rs/repo/github/rust-lang-libs/eventure/status.svg)](https://deps.rs/repo/github/rust-lang-libs/eventure)

It's an early alpha version, not ready for production yet.

The main goal of the project is to create a library with one model abstraction and different implementations 
for a variety of message brokers, like Kafka, RabbitMQ, Iggy, custom In-memory, etc... Ideally, the library should
be able to handle all possible event-driven oriented scenarios, both for modular monolith and distributed
applications (microservices, typically).

[*Release notes*](https://github.com/rust-lang-libs/eventure/releases)

## Building project
`cargo build`

## Testing project
`cargo test`

## Examples
* [Getting Started](examples/src/getting-started/main.rs)
* [Im-Memory](examples/src/in_memory/main.rs)
* [Im-Memory Multi-Threaded](examples/src/in_memory_multi_threaded/main.rs)
* [Kafka](examples/src/kafka/main.rs)
* [Kafka Many Handlers](examples/src/kafka_many_handlers/main.rs)

## Running examples

### Getting started
`cargo run --example getting-started`

### In-Memory messages
Run examples:
```
cargo run --example in-memory
cargo run --example in-memory-multi-threaded
```

### Kafka messages
Install docker and docker-compose on local machine, start kafka containers:
```
cd tools\kafka
docker-compose up -d
```
Create topic 'orders':
```
kafka-topics --bootstrap-servers localhost:9092 --create --topic orders
```
Run examples:
```
cargo run --example kafka
cargo run --example kafka-many-handlers
```
