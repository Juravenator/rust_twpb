use bytes::{BufMut, buf::UninitSlice};

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

pub struct NullCounterBuffer {
    current_index: usize
}

impl NullCounterBuffer {
    pub fn new() -> Self {
        NullCounterBuffer{current_index: 0}
    }
}

unsafe impl BufMut for NullCounterBuffer {
    fn remaining_mut(&self) -> usize {
        usize::MAX
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.current_index += cnt;
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        let buf = heapless::Vec::<u8, 10>::new();
        let ptr = buf.as_ptr() as *mut _;
        let len = buf.len();
        let slice = unsafe { UninitSlice::from_raw_parts_mut(ptr, len) };
        slice
    }
}