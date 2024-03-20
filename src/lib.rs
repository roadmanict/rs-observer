pub trait Observer {
    type Item;

    fn on_notify(&self, data: Self::Item);
}

pub struct Subject<'a, T> {
    observers: Vec<&'a dyn Observer<Item = T>>,
}

impl<'a, T: Clone> Subject<'a, T> {
    pub fn new() -> Subject<'a, T> {
        Subject { observers: vec![] }
    }

    pub fn attach(&mut self, observer: &'a impl Observer<Item = T>) {
        self.observers.push(observer);
    }

    pub fn detach(&mut self, observer: &impl Observer<Item = T>) {
        self.observers.retain(|o| !std::ptr::eq(*o, observer));
    }

    pub fn num_observers(&self) -> usize {
        self.observers.len()
    }

    pub fn notify(&self, data: &T) {
        self.observers
            .iter()
            .for_each(|h| h.on_notify(data.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObserver;
    struct TestObserverTwo;

    impl Observer for TestObserver {
        type Item = String;

        fn on_notify(&self, _data: Self::Item) {}
    }
    impl Observer for TestObserverTwo {
        type Item = String;

        fn on_notify(&self, _data: Self::Item) {}
    }

    #[test]
    fn can_add_and_remove_observer() {
        let mut subject: Subject<String> = Subject::new();
        let observer = TestObserver;
        let observer_two = TestObserverTwo;

        subject.attach(&observer);
        assert_eq!(subject.num_observers(), 1);
        subject.notify(&"String".to_string());
        subject.detach(&observer);
        subject.attach(&observer_two);
        assert_eq!(subject.num_observers(), 1);
    }
}
