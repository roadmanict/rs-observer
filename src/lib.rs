pub trait Observer {
    type Item;

    fn on_notify(&self, data: Self::Item);
}

pub struct Subject<'a, T> {
    observers: Vec<&'a dyn Observer<Item = T>>,
}

impl<'a, T: Clone> Subject<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn attach(&mut self, observer: &'a dyn Observer<Item = T>) {
        self.observers.push(observer);
    }

    pub fn detach(&mut self, observer: &dyn Observer<Item = T>) {
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

impl<'a, T: Clone> Default for Subject<'a, T> {
    fn default() -> Self {
        Self {
            observers: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObserver;

    impl Observer for TestObserver {
        type Item = String;

        fn on_notify(&self, _data: Self::Item) {}
    }

    #[test]
    fn can_add_and_remove_observer() {
        let mut subject = Subject::new();
        let observer = TestObserver;

        subject.attach(&observer);
        assert_eq!(subject.num_observers(), 1);
        subject.notify(&"String".to_string());
        subject.detach(&observer);
        assert_eq!(subject.num_observers(), 0);
    }
}
