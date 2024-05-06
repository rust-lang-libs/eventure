// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use log::LevelFilter;
use simple_logger::SimpleLogger;

use eventure::kafka;
use examples::shared::order_created;

// -----------------------------------------------------------------------------------------------------------------------------------------
// Main function
// -----------------------------------------------------------------------------------------------------------------------------------------

fn main() {
    println!();
    init_logger();

    let configuration = kafka::configuration("orders", 0);
    kafka::setup(configuration);

    let order_created = order_created::create();
    kafka::emit(&order_created);

    println!();
}

fn init_logger() {
    SimpleLogger::new()
        .with_colors(false)
        .with_level(LevelFilter::Debug)
        .init().unwrap();
}
