pub trait Event {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}

pub trait EventHandler {
    fn handle(&self, event: Box<dyn Event>);
}

pub trait EventHandlerRegistry {
    fn register(&self, event_handler: Box<dyn EventHandler>);
}
