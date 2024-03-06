use core::hash::Hash;
use std::collections::HashMap;

pub type Subscriber<T> = fn(value: T);

pub struct Publisher<E: Eq + Hash + Clone, T: Clone> {
    handlers: HashMap<E, Vec<Subscriber<T>>>,
}

impl<E: Eq + Hash + Clone, T: Clone> Publisher<E, T> {
    pub fn new() -> Publisher<E, T> {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_type: impl Into<E>, listener: Subscriber<T>) {
        let event: E = event_type.into();
        self.handlers.entry(event.clone()).or_default();
        self.handlers.get_mut(&event).map(|v| v.push(listener));
    }

    pub fn unsubscribe(&mut self, event_type: impl Into<E>, listener: Subscriber<T>) {
        self.handlers
            .get_mut(&event_type.into())
            .map(|v| v.retain(|&h| h != listener));
    }

    pub fn notify(&self, event_type: impl Into<E>, value: T) {
        self.handlers
            .get(&event_type.into())
            .map(|v| v.iter().for_each(|h| h(value.clone())));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_subscribe() {
        #[derive(PartialEq, Eq, Hash, Clone)]
        enum EventType {
            First,
            _Second,
        }

        let mut publisher: Publisher<EventType, String> = Publisher::new();

        publisher.subscribe(EventType::First, |_s| ())
    }
}
