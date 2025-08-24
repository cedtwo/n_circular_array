use std::array;
use std::ops::{Index, Range};

use crate::array_iter::CircularArrayIterator;
use crate::index::RawIndexAdaptor;
use crate::index_iter::IndexIterator;
use crate::span::{BoundSpan, UnboundSpan};
use crate::CircularArray;

/// Indexing `CircularArray` operations.
pub trait CircularIndex<'a, const N: usize, T: 'a> {
    /// Get a reference to the element at the given index, aligned to the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.get([0, 0]), &0);
    /// ```
    fn get(&'a self, index: [usize; N]) -> &'a T;

    /// Get a reference to the element at the given index. This does **not**
    /// account for the offset. See [`CircularArray::offset`].
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.get_raw([0, 0]), &8);
    /// ```
    fn get_raw(&'a self, index: [usize; N]) -> &'a T;

    /// Iterate over all elements of the inner array, aligned to the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter().cloned().collect::<Vec<_>>(), &[
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8
    /// ]);
    /// ```
    fn iter(&'a self) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the inner array, ignoring the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// ```
    fn iter_raw(&'a self) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `index`, aligned to the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_index(0, 0).cloned().collect::<Vec<_>>(), &[
    ///     0, 3, 6
    /// ]);
    /// ```
    fn iter_index(&'a self, axis: usize, index: usize) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `index`, aligned to the offset
    /// in **contiguous** order.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_index_contiguous(0, 0).cloned().collect::<Vec<_>>(), &[
    ///     6, 0, 3
    /// ]);
    /// ```
    fn iter_index_contiguous(
        &'a self,
        axis: usize,
        index: usize,
    ) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `index`, ignoring the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_index_raw(0, 0).cloned().collect::<Vec<_>>(), &[
    ///     8,
    ///     2,
    ///     5
    /// ]);
    /// ```
    fn iter_index_raw(&'a self, axis: usize, index: usize) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `range`, aligned to the offset.
    /// This is equivalent to [`CircularIndex::iter_slice`] where all axis ranges are
    /// exhaustive with the exception of the specified `axis`.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_range(0, 1..3).cloned().collect::<Vec<_>>(), &[
    ///     1, 2,
    ///     4, 5,
    ///     7, 8
    /// ]);
    /// ```
    fn iter_range(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `range`, aligned to the offset
    /// in **contiguous** order. This is equivalent to [`CircularIndex::iter_slice_contiguous`]
    /// where all axis ranges are exhaustive with the exception of the specified `axis`.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_range(0, 1..3).cloned().collect::<Vec<_>>(), &[
    ///     1, 2,
    ///     4, 5,
    ///     7, 8
    /// ]);
    /// ```
    fn iter_range_contiguous(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the specified `axis` and `range`, ignoring the offset.
    /// This is equivalent to [`CircularIndex::iter_slice_raw`] where all axis
    /// ranges are exhaustive except for the specified `axis`.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    /// assert_eq!(array.iter_range_raw(0, 1..3).cloned().collect::<Vec<_>>(), &[
    ///     6, 7,
    ///     0, 1,
    ///     3, 4
    /// ]);
    /// ```
    fn iter_range_raw(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the given index `slice`, aligned to the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    ///
    /// assert_eq!(array.iter_slice([1..3, 1..3]).cloned().collect::<Vec<_>>(), &[
    ///     4, 5,
    ///     7, 8
    /// ]);
    /// ```
    fn iter_slice(&'a self, slice: [Range<usize>; N]) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the given index `slice`, aligned to the offset
    /// in **contiguous** order.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4
    /// ]);
    ///
    /// assert_eq!(array.iter_slice_contiguous([1..3, 1..3]).cloned().collect::<Vec<_>>(), &[
    ///     8, 7,
    ///     5, 4
    /// ]);
    /// ```
    fn iter_slice_contiguous(
        &'a self,
        slice: [Range<usize>; N],
    ) -> impl ExactSizeIterator<Item = &'a T>;

    /// Iterate over all elements of the given index `slice`, ignoring the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 1], vec![
    ///     8, 6, 7,
    ///     2, 0, 1,
    ///     5, 3, 4,
    /// ]);
    /// assert_eq!(array.iter_slice_raw([1..3, 1..3]).cloned().collect::<Vec<_>>(), &[
    ///     0, 1,
    ///     3, 4
    /// ]);
    /// ```
    fn iter_slice_raw(&'a self, slice: [Range<usize>; N]) -> impl ExactSizeIterator<Item = &'a T>;
}

impl<const N: usize, A, T> CircularArray<N, A, T> {
    /// Get the exhaustive spans of the array, aligned to the offset.
    pub(crate) fn spans(&self) -> [BoundSpan; N] {
        array::from_fn(|i| BoundSpan::new(self.offset[i], self.shape[i], self.shape[i]))
    }

    /// Get the raw exhaustive spans of the array.
    #[allow(dead_code)]
    pub(crate) fn spans_raw(&self) -> [UnboundSpan; N] {
        array::from_fn(|i| UnboundSpan::from_len(0, self.shape[i]))
    }

    /// Get the spans of the array, bound by the given `span` on the given `axis`,
    /// aligned to the offset.
    pub(crate) fn spans_axis_bound(&self, axis: usize, span: BoundSpan) -> [BoundSpan; N] {
        debug_assert!(span.len() <= self.shape[axis]);
        array::from_fn(|i| {
            if i == axis {
                (span + self.offset[i]) % self.shape[i]
            } else {
                BoundSpan::new(self.offset[i], self.shape[i], self.shape[i])
            }
        })
    }

    /// Get the raw spans of the array, bound by the given `span` on the given `axis`.
    pub(crate) fn spans_axis_bound_raw(&self, axis: usize, span: UnboundSpan) -> [UnboundSpan; N] {
        array::from_fn(|i| {
            if i == axis {
                span
            } else {
                UnboundSpan::from_len(0, self.shape[i])
            }
        })
    }
}

impl<'a, const N: usize, A: AsRef<[T]>, T: 'a> CircularIndex<'a, N, T> for CircularArray<N, A, T> {
    fn iter(&'a self) -> impl ExactSizeIterator<Item = &'a T> {
        let iter = IndexIterator::new_bound_contiguous(self.spans())
            .into_flat_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, self.len())
    }

    fn iter_raw(&'a self) -> impl ExactSizeIterator<Item = &'a T> {
        let iter = self.array.as_ref().iter();

        CircularArrayIterator::new(iter, self.len())
    }

    fn iter_index(&'a self, axis: usize, index: usize) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_index!(self, axis, index);

        let iter = IndexIterator::new_bound_contiguous(
            self.spans_axis_bound(axis, BoundSpan::new(index, 1, self.shape[axis])),
        )
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, self.slice_len(axis))
    }

    fn iter_index_contiguous(
        &'a self,
        axis: usize,
        index: usize,
    ) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_index!(self, axis, index);

        let iter = IndexIterator::new_bound_contiguous_ordered(
            self.spans_axis_bound(axis, BoundSpan::new(index, 1, self.shape[axis])),
        )
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, self.slice_len(axis))
    }

    fn iter_index_raw(&'a self, axis: usize, index: usize) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_index!(self, axis, index);

        let iter = IndexIterator::new_unbound(
            self.spans_axis_bound_raw(axis, UnboundSpan::from_len(index, 1)),
        )
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, self.slice_len(axis))
    }

    fn iter_range(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_range!(self, axis, range);

        let iter = IndexIterator::new_bound_contiguous(self.spans_axis_bound(
            axis,
            BoundSpan::new(range.start, range.len(), self.shape[axis]),
        ))
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, range.len() * self.slice_len(axis))
    }

    fn iter_range_contiguous(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_range!(self, axis, range);

        let iter = IndexIterator::new_bound_contiguous_ordered(self.spans_axis_bound(
            axis,
            BoundSpan::new(range.start, range.len(), self.shape[axis]),
        ))
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, range.len() * self.slice_len(axis))
    }

    fn iter_range_raw(
        &'a self,
        axis: usize,
        range: Range<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T> {
        assert_shape_index!(axis, N);
        assert_slice_range!(self, axis, range);

        let iter = IndexIterator::new_unbound(
            self.spans_axis_bound_raw(axis, UnboundSpan::from_len(range.start, range.len())),
        )
        .into_flat_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range]);

        CircularArrayIterator::new(iter, range.len() * self.slice_len(axis))
    }

    fn iter_slice(&'a self, slice: [Range<usize>; N]) -> impl ExactSizeIterator<Item = &'a T> {
        let spans = array::from_fn(|i| {
            let range = &slice[i];
            assert_slice_range!(self, i, range);

            BoundSpan::new(
                (range.start + self.offset[i]) % self.shape[i],
                range.len(),
                self.shape[i],
            ) % self.shape[i]
        });

        let iter = IndexIterator::new_bound_contiguous(spans)
            .into_flat_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range]);
        let len = spans.iter().map(|spans| spans.len()).product();

        CircularArrayIterator::new(iter, len)
    }

    fn iter_slice_contiguous(
        &'a self,
        slice: [Range<usize>; N],
    ) -> impl ExactSizeIterator<Item = &'a T> {
        let spans = array::from_fn(|i| {
            let range = &slice[i];
            assert_slice_range!(self, i, range);

            BoundSpan::new(
                (range.start + self.offset[i]) % self.shape[i],
                range.len(),
                self.shape[i],
            ) % self.shape[i]
        });

        let iter = IndexIterator::new_bound_contiguous_ordered(spans)
            .into_flat_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range]);
        let len = spans.iter().map(|spans| spans.len()).product();

        CircularArrayIterator::new(iter, len)
    }

    fn iter_slice_raw(&'a self, slice: [Range<usize>; N]) -> impl ExactSizeIterator<Item = &'a T> {
        let spans = array::from_fn(|i| {
            let range = &slice[i];
            assert_slice_range!(self, i, range);

            UnboundSpan::from_len(range.start, range.len())
        });

        let iter = IndexIterator::new_unbound(spans)
            .into_flat_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range]);
        let len = spans.iter().map(|spans| spans.len()).product();

        CircularArrayIterator::new(iter, len)
    }

    fn get(&'a self, mut index: [usize; N]) -> &'a T {
        index.iter_mut().enumerate().for_each(|(i, idx)| {
            assert_slice_index!(self, i, *idx);
            *idx = (*idx + self.offset[i]) % (self.shape[i]);
        });

        &self.array.as_ref()[self.strides.offset_index(index)]
    }

    fn get_raw(&'a self, index: [usize; N]) -> &'a T {
        &self.array.as_ref()[self.strides.offset_index(index)]
    }
}

impl<'a, const N: usize, A: AsRef<[T]>, T: 'a> Index<[usize; N]> for CircularArray<N, A, T> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        self.get(index)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::CircularArrayVec;

    #[test]
    fn iter() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter(shape, 0..shape.iter().product());
        m.offset = [1, 1, 1];

        #[rustfmt::skip]
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), [
            13, 14, 12,
            16, 17, 15,
            10, 11, 9,

            22, 23, 21,
            25, 26, 24,
            19, 20, 18, 

             4,  5,  3,
             7,  8,  6, 
             1,  2,  0
        ]);
        assert_eq!(m.iter().len(), 27);
    }

    #[test]
    fn iter_raw() {
        let shape = [3, 3, 3];
        let m = CircularArrayVec::from_iter(shape, 0..shape.iter().product());

        assert_eq!(
            m.iter_raw().cloned().collect::<Vec<_>>(),
            (0..3 * 3 * 3).collect::<Vec<_>>()
        );
        assert_eq!(m.iter().len(), 27);
    }

    #[test]
    fn iter_index() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, [1, 0, 0], 0..shape.iter().product());

        #[rustfmt::skip]
        assert_eq!(
            m.iter_index(0, 1).cloned().collect::<Vec<_>>(),
            [2, 5, 8, 11, 14, 17, 20, 23, 26]
        );
        assert_eq!(m.iter_index(0, 1).len(), 9);
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_index(1, 1).cloned().collect::<Vec<_>>(),
            [6, 7, 8, 15, 16, 17, 24, 25, 26]
        );
        assert_eq!(m.iter_index(1, 1).len(), 9);
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_index(2, 1).cloned().collect::<Vec<_>>(),
            [18, 19, 20, 21, 22, 23, 24, 25, 26]
        );
        assert_eq!(m.iter_index(2, 1).len(), 9);
        m.offset = [1, 1, 1];
        #[rustfmt::skip]
        assert_eq!(
            m.iter_index(0, 0).cloned().collect::<Vec<_>>(),
            [13, 16, 10, 22, 25, 19, 4, 7, 1]
        );
        assert_eq!(m.iter_index(0, 0).len(), 9);
    }

    #[test]
    fn iter_range() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, [1, 0, 0], 0..shape.iter().product());

        #[rustfmt::skip]
        assert_eq!(
            m.iter_range(0, 0..2).cloned().collect::<Vec<_>>(),
            [1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26]
        );
        assert_eq!(m.iter_range(0, 0..2).len(), 18);
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_range(1, 1..3).cloned().collect::<Vec<_>>(),
            [6, 7, 8, 0, 1, 2, 15, 16, 17, 9, 10, 11, 24, 25, 26, 18, 19, 20]
        );
        assert_eq!(m.iter_range(1, 1..3).len(), 18);
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_range(2, 1..2).cloned().collect::<Vec<_>>(),
            [18, 19, 20, 21, 22, 23, 24, 25, 26]
        );
        assert_eq!(m.iter_range(2, 1..2).len(), 9);
        m.offset = [1, 1, 1];
        #[rustfmt::skip]
        assert_eq!(m.iter_range(0, 1..4).cloned().collect::<Vec<_>>(), [
                14, 12, 13,
                17, 15, 16,
                11,  9, 10,

                23, 21, 22,
                26, 24, 25,
                20, 18, 19,

                 5,  3,  4,
                 8,  6,  7,
                 2,  0,  1
            ]);
        assert_eq!(m.iter_range(0, 1..4).len(), 27);
    }

    #[test]
    fn iter_range_raw() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, [1, 0, 0], 0..shape.iter().product());

        #[rustfmt::skip]
        assert_eq!(
            m.iter_range_raw(0, 0..2).cloned().collect::<Vec<_>>(),
            [0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25]
        );
        assert_eq!(m.iter_range_raw(0, 0..2).len(), 18);
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_range_raw(1, 1..3).cloned().collect::<Vec<_>>(),
            [3, 4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 21, 22, 23, 24, 25, 26]
        );
        assert_eq!(m.iter_range_raw(1, 1..3).len(), 18);
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_range_raw(2, 1..2).cloned().collect::<Vec<_>>(),
            [9, 10, 11, 12, 13, 14, 15, 16, 17]
        );
        assert_eq!(m.iter_range_raw(2, 1..2).len(), 9);
        m.offset = [1, 1, 1];
        #[rustfmt::skip]
        assert_eq!(m.iter_range_raw(0, 1..3).cloned().collect::<Vec<_>>(), [
             1,  2,
             4,  5,
             7,  8,
            
            10, 11,
            13, 14,
            16, 17,
            
            19, 20,
            22, 23,
            25, 26            
            ]);
        assert_eq!(m.iter_range_raw(0, 1..3).len(), 18);
    }

    #[test]
    fn iter_slice() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, [1, 1, 1], 0..shape.iter().product());

        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).cloned().collect::<Vec<_>>(), &[13]);
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).len(), 1);
        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).cloned().collect::<Vec<_>>(), &[
            22, 23, 21,
            25, 26, 24,
            19, 20, 18
        ]);
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).len(), 9);

        m.offset = [2, 2, 2];

        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).cloned().collect::<Vec<_>>(), &[26]);
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).len(), 1);
        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).cloned().collect::<Vec<_>>(), &[
            8, 6, 7,
            2, 0, 1,
            5, 3, 4
        ]);
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).len(), 9);
    }

    #[test]
    fn get() {
        let shape = [3, 3, 3];
        let m = CircularArrayVec::from_iter_offset(shape, [1, 1, 1], 0..shape.iter().product());

        assert_eq!(m.get([0, 0, 0]), &13);
        assert_eq!(m.get([1, 1, 1]), &26);
        assert_eq!(m.get([2, 2, 2]), &0);
    }

    #[test]
    fn get_raw() {
        let m = CircularArray::new([3, 3, 3], (0..3 * 3 * 3).collect::<Vec<_>>());

        assert_eq!(m.get_raw([0, 0, 0]), &0);
        assert_eq!(m.get_raw([1, 1, 1]), &13);
        assert_eq!(m.get_raw([2, 2, 2]), &26);
    }
}
