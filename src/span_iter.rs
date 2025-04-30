use std::ops::Range;
// use crate::axis_iter::AxisIterator;
use std::{array, fmt::Debug};

use crate::index::RawIndexSpan;
use crate::index_bounds::IndexBounds;
use crate::span::BoundSpan;
use crate::strides::Strides;

/// `CircularArray` index span iterator. Derives indices from the Cartesian product
/// of `IndexBounds` sets  within. Returns contiguous sequences of indices where
/// possible, starting at indices defined within the `Span`s provided.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SpanIterator<const D: usize>([IndexBounds; D]);

impl<const D: usize> SpanIterator<D> {
    /// Create a new iterator for the given axis `spans`, with iteration aligned
    /// to the offset.
    pub(crate) fn new(spans: [BoundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|span| {
            let bounds = IndexBounds::new(span, false, cont);
            cont = cont && bounds.exhaustive();

            bounds
        });

        SpanIterator(bounds)
    }

    /// Create a new iterator for the given axis `spans`, maintaining the contiguity
    /// of the array.
    #[allow(dead_code)]
    pub(crate) fn new_contiguous(spans: [BoundSpan; D]) -> Self {
        let mut cont = true;

        let bounds = spans.map(|mut span| {
            // Mutate spans into exhaustive spans, if possible.
            if span.len() == span.bound() {
                span = BoundSpan::new(0, span.bound(), span.bound());
            }

            let bounds = IndexBounds::new(span, true, cont);
            cont = cont && bounds.exhaustive();

            bounds
        });

        SpanIterator(bounds)
    }

    /// Get a reference to the inner `IndexBounds` array.
    fn inner(&self) -> &[IndexBounds; D] {
        &self.0
    }

    /// Get a mutable reference to the inner `IndexBounds` array.
    fn inner_mut(&mut self) -> &mut [IndexBounds; D] {
        &mut self.0
    }
}

impl<const D: usize> Iterator for SpanIterator<D> {
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

/// Iterator adaptor for `RawIndexSpan` conversion.
pub(crate) trait RawIndexAdaptor<'a, const N: usize> {
    /// Flatten `RawIndexSpan` types into `usize` elements.
    #[allow(dead_code)]
    fn into_indices(self, strides: &'a Strides<N>) -> impl Iterator<Item = usize> + Clone + 'a;

    /// Type conversion from `RawIndexSpan` into slice `Range<usize>` types.
    fn into_ranges(
        self,
        strides: &'a Strides<N>,
    ) -> impl Iterator<Item = Range<usize>> + Clone + 'a;
}

impl<'a, const N: usize, T: Iterator<Item = RawIndexSpan<N>> + Clone + 'a> RawIndexAdaptor<'a, N>
    for T
{
    fn into_indices(self, strides: &'a Strides<N>) -> impl Iterator<Item = usize> + Clone + 'a {
        self.flat_map(|span| {
            let (start, end) = span.split_bounds();
            strides.apply_to_index(*start)..strides.apply_to_index(*end) + 1
        })
    }

    fn into_ranges(
        self,
        strides: &'a Strides<N>,
    ) -> impl Iterator<Item = Range<usize>> + Clone + 'a {
        self.map(|span| {
            let (start, end) = span.split_bounds();
            strides.apply_to_index(*start)..strides.apply_to_index(*end) + 1
        })
    }
}

#[cfg(test)]
mod test {
    use crate::span_iter::SpanIterator;
    use crate::CircularArrayVec;

    #[test]
    fn iter() {
        let shape = [4, 3, 2];
        let mut array = CircularArrayVec::from_iter(shape, 0..shape.iter().product());

        array.offset = [2, 2, 1];
        let iter = SpanIterator::new(array.spans());
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
        let iter = SpanIterator::new(array.spans());
        #[rustfmt::skip]
        assert_eq!(iter.collect::<Vec<_>>(), [
            ([0, 2, 1], [3, 2, 1]),
            ([0, 0, 1], [3, 1, 1]),
            ([0, 2, 0], [3, 2, 0]),
            ([0, 0, 0], [3, 1, 0])
        ]);

        array.offset = [0, 0, 1];
        let iter = SpanIterator::new(array.spans());
        #[rustfmt::skip]
        assert_eq!(iter.collect::<Vec<_>>(), [
            ([0, 0, 1], [3, 2, 1]),
            ([0, 0, 0], [3, 2, 0]),
        ]);

        array.offset = [0, 0, 0];
        let iter = SpanIterator::new(array.spans());
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
        let iter = SpanIterator::new_contiguous(array.spans());
        assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

        array.offset = [0, 2, 1];
        let iter = SpanIterator::new_contiguous(array.spans());
        assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

        array.offset = [0, 0, 1];
        let iter = SpanIterator::new_contiguous(array.spans());
        assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);

        array.offset = [0, 0, 0];
        let iter = SpanIterator::new_contiguous(array.spans());
        assert_eq!(iter.collect::<Vec<_>>(), [([0, 0, 0], [3, 2, 1]),]);
    }
}
