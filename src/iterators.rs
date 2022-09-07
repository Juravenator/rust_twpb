use crate::traits::{Writer, WriterError};

pub struct LimitedIterator<I> where I: Iterator<Item = u8> {
    source_iterator: I,
    range: u32,
    current_index: u32,
}

impl<I> LimitedIterator<I> where I: Iterator<Item = u8> {
    pub fn new(source_iterator: I, range: u32) -> Self {
        LimitedIterator{
            source_iterator: source_iterator,
            range: range,
            current_index: 0,
        }
    }
}

impl<I> Iterator for LimitedIterator<I> where I: Iterator<Item = u8> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.range {
            self.current_index += 1;
            self.source_iterator.next()
        } else {
            None
        }
    }
}

pub struct NullCounterBuffer {
    current_index: usize
}

impl NullCounterBuffer {
    pub fn new() -> Self {
        NullCounterBuffer{current_index: 0}
    }
}

impl Writer for NullCounterBuffer {
    fn write(&mut self, _byte: u8) -> Result<(), WriterError> {
        self.current_index += 1;
        Ok(())
    }
}