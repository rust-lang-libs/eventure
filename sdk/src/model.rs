// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::any::Any;
use std::fmt::{Display};

// -----------------------------------------------------------------------------------------------------------------------------------------
// Public traits
// -----------------------------------------------------------------------------------------------------------------------------------------

/// Base event abstraction. It should be implemented for each event.
///
/// # Examples
///
/// ```
/// use std::any::Any;
/// use std::fmt::{Display, Formatter};
/// use eventure::model;
///
/// pub struct OrderCreated {
///     event_id: String,
///     customer_id: String,
/// }
///
/// impl Display for OrderCreated {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{} event with id {}",
///                "OrderCreated", self.event_id)
///     }
/// }
///
/// impl model::Event for OrderCreated {
///     fn id(&self) -> &str {
///         &self.event_id[..]
///     }
///     fn name(&self) -> &str {
///         "OrderCreated"
///     }
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// ```
pub trait Event: Display {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

/// Base event handler abstraction. It should be implemented for each event handler.
/// # Examples
///
/// ```
/// use std::fmt::{Display, Formatter};
/// use log::info;
/// use eventure::model;
///
/// struct OrderCreatedEventHandler {
///    id: String
///}
///
/// struct OrderCreated {
///     event_id: String,
///     customer_id: String,
/// }
///
/// impl Display for OrderCreated {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
///
/// impl Display for OrderCreatedEventHandler {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", "OrderEventHandler")
///     }
/// }
///
/// impl model::EventHandler for OrderCreatedEventHandler {
///     fn handle(&self, event: &(dyn model::Event + '_)) {
///         match event.as_any().downcast_ref::<OrderCreated>() {
///             Some(order_create) => self.handle(order_create),
///             None => info!(target: "OrderCreatedEventHandler", "not handling {}", event)
///         }
///     }
///
///     fn id(&self) -> String {
///         String::from(&self.id)
///     }
/// }
///
///
/// impl OrderCreatedEventHandler {
///     fn handle(&self, event: &OrderCreated) {
///         info!(target: "OrderCreatedEventHandler", "handling {}", event)
///     }
/// }
/// ```
pub trait EventHandler: Display {
    fn handle(&self, event: &dyn Event);
    fn id(&self) -> String;
}
