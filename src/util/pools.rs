use std::collections::VecDeque;

pub struct ObjectPool<T> {
    available: VecDeque<T>,
    in_use: Vec<T>,
    factory: Box<dyn Fn() -> T>,
    reset: Box<dyn Fn(&mut T)>,
}

impl<T> ObjectPool<T> {
    pub fn new<F, R>(capacity: usize, factory: F, reset: R) -> Self
    where
        F: Fn() -> T + 'static,
        R: Fn(&mut T) + 'static,
    {
        let mut available = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            available.push_back(factory());
        }

        Self {
            available,
            in_use: Vec::with_capacity(capacity),
            factory: Box::new(factory),
            reset: Box::new(reset),
        }
    }

    pub fn acquire(&mut self) -> T {
        if let Some(mut obj) = self.available.pop_front() {
            (self.reset)(&mut obj);
            obj
        } else {
            (self.factory)()
        }
    }

    pub fn release(&mut self, mut obj: T) {
        (self.reset)(&mut obj);
        self.available.push_back(obj);
    }

    pub fn clear(&mut self) {
        while let Some(obj) = self.in_use.pop() {
            self.available.push_back(obj);
        }
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    pub fn total_count(&self) -> usize {
        self.available.len() + self.in_use.len()
    }
}
