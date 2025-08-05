use crate::span::{BoundSpan, UnboundSpan};

/// Defines iterators that produce axis indices from spans.
pub(crate) trait SpanIterator: Iterator<Item = UnboundSpan> {
    /// Returns the current iteration.
    fn i(&self) -> usize;

    /// Returns `true` if the span exhausts the axis.
    fn exhaustive(&self) -> bool;

    /// Increment the iteration index.
    fn incr(&mut self);

    /// Returns `true` if iteration has finished.
    fn is_finished(&self) -> bool;

    /// Reset the iterator.
    fn reset(&mut self);

    /// Get the span or index for the current iteration index.
    fn get(&self) -> Option<<Self as Iterator>::Item>;
}

/// [`UnboundSpan`] span iterator. Produces [`UnboundSpan`]s of **contiguous**
/// elements during iteration.
#[derive(Debug, Clone, Copy)]
pub(crate) struct UnboundSpanIterator {
    /// The span that will be iterated over.
    span: UnboundSpan,
    /// Iteration index.
    i: usize,

    /// Iterate over contiguous spans.
    iter_span: bool,
}

impl UnboundSpanIterator {
    /// Create a pair of `IndexBounds` a set, or sets of `Bounds`.
    pub(crate) fn new(span: UnboundSpan, iter_span: bool) -> Self {
        Self {
            span,
            i: 0,
            iter_span,
        }
    }
}

impl SpanIterator for UnboundSpanIterator {
    fn i(&self) -> usize {
        self.i
    }

    fn exhaustive(&self) -> bool {
        false
    }

    fn incr(&mut self) {
        self.i += 1;
    }

    fn is_finished(&self) -> bool {
        self.i() >= self.len()
    }

    fn reset(&mut self) {
        self.i = 0;
    }

    fn get(&self) -> Option<<Self as Iterator>::Item> {
        match self.iter_span {
            true => {
                if self.i == 0 {
                    Some(self.span)
                } else {
                    None
                }
            }
            false => self.span.get_index(self.i).map(|i| i.into()),
        }
    }
}

impl ExactSizeIterator for UnboundSpanIterator {
    fn len(&self) -> usize {
        if self.iter_span {
            1
        } else {
            self.span.len()
        }
    }
}

impl Iterator for UnboundSpanIterator {
    type Item = UnboundSpan;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get();
        self.incr();

        item
    }
}

/// [`BoundSpan`] span iterator. Produces [`UnboundSpan`]s of **contiguous**
/// elements during iteration. In contrast to [`UnboundSpanIterator`], this may
/// produce additional [`UnboundSpan`]s across axis bounds.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BoundSpanIterator {
    /// The span that will be iterated over.
    bound_span: BoundSpan,
    /// Iteration index.
    i: usize,

    /// Iterate over elements sequentially.
    iter_seq: bool,
    /// Iterate over contiguous spans.
    iter_span: bool,
}

impl BoundSpanIterator {
    /// Create a pair of `IndexBounds` a set, or sets of `Bounds`.
    pub(crate) fn new(span: BoundSpan, iter_seq: bool, iter_span: bool) -> Self {
        Self {
            bound_span: span,
            i: 0,
            iter_seq,
            iter_span,
        }
    }
}

impl SpanIterator for BoundSpanIterator {
    fn i(&self) -> usize {
        self.i
    }

    fn exhaustive(&self) -> bool {
        self.iter_span && self.bound_span.exhaustive()
    }

    fn incr(&mut self) {
        self.i += 1;
    }

    fn is_finished(&self) -> bool {
        self.i() >= self.len()
    }

    fn reset(&mut self) {
        self.i = 0;
    }

    fn get(&self) -> Option<<Self as Iterator>::Item> {
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

impl ExactSizeIterator for BoundSpanIterator {
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

impl Iterator for BoundSpanIterator {
    type Item = UnboundSpan;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get();
        self.incr();

        item
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod unbound {

        use crate::span::UnboundSpan;
        use crate::span_iter::UnboundSpanIterator;

        #[test]
        fn len() {
            let idx_iter = UnboundSpanIterator::new(UnboundSpan::new(1, 3), false);
            let span_iter = UnboundSpanIterator::new(UnboundSpan::new(1, 3), true);

            assert_eq!(idx_iter.len(), 3);
            assert_eq!(span_iter.len(), 1);
        }

        #[test]
        fn iter() {
            let iter = UnboundSpanIterator::new(UnboundSpan::new(1, 3), false);

            #[rustfmt::skip]
            assert_eq!(iter.collect::<Vec<_>>(), [
                (1, 1).into(), (2, 2).into(), (3, 3).into()
            ]);
        }

        #[test]
        fn iter_span() {
            let iter = UnboundSpanIterator::new(UnboundSpan::new(1, 3), true);

            assert_eq!(iter.collect::<Vec<_>>(), [(1, 3).into()]);
        }
    }

    #[cfg(test)]
    mod bound {

        use crate::span::BoundSpan;
        use crate::span_iter::BoundSpanIterator;

        #[test]
        fn len() {
            let idx_iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), false, false);
            let span_iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), false, true);

            assert_eq!(idx_iter.len(), 5);
            assert_eq!(span_iter.len(), 2);
        }

        #[test]
        fn iter() {
            let iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), false, false);

            #[rustfmt::skip]
            assert_eq!(iter.collect::<Vec<_>>(), [
                (4, 4).into(), (5, 5).into(), (0, 0).into(), (1, 1).into(), (2, 2).into()
            ]);
        }

        #[test]
        fn iter_seq() {
            let iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), true, false);

            #[rustfmt::skip]
            assert_eq!(iter.collect::<Vec<_>>(), [
                (0, 0).into(), (1, 1).into(), (2, 2).into(), (4, 4).into(), (5, 5).into()
            ]);
        }

        #[test]
        fn iter_span() {
            let iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), false, true);

            assert_eq!(iter.collect::<Vec<_>>(), [(4, 5).into(), (0, 2).into()]);
        }

        #[test]
        fn iter_seq_span() {
            let iter = BoundSpanIterator::new(BoundSpan::new(4, 5, 6), true, true);

            assert_eq!(iter.collect::<Vec<_>>(), [(0, 2).into(), (4, 5).into()]);
        }
    }
}
