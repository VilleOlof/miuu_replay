#[derive(Debug)]
pub struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    start: usize,
    end: usize,
    size: usize,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            start: 0,
            end: 0,
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            start: 0,
            end: 0,
            size: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn fast_front(&self) -> &T {
        &self.buffer[self.start].as_ref().unwrap()
    }

    pub fn fast_back(&self) -> &T {
        &self.buffer[if self.end != 0 {
            self.end
        } else {
            self.capacity() - 1
        }]
        .as_ref()
        .unwrap()
    }

    pub fn get(&self, idx: usize) -> &T {
        let len = self.buffer.len();
        let real_idx = self.start
            + if idx < len - self.start {
                idx
            } else {
                idx - len
            };
        &self.buffer[real_idx].as_ref().unwrap()
    }

    pub fn set(&mut self, idx: usize, value: T) {
        let real_idx = self.internal_index(idx);
        self.buffer[real_idx] = Some(value);
    }

    pub fn internal_index(&self, idx: usize) -> usize {
        let len = self.buffer.len();
        self.start
            + if idx < len - self.start {
                idx
            } else {
                idx - len
            }
    }

    fn increment(&mut self, idx: usize) -> usize {
        let mut num = idx + 1;
        if num == self.capacity() {
            num = 0;
        }

        num
    }

    fn decrement(&mut self, idx: usize) -> usize {
        let mut num = idx;
        if num == 0 {
            num = self.capacity();
        }
        num -= 1;

        num
    }

    pub fn push_back(&mut self, value: T) {
        self.buffer[self.end] = Some(value);
        self.end = self.increment(self.end);

        if self.is_full() {
            self.start = self.end;
        } else {
            self.size += 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.is_full() {
            self.start = self.decrement(self.start);
            self.end = self.start;
            self.buffer[self.end] = Some(value);
        } else {
            self.start = self.decrement(self.start);
            self.buffer[self.start] = Some(value);
            self.size += 1;
        }
    }
}
