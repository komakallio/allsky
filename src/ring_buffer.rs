use std::collections::VecDeque;

pub(crate) struct RingBuffer<T> {
    internal_queue: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            internal_queue: VecDeque::<T>::with_capacity(capacity),
            capacity,
        }
    }

    pub(crate) fn add(&mut self, item: T) {
        if self.internal_queue.len() >= self.capacity {
            _ = self.internal_queue.pop_front();
        }
        self.internal_queue.push_back(item);
    }

    pub(crate) fn get_contents(&mut self) -> &[T] {
        self.internal_queue.make_contiguous()
    }
}
