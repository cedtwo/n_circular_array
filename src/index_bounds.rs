//! Logic for confining an index or indices within limits.
use std::fmt::Debug;

use super::span::Span;
use crate::span::BoundSpan;

/// An `Iterator` of indices across an axis. Defines iteration strategies ovr the
/// contained `Span`. This should be constructed by [`CircularIterator`] rather
/// than manually.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IndexBounds {
    /// The span that will be iterated over.
    bound_span: BoundSpan,
    /// Iteration index.
    i: usize,

    /// Iterate over elements sequentially.
    iter_seq: bool,
    /// Iterate over contiguous spans.
    iter_span: bool,
}

impl IndexBounds {
    /// Create a pair of `IndexBounds` a set, or sets of `Bounds`.
    pub(crate) fn new(span: BoundSpan, iter_seq: bool, iter_span: bool) -> Self {
        Self {
            bound_span: span,
            i: 0,
            iter_seq,
            iter_span,
        }
    }

    /// Returns the current iteration.
    pub(crate) fn i(&self) -> usize {
        self.i
    }

    /// Returns `true` if the span is both contiguous and exhaustive.
    pub(crate) fn exhaustive(&self) -> bool {
        self.iter_span && self.bound_span.exhaustive()
    }

    /// Increment the iteration index.
    fn incr(&mut self) {
        self.i += 1;
    }

    /// Returns `true` if iteration has finished.
    pub(crate) fn is_finished(&self) -> bool {
        self.i() >= self.len()
    }

    /// Reset the iterator.
    pub(crate) fn reset(&mut self) {
        self.i = 0;
    }

    /// Get the span or index for the current iteration index.
    pub(crate) fn get(&self) -> Option<<Self as Iterator>::Item> {
        match (self.iter_seq, self.iter_span) {
            // Iterate over sequential spans.
            (true, true) => {
                if self.bound_span.is_wrapping() {
                    match self.i {
                        0 => self.bound_span.get_span(1),
                        1 => self.bound_span.get_span(0),
                        _ => None,
                    }
                } else {
                    self.bound_span.get_span(0)
                }
            }
            // Iterate over sequential indices.
            (true, false) => self.bound_span.get_index_ordered(self.i).map(|i| i.into()),

            // Iterate over non-sequential spans.
            (false, true) => self.bound_span.get_span(self.i),
            // Iterate over non-sequential indices.
            (false, false) => self.bound_span.get_index(self.i).map(|i| i.into()),
        }
    }
}

impl ExactSizeIterator for IndexBounds {
    fn len(&self) -> usize {
        if self.iter_span {
            match self.bound_span.is_wrapping() {
                true => 2,
                false => 1,
            }
        } else {
            self.bound_span.len()
        }
    }
}

impl Iterator for IndexBounds {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get();
        self.incr();

        item
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn len() {
        let idx_iter = IndexBounds::new(BoundSpan::new(4, 5, 6), false, false);
        let span_iter = IndexBounds::new(BoundSpan::new(4, 5, 6), false, true);

        assert_eq!(idx_iter.len(), 5);
        assert_eq!(span_iter.len(), 2);
    }

    #[test]
    fn iter() {
        let iter = IndexBounds::new(BoundSpan::new(4, 5, 6), false, false);

        #[rustfmt::skip]
        assert_eq!(iter.collect::<Vec<_>>(), [
                (4, 4).into(), (5, 5).into(), (0, 0).into(), (1, 1).into(), (2, 2).into()
        ]);
    }

    #[test]
    fn iter_seq() {
        let iter = IndexBounds::new(BoundSpan::new(4, 5, 6), true, false);

        #[rustfmt::skip]
        assert_eq!(iter.collect::<Vec<_>>(), [
            (0, 0).into(), (1, 1).into(), (2, 2).into(), (4, 4).into(), (5, 5).into()
        ]);
    }

    #[test]
    fn iter_span() {
        let iter = IndexBounds::new(BoundSpan::new(4, 5, 6), false, true);

        assert_eq!(iter.collect::<Vec<_>>(), [(4, 5).into(), (0, 2).into()]);
    }

    #[test]
    fn iter_seq_span() {
        let iter = IndexBounds::new(BoundSpan::new(4, 5, 6), true, true);

        assert_eq!(iter.collect::<Vec<_>>(), [(0, 2).into(), (4, 5).into()]);
    }
}
