pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    start: usize,
    filled: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }
        Self {
            buffer, 
            capacity,
            start: 0,
            filled: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.filled != self.capacity {
            self.buffer[(self.start + self.filled) % self.capacity] = Some(element);
            self.filled += 1;
            Ok(())
        } else {
            Err(Error::FullBuffer)
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.filled > 0 {
            if let Some(val) = self.buffer.get_mut(self.start) {
                if let Some(ret) = val.take() {
                    self.start = (self.start + 1) % self.capacity;
                    self.filled -= 1;
                    return Ok(ret);
                }
            }
        }
        Err(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        for i in 0..self.filled {
            self.buffer[(self.start + i) % self.capacity] = None;
        }
        self.filled = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.filled != self.capacity {
            match self.write(element) {
                Err(Error::FullBuffer) => unreachable!(),
                _ => {}
            }
        } else {
            self.buffer[self.start] = Some(element);
            self.start = (self.start + 1) % self.capacity;
        }
    }
}
