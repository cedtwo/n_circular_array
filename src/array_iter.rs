/// `ExactSizeIterator` implementation for indexing operations.
pub struct CircularArrayIterator<'a, I: Iterator<Item = &'a T>, T: 'a> {
    iter: I,
    len: usize,
}

impl<'a, I: Iterator<Item = &'a T> + Clone, T: 'a> CircularArrayIterator<'a, I, T> {
    /// Create a new `CircularArrayIterator`. The given `len` **must** match the
    /// length of the `Iterator` provided.
    pub(crate) fn new(iter: I, len: usize) -> Self {
        debug_assert_eq!(iter.clone().count(), len);

        Self { iter, len }
    }
}

impl<'a, I: Iterator<Item = &'a T>, T: 'a> Iterator for CircularArrayIterator<'a, I, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, I: Iterator<Item = &'a T>, T: 'a> ExactSizeIterator for CircularArrayIterator<'a, I, T> {
    fn len(&self) -> usize {
        self.len
    }
}
