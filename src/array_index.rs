use std::array;
use std::ops::{Index, IndexMut, Range};

use crate::array_iter::{CircularIterator, RawIndexAdaptor};
use crate::span::BoundSpan;
use crate::CircularArray;

/// Methods for retrieving elements from the array.
pub trait CircularArrayIndex<'a, const N: usize, T: 'a> {
    /// Iterate over all elements of the inner array, aligned to the offset.
    fn iter(&'a self) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of the inner array.
    fn iter_raw(&'a self) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of `index` for the given `axis` aligned to the offset.
    fn iter_index(&'a self, axis: usize, index: usize) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of `index` for the given `axis`.
    fn iter_index_raw(&'a self, axis: usize, index: usize) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of the given index `range` for the given `axis`
    /// aligned to the offset.
    fn iter_range(&'a self, axis: usize, range: Range<usize>) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of the given index `range` for the given `axis`.
    fn iter_range_raw(&'a self, axis: usize, range: Range<usize>) -> impl Iterator<Item = &'a T>;

    /// Iterate over all elements of the given index `slice`.
    fn iter_slice(&'a self, slice: [Range<usize>; N]) -> impl Iterator<Item = &'a T>;

    /// Get a reference to the element at the given index, aligned to the offset.
    fn get(&'a self, index: [usize; N]) -> &'a T;

    /// Get a reference to the element at the given index.
    fn get_raw(&'a self, index: [usize; N]) -> &'a T;
}

/// Methods for retrieving mutable references to elements of the array.
pub trait CircularArrayIndexMut<'a, const N: usize, T: 'a> {
    /// Get a mutable reference to the element at the given index, aligned to the offset.
    fn get_mut(&mut self, index: [usize; N]) -> &mut T;

    /// Get a mutable reference to the element at the given index.
    fn get_mut_raw(&mut self, index: [usize; N]) -> &mut T;
}

impl<const N: usize, A, T> CircularArray<N, A, T> {
    /// Get the exhaustive spans of the array, aligned to the offset.
    pub(crate) fn spans(&self) -> [BoundSpan; N] {
        array::from_fn(|i| BoundSpan::new(self.offset[i], self.shape[i], self.shape[i]))
    }

    /// Get the raw exhaustive spans of the array.
    #[allow(dead_code)]
    pub(crate) fn spans_raw(&self) -> [BoundSpan; N] {
        array::from_fn(|i| BoundSpan::new(0, self.shape[i], self.shape[i]))
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
    pub(crate) fn spans_axis_bound_raw(&self, axis: usize, span: BoundSpan) -> [BoundSpan; N] {
        array::from_fn(|i| {
            if i == axis {
                span
            } else {
                BoundSpan::new(0, self.shape[i], self.shape[i])
            }
        })
    }
}

impl<'a, const N: usize, A: AsRef<[T]>, T: 'a> CircularArrayIndex<'a, N, T>
    for CircularArray<N, A, T>
{
    fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        CircularIterator::new(self.spans())
            .into_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range])
    }

    fn iter_raw(&'a self) -> impl Iterator<Item = &'a T> {
        self.array.as_ref().iter()
    }

    fn iter_index(&'a self, axis: usize, index: usize) -> impl Iterator<Item = &'a T> {
        assert!(axis < N);
        assert!(index < self.shape[axis]);

        CircularIterator::new(
            self.spans_axis_bound(axis, BoundSpan::new(index, 1, self.shape[axis])),
        )
        .into_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range])
    }

    fn iter_range(&'a self, axis: usize, range: Range<usize>) -> impl Iterator<Item = &'a T> {
        assert!(axis < N);
        assert!(range.len() > 0);
        assert!(range.len() <= self.shape[axis]);

        CircularIterator::new(self.spans_axis_bound(
            axis,
            BoundSpan::new(range.start, range.len(), self.shape[axis]),
        ))
        .into_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range])
    }

    fn iter_range_raw(&'a self, axis: usize, range: Range<usize>) -> impl Iterator<Item = &'a T> {
        assert!(axis < N);
        assert!(range.len() > 0);
        assert!(range.len() <= self.shape[axis]);

        CircularIterator::new(self.spans_axis_bound_raw(
            axis,
            BoundSpan::new(range.start, range.len(), self.shape[axis]),
        ))
        .into_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range])
    }

    fn iter_slice(&'a self, slice: [Range<usize>; N]) -> impl Iterator<Item = &'a T> {
        let spans = array::from_fn(|i| {
            let range = &slice[i];
            assert!(range.len() > 0);
            assert!(range.len() <= self.shape[i]);

            BoundSpan::new(
                (range.start + self.offset[i]) % self.shape[i],
                range.len(),
                self.shape[i],
            ) % self.shape[i]
        });

        CircularIterator::new(spans)
            .into_ranges(&self.strides)
            .flat_map(|range| &self.array.as_ref()[range])
    }

    fn iter_index_raw(&'a self, axis: usize, index: usize) -> impl Iterator<Item = &'a T> {
        assert!(axis < N);
        assert!(index < self.shape[axis]);

        CircularIterator::new(
            self.spans_axis_bound_raw(axis, BoundSpan::new(index, 1, self.shape[axis])),
        )
        .into_ranges(&self.strides)
        .flat_map(|range| &self.array.as_ref()[range])
    }

    fn get(&'a self, mut index: [usize; N]) -> &'a T {
        index.iter_mut().enumerate().for_each(|(i, idx)| {
            assert!(*idx < self.shape[i]);
            *idx = (*idx + self.offset[i]) % (self.shape[i]);
        });

        &self.array.as_ref()[self.strides.apply_to_index(index)]
    }

    fn get_raw(&'a self, index: [usize; N]) -> &'a T {
        &self.array.as_ref()[self.strides.apply_to_index(index)]
    }
}

impl<'a, const N: usize, A: AsMut<[T]>, T: 'a> CircularArrayIndexMut<'a, N, T>
    for CircularArray<N, A, T>
{
    fn get_mut(&mut self, mut index: [usize; N]) -> &mut T {
        index.iter_mut().enumerate().for_each(|(i, idx)| {
            assert!(*idx < self.shape[i]);
            *idx = (*idx + self.offset[i]) % (self.shape[i]);
        });

        &mut self.array.as_mut()[self.strides.apply_to_index(index)]
    }

    fn get_mut_raw(&mut self, index: [usize; N]) -> &mut T {
        &mut self.array.as_mut()[self.strides.apply_to_index(index)]
    }
}

impl<'a, const N: usize, A: AsRef<[T]>, T: 'a> Index<[usize; N]> for CircularArray<N, A, T> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        self.get(index)
    }
}

impl<'a, const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: 'a> IndexMut<[usize; N]>
    for CircularArray<N, A, T>
{
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        self.get_mut(index)
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
    }

    #[test]
    fn iter_raw() {
        let shape = [3, 3, 3];
        let m = CircularArrayVec::from_iter(shape, 0..shape.iter().product());

        assert_eq!(
            m.iter_raw().cloned().collect::<Vec<_>>(),
            (0..3 * 3 * 3).collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_index() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 0, 0]);

        #[rustfmt::skip]
        assert_eq!(
            m.iter_index(0, 1).cloned().collect::<Vec<_>>(),
            [2, 5, 8, 11, 14, 17, 20, 23, 26]
        );
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_index(1, 1).cloned().collect::<Vec<_>>(),
            [6, 7, 8, 15, 16, 17, 24, 25, 26]
        );
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_index(2, 1).cloned().collect::<Vec<_>>(),
            [18, 19, 20, 21, 22, 23, 24, 25, 26]
        );
        m.offset = [1, 1, 1];
        #[rustfmt::skip]
        assert_eq!(
            m.iter_index(0, 0).cloned().collect::<Vec<_>>(),
            [13, 16, 10, 22, 25, 19, 4, 7, 1]
        );
    }

    #[test]
    fn iter_range() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 0, 0]);

        #[rustfmt::skip]
        assert_eq!(
            m.iter_range(0, 0..2).cloned().collect::<Vec<_>>(),
            [1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26]
        );
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_range(1, 1..3).cloned().collect::<Vec<_>>(),
            [6, 7, 8, 0, 1, 2, 15, 16, 17, 9, 10, 11, 24, 25, 26, 18, 19, 20]
        );
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_range(2, 1..2).cloned().collect::<Vec<_>>(),
            [18, 19, 20, 21, 22, 23, 24, 25, 26]
        );
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
    }

    #[test]
    fn iter_range_raw() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 0, 0]);

        #[rustfmt::skip]
        assert_eq!(
            m.iter_range_raw(0, 0..2).cloned().collect::<Vec<_>>(),
            [0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25]
        );
        m.offset = [0, 1, 0];
        assert_eq!(
            m.iter_range_raw(1, 1..3).cloned().collect::<Vec<_>>(),
            [3, 4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 21, 22, 23, 24, 25, 26]
        );
        m.offset = [0, 0, 1];
        assert_eq!(
            m.iter_range_raw(2, 1..2).cloned().collect::<Vec<_>>(),
            [9, 10, 11, 12, 13, 14, 15, 16, 17]
        );
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
    }

    #[test]
    fn iter_slice() {
        let shape = [3, 3, 3];
        let mut m = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 1, 1]);

        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).cloned().collect::<Vec<_>>(), &[13]);
        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).cloned().collect::<Vec<_>>(), &[
            22, 23, 21,
            25, 26, 24,
            19, 20, 18
        ]);

        m.offset = [2, 2, 2];

        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..1, 0..1, 0..1]).cloned().collect::<Vec<_>>(), &[26]);
        #[rustfmt::skip]
        assert_eq!(m.iter_slice([0..3, 0..3, 1..2]).cloned().collect::<Vec<_>>(), &[
            8, 6, 7,
            2, 0, 1,
            5, 3, 4
        ]);
    }

    #[test]
    fn get() {
        let shape = [3, 3, 3];
        let m = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 1, 1]);

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
