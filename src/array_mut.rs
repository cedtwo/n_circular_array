use std::ops::{IndexMut, Range};

use crate::index::RawIndexAdaptor;
use crate::index_iter::IndexIterator;
use crate::span::{BoundSpan, UnboundSpan};
use crate::CircularArray;

/// Mutating `CircularArray` operations.
pub trait CircularMut<'a, const N: usize, T> {
    /// Get a mutable reference to the element at the given index, aligned to the
    /// offset.
    /// 
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     2, 0, 1,
    ///     5, 3, 4,
    ///     8, 6, 7,
    /// ]);
    /// assert_eq!(array.get_mut([0, 0]), &mut 0);
    /// ```
    fn get_mut(&mut self, index: [usize; N]) -> &mut T;

    /// Get a mutable reference to the element at the given index. This does **not**
    /// account for the offset. See [`CircularArray::offset`].
    /// 
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     2, 0, 1,
    ///     5, 3, 4,
    ///     8, 6, 7,
    /// ]);
    /// assert_eq!(array.get_mut_raw([0, 0]), &mut 2);
    /// ```
    fn get_mut_raw(&mut self, index: [usize; N]) -> &mut T;

    /// Push elements to the front of the given `axis`, aligned to the offset.
    /// Elements must be an exact multiple of the slice size for the given `axis`.
    /// See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_front(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     11,  9, 10,
    ///      3,  4,  5,
    ///      6,  7,  8,
    /// ]);
    /// ```
    fn push_front(&'a mut self, axis: usize, el: &'a [T]);

    /// Push elements to the front of the given `axis`, aligned to the offset.
    /// Elements must be an exact multiple of the slice size for the given `axis`.
    /// See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_front_iter(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     11,  9, 10,
    ///      3,  4,  5,
    ///      6,  7,  8,
    /// ]);
    /// ```
    fn push_front_iter<'b, I>(&'a mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b;

    /// Push elements to the front of the given `axis`, taking into account only
    /// the offset of the given `axis`. Elements must be an exact multiple of
    /// the slice size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_front_raw(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     9, 10, 11,
    ///     3,  4,  5,
    ///     6,  7,  8,
    /// ]);
    /// ```
    fn push_front_raw(&'a mut self, axis: usize, el: &'a [T]);

    /// Push elements to the front of the given `axis`, taking into account the
    /// offsets of **all** axes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_front_raw_iter(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     9, 10, 11,
    ///     3,  4,  5,
    ///     6,  7,  8,
    /// ]);
    /// ```
    fn push_front_raw_iter<'b, I>(&'a mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b;

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** exes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_back(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///      0,  1,  2,
    ///      3,  4,  5,
    ///     11,  9, 10,
    /// ]);
    /// ```
    fn push_back(&'a mut self, axis: usize, el: &'a [T]);

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** exes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_back_iter(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///      0,  1,  2,
    ///      3,  4,  5,
    ///     11,  9, 10,
    /// ]);
    /// ```
    fn push_back_iter<'b, I>(&'a mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b;

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** axes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_back_raw(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     0,  1,  2,
    ///     3,  4,  5,
    ///     9, 10, 11,
    /// ]);
    /// ```
    fn push_back_raw(&'a mut self, axis: usize, el: &'a [T]);

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** axes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut};
    /// let mut array = CircularArray::new_offset([3, 3], [1, 0], vec![
    ///     0, 1, 2,
    ///     3, 4, 5,
    ///     6, 7, 8,
    /// ]);
    ///
    /// array.push_back_raw_iter(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     0,  1,  2,
    ///     3,  4,  5,
    ///     9, 10, 11,
    /// ]);
    /// ```
    fn push_back_raw_iter<'b, I>(&'a mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b;

    /// Translate the array by `n` on the given `axis`, inserting elements to the
    /// **front** of the array.
    ///
    /// Requires specifying the array `origin` of the `CircularArray` relative to
    /// translation. `N` dimensional index range (`[Range<usize>; N]`) will be passed
    /// to the `el_fn` for slicing a source buffer to retrieve the new elements.
    /// Note that the caler should ensure that a translation of `n` is within the
    /// *source* array bounds prior to calling this function.
    ///
    /// In the following example, we pre-calculate the [`crate::Strides`] of
    /// the *source* array to flatten the `N` dimensional index into a contiguous
    /// range (requires feature flag `strides`). Alternatively, the index range can
    /// be passed to 3rd party crates for slicing operations.
    ///
    /// ```
    /// # #[cfg(feature = "strides")] {
    /// # use std::ops::Range;
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut, Strides};
    /// // A [5, 5] source array.
    /// let src = [
    ///      0,  1,  2,  3,  4,
    ///      5,  6,  7,  8,  9,
    ///     10, 11, 12, 13, 14,
    ///     15, 16, 17, 18, 19,
    ///     20, 21, 22, 23, 24,
    /// ];
    /// // Strides used for flattening `N` dimensional indices.
    /// let src_strides = Strides::new(&[5, 5]);
    ///
    /// // Slice function.
    /// let el_fn = |mut index: [Range<usize>; 2]| {
    ///     &src[src_strides.flatten_range(index)]
    /// };
    ///
    /// // A [3, 3] circular array positioned at `[0, 0]`.
    /// let mut origin = [0, 0];
    /// let mut dst = CircularArray::new([3, 3], vec![
    ///      0,  1,  2,
    ///      5,  6,  7,
    ///     10, 11, 12
    /// ]);
    ///
    /// // Translate by 2 on axis 0 (Pushes 2 columns to front of axis 0).
    /// let (axis, n) = (0, 2);
    /// dst.translate_front(axis, n, origin, el_fn);
    /// origin[axis] += n as usize;
    ///
    /// assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
    ///      2,  3,  4,
    ///      7,  8,  9,
    ///     12, 13, 14,
    /// ]);
    ///
    /// // Translate by 1 on axis 1 (Pushes 1 row to front of axis 1).
    /// let (axis, n) = (1, 1);
    /// dst.translate_front(axis, n, origin, el_fn);
    /// origin[axis] += n as usize;
    ///
    /// assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
    ///      7,  8,  9,
    ///     12, 13, 14,
    ///     17, 18, 19,
    /// ]);
    /// # }
    /// ```
    fn translate_front<'b, F>(&'a mut self, axis: usize, n: usize, origin: [usize; N], el_fn: F)
    where
        T: 'b,
        F: FnMut([Range<usize>; N]) -> &'b [T];

    /// Translate the array by `-n` on the given `axis`, inserting elements to the
    /// **back** of the array.
    ///
    /// Requires specifying the array `origin` of the `CircularArray` relative to
    /// translation. `N` dimensional index range (`[Range<usize>; N]`) will be passed
    /// to the `el_fn` for slicing a source buffer to retrieve the new elements.
    /// Note that the caler should ensure that a translation of `n` is within the
    /// *source* array bounds prior to calling this function.
    ///
    /// In the following example, we pre-calculate the [`crate::Strides`] of
    /// the *source* array to flatten the `N` dimensional index into a contiguous
    /// range (requires feature flag `strides`). Alternatively, the index range can
    /// be passed to 3rd party crates for slicing operations.
    ///
    /// ```
    /// # #[cfg(feature = "strides")] {
    /// # use std::ops::Range;
    /// # use n_circular_array::{CircularArray, CircularIndex, CircularMut, Strides};
    /// // A [5, 5] source array.
    /// let src = [
    ///      0,  1,  2,  3,  4,
    ///      5,  6,  7,  8,  9,
    ///     10, 11, 12, 13, 14,
    ///     15, 16, 17, 18, 19,
    ///     20, 21, 22, 23, 24,
    /// ];
    /// // Strides used for flattening `N` dimensional indices.
    /// let src_strides = Strides::new(&[5, 5]);
    ///
    /// // Slice function.
    /// let el_fn = |mut index: [Range<usize>; 2]| {
    ///     &src[src_strides.flatten_range(index)]
    /// };
    ///
    /// // A [3, 3] circular array positioned at `[2, 2]`.
    /// let mut origin = [2, 2];
    /// let mut dst = CircularArray::new([3, 3], vec![
    ///     12, 13, 14,
    ///     17, 18, 19,
    ///     22, 23, 24,
    /// ]);
    ///
    /// // Translate by -2 on axis 0 (Pushes 2 columns to back of axis 0).
    /// let (axis, n) = (0, 2);
    /// dst.translate_back(axis, n, origin, el_fn);
    /// origin[axis] -= n;
    ///
    /// assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
    ///     10, 11, 12,
    ///     15, 16, 17,
    ///     20, 21, 22,
    /// ]);
    ///
    /// // Translate by -1 on axis 1 (Pushes 1 row to back of axis 1).
    /// let (axis, n) = (1, 1);
    /// dst.translate_back(axis, n, origin, el_fn);
    /// origin[axis] -= n;
    ///
    /// assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
    ///      5,  6,  7,
    ///     10, 11, 12,
    ///     15, 16, 17,
    /// ]);
    /// # }
    /// ```
    fn translate_back<'b, F>(&'a mut self, axis: usize, n: usize, origin: [usize; N], el_fn: F)
    where
        T: 'b,
        F: FnMut([Range<usize>; N]) -> &'b [T];
}

impl<const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: Clone> CircularArray<N, A, T> {
    /// Push a contiguous slice of elements into the array.
    fn push<'a>(&'a mut self, spans: impl RawIndexAdaptor<'a, N>, mut el: &[T]) {
        let iter = spans.into_flat_ranges(&self.strides);

        for slice_range in iter {
            let len = slice_range.len();
            self.array.as_mut()[slice_range].clone_from_slice(&el[..len]);
            (_, el) = el.split_at(len);
        }
    }

    /// Push an iterator of elements into the array.
    fn push_iter<'a, 'b>(
        &'a mut self,
        spans: impl RawIndexAdaptor<'a, N>,
        mut el: impl Iterator<Item = &'b T>,
    ) where
        T: 'b,
    {
        let iter = spans.into_flat_ranges(&self.strides);

        for slice_range in iter {
            let len = slice_range.len();
            self.array.as_mut()[slice_range]
                .iter_mut()
                .zip((&mut el).take(len))
                .for_each(|(a, b)| *a = b.clone());
        }
    }

    /// Push slice(s) retrieved from the given `el_fn` into the array.
    fn translate<'a, 'b, F>(
        &'a mut self,
        src_spans: impl RawIndexAdaptor<'a, N>,
        dst_spans: impl RawIndexAdaptor<'a, N>,
        origin: [usize; N],
        mut el_fn: F,
    ) where
        T: 'b,
        F: FnMut([Range<usize>; N]) -> &'b [T],
    {
        let src_iter = src_spans.into_ranges(origin);
        let mut dst_iter = dst_spans.into_flat_ranges(&self.strides);

        for mut src_slice in src_iter.map(|range| el_fn(range)) {
            let mut src_len = src_slice.len();

            while src_len > 0 {
                let dst_range = dst_iter.next().expect("Misaligned src/dst ranges");
                let dst_len = dst_range.len();

                self.array.as_mut()[dst_range].clone_from_slice(&src_slice[..dst_len]);
                (_, src_slice) = src_slice.split_at(dst_len);
                src_len = src_slice.len();
            }
        }
    }

    /// Increment the offset by `n` on the given `axis`.
    pub(crate) fn incr_offset(&mut self, axis: usize, n: usize) {
        self.offset[axis] = (self.offset[axis] + n) % self.shape()[axis];
    }

    /// Decrement the offset by `n` on the given `axis`.
    pub(crate) fn decr_offset(&mut self, axis: usize, n: usize) {
        self.offset[axis] = (self.shape()[axis] + self.offset[axis] - n) % self.shape()[axis];
    }
}

impl<'a, const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: Clone + 'a> CircularMut<'a, N, T>
    for CircularArray<N, A, T>
{
    fn get_mut(&mut self, mut index: [usize; N]) -> &mut T {
        index.iter_mut().enumerate().for_each(|(i, idx)| {
            assert_slice_index!(self, i, *idx);
            *idx = (*idx + self.offset[i]) % (self.shape[i]);
        });

        &mut self.array.as_mut()[self.strides.offset_index(index)]
    }

    fn get_mut_raw(&mut self, index: [usize; N]) -> &mut T {
        &mut self.array.as_mut()[self.strides.offset_index(index)]
    }

    fn push_front(&'a mut self, axis: usize, el: &'a [T]) {
        let el_len = el.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            // Copy/Clone into array, and clear offset.
            if n == self.shape()[axis] {
                self.array.as_mut().clone_from_slice(el);
                self.offset = [0; N];
            // Copy/Clone into slices, and increment offset.
            } else {
                let spans = self.spans_axis_bound(axis, BoundSpan::new(0, n, self.shape[axis]));

                self.push(IndexIterator::new_bound_contiguous(spans), el);
                self.incr_offset(axis, n);
            }
        }
    }

    fn push_front_iter<'b, I>(&mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b,
    {
        let iter = el.into_iter();
        let el_len = iter.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            let spans = self.spans_axis_bound(axis, BoundSpan::new(0, n, self.shape[axis]));

            self.push_iter(IndexIterator::new_bound_contiguous(spans), iter);
            self.incr_offset(axis, n);
        }
    }



    fn push_front_raw(&'a mut self, axis: usize, el: &'a [T]) {
        let el_len = el.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            // Copy/Clone into array, and clear offset.
            if n == self.shape()[axis] {
                self.array.as_mut().clone_from_slice(el);
                self.offset = [0; N];
            // Copy/Clone into slices, and increment offset.
            } else {
                let spans = self.spans_axis_bound_raw(axis, UnboundSpan::from_len(0, n));

                self.push(IndexIterator::new_unbound(spans), el);
                self.incr_offset(axis, n);
            }
        }
    }

    fn push_front_raw_iter<'b, I>(&mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b,
    {
        let iter = el.into_iter();
        let el_len = iter.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            let spans = self.spans_axis_bound_raw(axis, UnboundSpan::from_len(0, n));

            self.push_iter(IndexIterator::new_unbound(spans), iter);
            self.incr_offset(axis, n);
        }
    }

    fn push_back(&'a mut self, axis: usize, el: &'a [T]) {
        let el_len = el.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            // Copy/Clone into array, and clear offset.
            if n == self.shape()[axis] {
                self.array.as_mut().clone_from_slice(el);
                self.offset = [0; N];
            // Copy/Clone into slices, and increment offset.
            } else {
                let span = BoundSpan::new(self.shape[axis] - n, n, self.shape[axis]);
                let spans = self.spans_axis_bound(axis, span);

                self.push(IndexIterator::new_bound_contiguous(spans), el);
                self.decr_offset(axis, n);
            }
        }
    }

    fn push_back_iter<'b, I>(&mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b,
    {
        let iter = el.into_iter();
        let el_len = iter.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            let span = BoundSpan::new(self.shape[axis] - n, n, self.shape[axis]);
            let spans = self.spans_axis_bound(axis, span);

            self.push_iter(IndexIterator::new_bound_contiguous(spans), iter);
            self.decr_offset(axis, n);
        }
    }



    fn push_back_raw(&'a mut self, axis: usize, el: &'a [T]) {
        let el_len = el.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            // Copy/Clone into array, and clear offset.
            if n == self.shape()[axis] {
                self.array.as_mut().clone_from_slice(el);
                self.offset = [0; N];
            // Copy/Clone into slices, and increment offset.
            } else {
                let span = UnboundSpan::from_len((self.shape[axis] - n) % self.shape[axis], n);
                let spans = self.spans_axis_bound_raw(axis, span);

                self.push(IndexIterator::new_unbound(spans), el);
                self.decr_offset(axis, n);
            }
        }
    }

    fn push_back_raw_iter<'b, I>(&mut self, axis: usize, el: I)
    where
        I: IntoIterator<IntoIter: ExactSizeIterator, Item = &'b T>,
        T: 'b,
    {
        let iter = el.into_iter();
        let el_len = iter.len();
        let slice_len = self.slice_len(axis);
        let n = el_len / slice_len;

        assert_element_len!(axis, el_len, slice_len);
        assert_slice_len!(self, axis, n);

        if n != 0 {
            let span = UnboundSpan::from_len((self.shape[axis] - n) % self.shape[axis], n);
            let spans = self.spans_axis_bound_raw(axis, span);

            self.push_iter(IndexIterator::new_unbound(spans), iter);
            self.decr_offset(axis, n);
        }   
    }
    
    fn translate_front<'b, F>(
        &'a mut self,
        axis: usize,
        mut n: usize,
        mut origin: [usize; N],
        mut el_fn: F,
    ) where
        T: 'b,
        F: FnMut([Range<usize>; N]) -> &'b [T],
    {
        if n != 0 {
            origin[axis] += self.shape[axis] + n - n.min(self.shape[axis]);
            n = n.min(self.shape[axis]);

            // Copy/Clone equal length slices.
            if n >= self.shape()[axis] {
                let src_span = UnboundSpan::from_len(0, n);

                let src = IndexIterator::new_unbound(self.spans_axis_bound_raw(axis, src_span));
                let dst = IndexIterator::new_unbound(self.spans_raw());

                src.into_ranges(origin)
                    .zip(dst.into_flat_ranges(&self.strides))
                    .for_each(|(src, dst)| {
                        self.array.as_mut()[dst].clone_from_slice(el_fn(src));
                    });
                self.offset = [0; N];
            // Copy/Clone (possibly) divergent length slices.
            } else {
                let src_span = UnboundSpan::from_len(0, n);
                let dst_span = BoundSpan::new(0, n, self.shape[axis]);

                let src = IndexIterator::new_unbound(self.spans_axis_bound_raw(axis, src_span));
                let dst = IndexIterator::new_bound(self.spans_axis_bound(axis, dst_span));

                self.translate(src, dst, origin, el_fn);
                self.incr_offset(axis, n);
            }
        }
    }

    fn translate_back<'b, F>(
        &'a mut self,
        axis: usize,
        mut n: usize,
        mut origin: [usize; N],
        mut el_fn: F,
    ) where
        T: 'b,
        F: FnMut([Range<usize>; N]) -> &'b [T],
    {
        assert_origin_bounds!(axis, origin, -n);

        if n != 0 {
            origin[axis] -= n;
            n = n.min(self.shape[axis]);

            // Copy/Clone equal length slices.
            if n >= self.shape()[axis] {
                let src_span = UnboundSpan::from_len(0, n);

                let src = IndexIterator::new_unbound(self.spans_axis_bound_raw(axis, src_span));
                let dst = IndexIterator::new_unbound(self.spans_raw());

                src.into_ranges(origin)
                    .zip(dst.into_flat_ranges(&self.strides))
                    .for_each(|(src, dst)| {
                        self.array.as_mut()[dst].clone_from_slice(el_fn(src));
                    });
                self.offset = [0; N];
            // Copy/Clone (possibly) divergent length slices.
            } else {
                let src_span = UnboundSpan::from_len(0, n);
                let dst_span = BoundSpan::new(self.shape[axis] - n, n, self.shape[axis]);

                let src = IndexIterator::new_unbound(self.spans_axis_bound_raw(axis, src_span));
                let dst = IndexIterator::new_bound(self.spans_axis_bound(axis, dst_span));

                self.translate(src, dst, origin, el_fn);
                self.decr_offset(axis, n);
            }
        }
    }
}

impl<'a, const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: Clone + 'a> IndexMut<[usize; N]>
    for CircularArray<N, A, T>
{
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::array_index::CircularIndex;
    use crate::CircularArrayVec;

    macro_rules! push_front {
        (
            $m:ident,
            $axis:literal,
            $payload:expr
        ) => {
            let n = $payload.len() / $m.slice_len($axis);
            $m.push_front($axis, $payload);

            let slice = IndexIterator::new_bound($m.spans_axis_bound(
                $axis,
                BoundSpan::new($m.shape()[$axis] - n, n, $m.shape()[$axis]),
            ))
            .into_flat_indices(&$m.strides)
            .map(|i| $m.array[i].clone())
            .collect::<Vec<_>>();

            assert_eq!(slice, $payload);
        };
    }

    #[test]
    fn push_front() {
        let shape = [4, 3, 2];
        let n = shape.iter().product::<usize>();
        let input = CircularArrayVec::from_iter(shape, n..n * 2);

        // Axis 0.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_front!(m, 0, input.iter_index(0, 0).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[0], 1);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             24,  1,  2,  3, 
             28,  5,  6,  7, 
             32,  9, 10, 11, 

             36, 13, 14, 15, 
             40, 17, 18, 19, 
             44, 21, 22, 23, 
        ]);
        #[rustfmt::skip]
        push_front!(m, 0, input.iter_range(0, 1..4).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[0], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // Axis 1.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_front!(m, 1, input.iter_index(1, 0).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[1], 1);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             24, 25, 26, 27, 
              4,  5,  6,  7, 
              8,  9, 10, 11, 

             36, 37, 38, 39, 
             16, 17, 18, 19, 
             20, 21, 22, 23, 
        ]);
        #[rustfmt::skip]
        push_front!(m, 1, input.iter_range(1, 1..3).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[1], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // Axis 2.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_front!(m, 2, input.iter_index(2, 0).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[2], 1);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             24, 25, 26, 27, 
             28, 29, 30, 31, 
             32, 33, 34, 35, 

             12, 13, 14, 15, 
             16, 17, 18, 19, 
             20, 21, 22, 23, 
        ]);
        #[rustfmt::skip]
        push_front!(m, 2, input.iter_range(2, 1..2).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[2], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // All axis.
        let mut m = CircularArrayVec::from_iter(shape, (0..n).map(|_| "___".to_string()));
        #[rustfmt::skip]
        push_front!(m, 0, (0..m.slice_len(0)).map(|i| format!("A{:02}", i)).collect::<Vec<_>>().as_slice());
        #[rustfmt::skip]
        push_front!(m, 1, (0..m.slice_len(1)).map(|i| format!("B{:02}", i)).collect::<Vec<_>>().as_slice());
        #[rustfmt::skip]
        push_front!(m, 2, (0..m.slice_len(2)).map(|i| format!("C{:02}", i)).collect::<Vec<_>>().as_slice());

        #[rustfmt::skip]
        assert_eq!(m.array, &[
            "C11", "C08", "C09", "C10",
            "C03", "C00", "C01", "C02",
            "C07", "C04", "C05", "C06",

            "B07", "B04", "B05", "B06",
            "A04", "___", "___", "___",
            "A05", "___", "___", "___"            
            ]
        );
    }

    macro_rules! push_back {
        (
            $m:ident,
            $axis:literal,
            $payload:expr
        ) => {
            let n = $payload.len() / $m.slice_len($axis);
            $m.push_back($axis, $payload);

            let slice = IndexIterator::new_bound(
                $m.spans_axis_bound($axis, BoundSpan::new(0, n, $m.shape()[$axis])),
            )
            .into_flat_indices(&$m.strides)
            .map(|i| $m.array[i].clone())
            .collect::<Vec<_>>();

            assert_eq!(slice, $payload);
        };
    }

    #[test]
    fn push_back() {
        let shape = [4, 3, 2];
        let n = shape.iter().product::<usize>();
        let input = CircularArrayVec::from_iter(shape, n..n * 2);

        // Axis 0.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_back!(m, 0, input.iter_index(0, 3).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[0], 3);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             0,  1,  2, 27,
             4,  5,  6, 31,
             8,  9, 10, 35,
            12, 13, 14, 39,
            16, 17, 18, 43,
            20, 21, 22, 47
        ]);
        #[rustfmt::skip]
        push_back!(m, 0, input.iter_range(0, 0..3).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[0], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // Axis 1.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_back!(m, 1, input.iter_index(1, 2).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[1], 2);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             0,  1,  2,  3,
             4,  5,  6,  7,
            32, 33, 34, 35,

            12, 13, 14, 15,
            16, 17, 18, 19,
            44, 45, 46, 47            
        ]);
        #[rustfmt::skip]
        push_back!(m, 1, input.iter_range(1, 0..2).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[1], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // Axis 2.
        let mut m = CircularArrayVec::from_iter(shape, 0..n);
        #[rustfmt::skip]
        push_back!(m, 2, input.iter_index(2, 1).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[2], 1);
        #[rustfmt::skip]
        assert_eq!(m.array, &[
             0,  1,  2,  3,
             4,  5,  6,  7,
             8,  9, 10, 11,

            36, 37, 38, 39,
            40, 41, 42, 43,
            44, 45, 46, 47
        ]);
        #[rustfmt::skip]
        push_back!(m, 2, input.iter_range(2, 0..1).cloned().collect::<Vec<usize>>().as_slice());
        assert_eq!(m.offset()[2], 0);
        #[rustfmt::skip]
        assert_eq!(m.array, input.array);

        // All axis.
        let mut m = CircularArrayVec::from_iter(shape, (0..n).map(|_| "___".to_string()));
        #[rustfmt::skip]
        push_back!(m, 0, (0..m.slice_len(0)).map(|i| format!("A{:02}", i)).collect::<Vec<_>>().as_slice());
        #[rustfmt::skip]
        push_back!(m, 1, (0..m.slice_len(1)).map(|i| format!("B{:02}", i)).collect::<Vec<_>>().as_slice());
        #[rustfmt::skip]
        push_back!(m, 2, (0..m.slice_len(2)).map(|i| format!("C{:02}", i)).collect::<Vec<_>>().as_slice());

        #[rustfmt::skip]
        assert_eq!(m.array, &[
            "___", "___", "___", "A00",
            "___", "___", "___", "A01",
            "B01", "B02", "B03", "B00",

            "C05", "C06", "C07", "C04",
            "C09", "C10", "C11", "C08",
            "C01", "C02", "C03", "C00"
        ]);
    }

    #[cfg(feature = "strides")]
    mod translate_front {
        use super::*;
        use crate::Strides;

        #[test]
        fn translate_partial() {
            let src_strides = Strides::new(&[5, 5, 2]);
            #[rustfmt::skip]
            let src = [
                 0,  1,  2,  3,  4,
                 5,  6,  7,  8,  9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,

                25, 26, 27, 28, 29,
                30, 31, 32, 33, 34,
                35, 36, 37, 38, 39,
                40, 41, 42, 43, 44,
                45, 46, 47, 48, 49,
            ];
            let src_fn = |idx: [Range<usize>; 3]| {
                &src[src_strides.flatten_range(idx)]
            };

            #[rustfmt::skip]
            let mut dst = CircularArray::new([3, 3, 1], vec![
                 0,  1,  2,
                 5,  6,  7,
                10, 11, 12,
            ]);

            // Axis 0.
            dst.translate_front(0, 1, [0, 0, 0], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                 1,  2,  3,  
                 6,  7,  8,  
                11, 12, 13, 
            ]);

            // Axis 1.
            dst.translate_front(1, 2, [1, 0, 0], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                11, 12, 13,
                16, 17, 18,
                21, 22, 23,
            ]);

            // Axis 2.
            dst.translate_front(2, 1, [1, 2, 0], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                36, 37, 38,
                41, 42, 43,
                46, 47, 48,
            ]);
        }

        #[test]
        fn translate_full() {
            let src_strides = Strides::new(&[5, 5]);
            #[rustfmt::skip]
            let src = [
                 0,  1,  2,  3,  4,
                 5,  6,  7,  8,  9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,
            ];
            let src_fn = |idx: [Range<usize>; 2]| {
                println!("Recieved range: {idx:?}");
                &src[src_strides.flatten_range(idx)]
            };

            #[rustfmt::skip]
            let mut dst = CircularArray::new([2, 2], vec![
                 0,  1,
                 5,  6,
            ]);

            // Axis 0.
            dst.translate_front(0, 3, [0, 0], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                3, 4,
                8, 9,
            ]);

            // Axis 1.
            dst.translate_front(1, 3, [3, 0], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                18, 19,
                23, 24,
            ]);
        }
    }

    #[cfg(feature = "strides")]
    mod translate_back {
        use super::*;
        use crate::Strides;

        #[test]
        fn translate_partial() {
            let src_strides = Strides::new(&[5, 5, 2]);
            #[rustfmt::skip]
            let src = [
                 0,  1,  2,  3,  4,
                 5,  6,  7,  8,  9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,

                25, 26, 27, 28, 29,
                30, 31, 32, 33, 34,
                35, 36, 37, 38, 39,
                40, 41, 42, 43, 44,
                45, 46, 47, 48, 49,
            ];
            let src_fn = |idx: [Range<usize>; 3]| {
                &src[src_strides.flatten_range(idx)]
            };

            #[rustfmt::skip]
            let mut dst = CircularArray::new([3, 3, 1], vec![
                37, 38, 39,
                42, 43, 44,
                47, 48, 49,
            ]);

            // Axis 0.
            dst.translate_back(0, 1, [2, 2, 1], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                36, 37, 38,
                41, 42, 43,
                46, 47, 48,
            ]);

            // Axis 1.
            dst.translate_back(1, 2, [1, 2, 1], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                26, 27, 28,
                31, 32, 33,
                36, 37, 38,
            ]);

            // Axis 2.
            dst.translate_back(2, 1, [1, 0, 1], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                 1,  2,  3,
                 6,  7,  8,
                11, 12, 13,
            ]);
        }

        #[test]
        fn translate_full() {
            let src_strides = Strides::new(&[5, 5]);
            #[rustfmt::skip]
            let src = [
                 0,  1,  2,  3,  4,
                 5,  6,  7,  8,  9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,
            ];
            let src_fn = |idx: [Range<usize>; 2]| {
                &src[src_strides.flatten_range(idx)]
            };

            #[rustfmt::skip]
            let mut dst = CircularArray::new([2, 2], vec![
                 18,  19,
                 23,  24,
            ]);

            // Axis 0.
            dst.translate_back(0, 3, [3, 3], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                15, 16,
                20, 21,
            ]);

            // Axis 1.
            dst.translate_back(1, 3, [0, 3], src_fn);
            #[rustfmt::skip]
            assert_eq!(dst.iter().cloned().collect::<Vec<_>>(), &[
                0, 1,
                5, 6,
            ]);
        } 
    }    
}
