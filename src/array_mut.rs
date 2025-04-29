use crate::span::BoundSpan;
use crate::span_iter::{RawIndexAdaptor, SpanIterator};
use crate::CircularArray;

/// Mutating `CircularArray` operations.
pub trait CircularArrayMut<const N: usize, T, El> {
    /// Push elements to the front of the given `axis`, taking into account the
    /// offsets of **all** axes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularArrayIndex, CircularArrayMut};
    /// let offset = [1, 0];
    /// let elements = [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    /// let mut array = CircularArray::new_offset([3, 3], elements, offset);
    ///
    /// array.push_front(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     11,  9, 10,
    ///      3,  4,  5,
    ///      6,  7,  8,
    /// ]);
    /// ```
    fn push_front(&mut self, axis: usize, el: El);

    /// Push elements to the front of the given `axis`, taking into account only
    /// the offset of the given `axis`. Elements must be an exact multiple of
    /// the slice size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularArrayIndex, CircularArrayMut};
    /// let offset = [1, 0];
    /// let elements = [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    /// let mut array = CircularArray::new_offset([3, 3], elements, offset);
    ///
    /// array.push_front_raw(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     9, 10, 11,
    ///     3,  4,  5,
    ///     6,  7,  8,
    /// ]);
    /// ```
    fn push_front_raw(&mut self, axis: usize, el: El);

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** exes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularArrayIndex, CircularArrayMut};
    /// let offset = [1, 0];
    /// let elements = [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    /// let mut array = CircularArray::new_offset([3, 3], elements, offset);
    ///
    /// array.push_back(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///      0,  1,  2,
    ///      3,  4,  5,
    ///     11,  9, 10,
    /// ]);
    /// ```
    fn push_back(&mut self, axis: usize, el: El);

    /// Push elements to the back of the given `axis`, taking into account the
    /// offsets of **all** axes. Elements must be an exact multiple of the slice
    /// size for the given `axis`. See [`CircularArray::slice_len`].
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularArrayIndex, CircularArrayMut};
    /// let offset = [1, 0];
    /// let elements = [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    /// let mut array = CircularArray::new_offset([3, 3], elements, offset);
    ///
    /// array.push_back_raw(1, &[9, 10, 11]);
    /// assert_eq!(array.iter_raw().cloned().collect::<Vec<_>>(), &[
    ///     0,  1,  2,
    ///     3,  4,  5,
    ///     9, 10, 11,
    /// ]);
    /// ```
    fn push_back_raw(&mut self, axis: usize, el: El);
}

impl<const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: Clone> CircularArray<N, A, T> {
    /// Push the given elements into the ranges defined by the given `spans`. Promotes
    /// cache locality for the input elements.
    fn push(&mut self, spans: [BoundSpan; N], mut el: &[T]) {
        let iter = SpanIterator::new(spans).into_ranges(&self.strides);

        for slice_range in iter {
            let len = slice_range.len();
            self.array.as_mut()[slice_range].clone_from_slice(&el[..len]);
            (_, el) = el.split_at(len);
        }
    }

    /// Increment the offset by `n` on the given `axis`.
    fn incr_offset(&mut self, axis: usize, n: usize) {
        self.offset[axis] = (self.offset[axis] + n) % self.shape()[axis];
    }

    /// Decrement the offset by `n` on the given `axis`.
    fn decr_offset(&mut self, axis: usize, n: usize) {
        self.offset[axis] = (self.shape()[axis] + self.offset[axis] - n) % self.shape()[axis];
    }
}

impl<'a, const N: usize, A: AsRef<[T]> + AsMut<[T]>, T: Clone> CircularArrayMut<N, T, &'a [T]>
    for CircularArray<N, A, T>
{
    fn push_front(&mut self, axis: usize, el: &'a [T]) {
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

                self.push(spans, el);
                self.incr_offset(axis, n);
            }
        }
    }

    fn push_front_raw(&mut self, axis: usize, el: &'a [T]) {
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
                let spans = self.spans_axis_bound_raw(axis, BoundSpan::new(0, n, self.shape[axis]));

                self.push(spans, el);
                self.incr_offset(axis, n);
            }
        }
    }

    fn push_back(&mut self, axis: usize, el: &'a [T]) {
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
                let span = BoundSpan::new(
                    (self.shape[axis] - n) % self.shape[axis],
                    n,
                    self.shape[axis],
                );
                let spans = self.spans_axis_bound(axis, span);

                self.push(spans, el);
                self.decr_offset(axis, n);
            }
        }
    }

    fn push_back_raw(&mut self, axis: usize, el: &'a [T]) {
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
                let span = BoundSpan::new(
                    (self.shape[axis] - n) % self.shape[axis],
                    n,
                    self.shape[axis],
                );
                let spans = self.spans_axis_bound_raw(axis, span);

                self.push(spans, el);
                self.decr_offset(axis, n);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::array_index::CircularArrayIndex;
    use crate::span_iter::RawIndexAdaptor;
    use crate::CircularArrayVec;

    macro_rules! push_front {
        (
            $m:ident,
            $axis:literal,
            $payload:expr
        ) => {
            let n = $payload.len() / $m.slice_len($axis);
            $m.push_front($axis, $payload);

            let slice = SpanIterator::new($m.spans_axis_bound(
                $axis,
                BoundSpan::new($m.shape()[$axis] - n, n, $m.shape()[$axis]),
            ))
            .into_indices(&$m.strides)
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

            let slice = SpanIterator::new(
                $m.spans_axis_bound($axis, BoundSpan::new(0, n, $m.shape()[$axis])),
            )
            .into_indices(&$m.strides)
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
}
