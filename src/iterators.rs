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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let dummydata = [1 as u8,2,3,4,5,6,7,8,9,10];
        let mut iter = dummydata.into_iter();

        let mut iter2 = LimitedIterator::new(&mut iter, 0);
        assert_eq!(None, iter2.next());
        assert_eq!(Some(1), iter.next());
    }

    #[test]
    fn test_limited_iterator() {
        // Make an array to iterate over.
        let dummydata = [1 as u8,2,3,4,5,6,7,8,9,10];
        // Create a regular iterator.
        let mut iter = dummydata.into_iter();

        // Create a limiter iterator that only yields the first 4 items.
        let mut iter2 = LimitedIterator::new(&mut iter, 4);
        assert_eq!(Some(1), iter2.next());
        assert_eq!(Some(2), iter2.next());
        assert_eq!(Some(3), iter2.next());
        assert_eq!(Some(4), iter2.next());
        assert_eq!(None, iter2.next());

        // Confirm that we can still read the original iterator.
        assert_eq!(Some(5), iter.next());

        // We can make more iterators yet again.
        let mut iter2 = LimitedIterator::new(&mut iter, 2);
        assert_eq!(Some(6), iter2.next());
        assert_eq!(Some(7), iter2.next());
        assert_eq!(None, iter2.next());
        
        // Zero-sized iterators don't do anything.
        let mut iter2 = LimitedIterator::new(&mut iter, 0);
        assert_eq!(None, iter2.next());

        // The limited iterator also returns none when the parent iterator is drained.
        let mut iter2 = LimitedIterator::new(&mut iter, 100);
        assert_eq!(Some(8), iter2.next());
        assert_eq!(Some(9), iter2.next());
        assert_eq!(Some(10), iter2.next());
        assert_eq!(None, iter2.next());

        // The parent iterator also returns None.
        assert_eq!(None, iter.next());
    }
}