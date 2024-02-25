pub struct RingBuffer<T, const SIZE: usize> {
    buffer: [Option<T>; SIZE],
    head: usize,
    tail: usize,
    size: usize,
}

pub enum BufferError {
    Overflow,
    // Other error types can be added here if needed in the future
}

impl<T: core::marker::Copy, const SIZE: usize> RingBuffer<T, SIZE> {
    // Creates a new RingBuffer
    pub fn new() -> RingBuffer<T, SIZE> {
        RingBuffer {
            buffer: [None; SIZE],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    // Adds an item to the buffer. Returns an error if the buffer is full.
    pub fn push(&mut self, item: T) -> Result<(), BufferError> {
        let mut result: Result<(), BufferError> = Ok(());
        while self.size >= SIZE {
            self.discard();
            result = Err(BufferError::Overflow);
        }
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % SIZE;
        self.size += 1;
        result
    }

    // Peeks the next item in the buffer
    pub fn peek(&mut self) -> Option<&mut T> {
        if self.size > 0 {
            self.buffer[self.head].as_mut()
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % SIZE;
            self.size -= 1;
            item
        } else {
            None
        }
    }

    // Discards the last item in the buffer
    pub fn discard(&mut self) {
        if self.size > 0 {
            self.buffer[self.head].take();
            self.head = (self.head + 1) % SIZE;
            self.size -= 1;
        }
    }
}
