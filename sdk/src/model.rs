// -----------------------------------------------------------------------------------------------------------------------------------------
// Rust-Lang Libs/Eventure 2024
// -----------------------------------------------------------------------------------------------------------------------------------------

use std::any::Any;
use std::fmt::Display;

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
/// impl Display for OrderCreated{
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
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
pub trait EventHandler: Display {
    fn handle(&self, event: &dyn Event);
}
