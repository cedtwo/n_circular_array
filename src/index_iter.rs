use std::{array, fmt::Debug};

use crate::index::RawIndexSpan;
use crate::span::{BoundSpan, UnboundSpan};
use crate::span_iter::{BoundSpanIterator, SpanIterator, UnboundSpanIterator};

/// [`CircularArray`] `N` dimensional index span iterator.
///
/// Derives contiguous indices from the Cartesian product of axis spans within.
/// Produces `N` dimensional [`RawIndexSpan`]s defining the bounds of contiguous
/// slices.
///
/// For unbound iteration, contiguous ranges are only derived from axis `0`, while
/// for bound spans, contiguous ranges will extend across axes if possible. As such,
/// unbound index ranges are applicable to any `N` dimensional array as long as spans
/// are in bounds.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IndexIterator<const D: usize, S>([S; D]);

impl<const D: usize> IndexIterator<D, UnboundSpanIterator> {
    /// Create a new iterator of unbound axis spans.
    pub(crate) fn new_unbound(spans: [UnboundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|span| {
            let bounds = UnboundSpanIterator::new(span, cont);
            cont = cont && bounds.exhaustive();

            bounds
        });

        IndexIterator(bounds)
    }
}

impl<const D: usize> IndexIterator<D, BoundSpanIterator> {
    /// Create a new iterator for bound axis spans. Spans are **not** contiguous
    /// across axes, and therefore only axis `0` will be contiguous. This should
    /// be preffered if mapping slices from/to an array of a different shape.
    pub(crate) fn new_bound(spans: [BoundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|span| {
            let bounds = BoundSpanIterator::new(span, false, cont);
            cont = false;

            bounds
        });

        IndexIterator(bounds)
    }

    /// Create a new iterator for bound axis spans. Spans are contiguous across
    /// axes where possible. This should be preffered destination arrays when
    /// mapping elements from iterators or contiguous slices.
    pub(crate) fn new_bound_contiguous(spans: [BoundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|span| {
            let bounds = BoundSpanIterator::new(span, false, cont);
            cont = cont && bounds.exhaustive();

            bounds
        });

        IndexIterator(bounds)
    }

    // TODO: This has the potential for improved cache locality for the destination
    // array. Requires creating `BoundSpan`s for the source. Applicable to `push` and
    // `push_fn` mutation methods.

    /// Create a new iterator for bound axis spans. Spans are contiguous across
    /// axes where possible and always ordered.
    #[allow(dead_code)]
    pub(crate) fn new_bound_contiguous_ordered(spans: [BoundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|mut span| {
            // Mutate spans into exhaustive spans, if possible.
            if span.len() == span.bound() {
                span = BoundSpan::new(0, span.bound(), span.bound());
            }

            let bounds = BoundSpanIterator::new(span, true, cont);
            cont = cont && bounds.exhaustive();

            bounds
        });

        IndexIterator(bounds)
    }
}

impl<const D: usize, S> IndexIterator<D, S> {
    /// Get a reference to the inner span array.
    fn inner(&self) -> &[S; D] {
        &self.0
    }

    /// Get a mutable reference to the inner span array.
    fn inner_mut(&mut self) -> &mut [S; D] {
        &mut self.0
    }
}

impl<const D: usize, S: SpanIterator> Iterator for IndexIterator<D, S> {
    type Item = RawIndexSpan<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner().iter().all(|bounds| bounds.is_finished()) {
            None
        } else {
            let mut finished = true;

            let span = array::from_fn(|i| {
                let bounds = &mut self.inner_mut()[i];

                let span = if finished {
                    match bounds.next() {
                        Some(bounds) => bounds,
                        None => {
                            bounds.reset();
                            bounds.next().expect("No bounds returned from iterator")
                        }
                    }
                // Continue or reset and continue iteration.
                } else {
                    // TODO: Should this ever be `None`?
                    // Why would the iterator be exhausted prior to calling current?
                    match bounds.get() {
                        Some(bounds) => bounds,
                        None => {
                            bounds.reset();
                            bounds.get().expect("No current bounds")
                        }
                    }
                };

                finished = finished && bounds.is_finished();
                span
            });

            Some(span.into())
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod index_iterator {

        #[cfg(test)]
        mod unbound {
            use crate::index_iter::IndexIterator;
            use crate::span::UnboundSpan;

            #[test]
            fn iter() {
                let iter = IndexIterator::new_unbound([
                    UnboundSpan::new(0, 1),
                    UnboundSpan::new(1, 2),
                    UnboundSpan::new(1, 1),
                ]);
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([0, 1, 1], [1, 1, 1]),
                    ([0, 2, 1], [1, 2, 1])
                ]);

                let iter = IndexIterator::new_unbound([
                    UnboundSpan::new(0, 2),
                    UnboundSpan::new(1, 3),
                    UnboundSpan::new(2, 3),
                ]);
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([0, 1, 2], [2, 1, 2]),
                    ([0, 2, 2], [2, 2, 2]),
                    ([0, 3, 2], [2, 3, 2]),
                    ([0, 1, 3], [2, 1, 3]),
                    ([0, 2, 3], [2, 2, 3]),
                    ([0, 3, 3], [2, 3, 3]),
                ]);
            }
        }

        mod bound {
            use crate::index_iter::IndexIterator;
            use crate::CircularArrayVec;

            #[test]
            fn iter() {
                let shape = [4, 3, 2];
                let mut array = CircularArrayVec::from_iter(shape, 0..shape.iter().product());

                array.offset = [2, 2, 1];
                let iter = IndexIterator::new_bound_contiguous(array.spans());
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([2, 2, 1], [3, 2, 1]),
                    ([0, 2, 1], [1, 2, 1]),
                    ([2, 0, 1], [3, 0, 1]),
                    ([0, 0, 1], [1, 0, 1]),
                    ([2, 1, 1], [3, 1, 1]),
                    ([0, 1, 1], [1, 1, 1]),
                    ([2, 2, 0], [3, 2, 0]),
                    ([0, 2, 0], [1, 2, 0]),
                    ([2, 0, 0], [3, 0, 0]),
                    ([0, 0, 0], [1, 0, 0]),
                    ([2, 1, 0], [3, 1, 0]),
                    ([0, 1, 0], [1, 1, 0])
                ]);

                array.offset = [0, 2, 1];
                let iter = IndexIterator::new_bound_contiguous(array.spans());
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([0, 2, 1], [3, 2, 1]),
                    ([0, 0, 1], [3, 1, 1]),
                    ([0, 2, 0], [3, 2, 0]),
                    ([0, 0, 0], [3, 1, 0])
                ]);

                array.offset = [0, 0, 1];
                let iter = IndexIterator::new_bound_contiguous(array.spans());
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([0, 0, 1], [3, 2, 1]),
                    ([0, 0, 0], [3, 2, 0]),
                ]);

                array.offset = [0, 0, 0];
                let iter = IndexIterator::new_bound_contiguous(array.spans());
                #[rustfmt::skip]
                assert_eq!(iter.collect::<Vec<_>>(), [
                    ([0, 0, 0], [3, 2, 1]),
                ]);
            }

            #[test]
            fn iter_cont() {
                let shape = [4, 3, 2];
                let mut array = CircularArrayVec::from_iter(shape, 0..shape.iter().product());

                array.offset = [2, 2, 1];
                let iter = IndexIterator::new_bound_contiguous_ordered(array.spans());
                assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

                array.offset = [0, 2, 1];
                let iter = IndexIterator::new_bound_contiguous_ordered(array.spans());
                assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

                array.offset = [0, 0, 1];
                let iter = IndexIterator::new_bound_contiguous_ordered(array.spans());
                assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

                array.offset = [0, 0, 0];
                let iter = IndexIterator::new_bound_contiguous_ordered(array.spans());
                assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);
            }
        }
    }
}
