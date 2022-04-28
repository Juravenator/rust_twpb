
pub struct LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    source_iterator: &'a mut I,
    range: u32,
    current_index: u32,
}

impl<'a, 'b, I> LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    pub fn new(source_iterator: &'a mut I, range: u32) -> Self {
        LimitedIterator{
            source_iterator: source_iterator,
            range: range,
            current_index: 0,
        }
    }
}

impl<'a, 'b, I> Iterator for LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    type Item = &'b u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.range {
            self.current_index += 1;
            self.source_iterator.next()
        } else {
            None
        }
    }
}