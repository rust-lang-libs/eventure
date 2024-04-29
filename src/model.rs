pub trait Event {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}

pub trait EventHandler {
    fn handle(&self, event: &dyn Event);
}

pub trait EventHandlerRegistry {
    fn register(&mut self, event_handler: Box<dyn EventHandler + Send>);
    fn emit(&self, event: &dyn Event);
}
