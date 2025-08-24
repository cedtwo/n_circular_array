use std::marker::PhantomData;

use crate::strides::Strides;

/// A `CircularArray` backed by a `Vec`.
pub type CircularArrayVec<const N: usize, T> = CircularArray<N, Vec<T>, T>;
/// A `CircularArray` backed by a `Box`.
pub type CircularArrayBox<const N: usize, T> = CircularArray<N, Box<[T]>, T>;

/// A circular array of `N` dimensions for elements of type `T`.
///
/// Supports any fixed size contiguous element buffer implementing `AsRef<[T]>`
/// and `AsMut<T>`.
///
/// General array data is accessible as associated methods while element access
/// and mutation are delegated to [`CircularIndex`](crate::CircularIndex) and [`CircularMut`](crate::CircularMut).
pub struct CircularArray<const N: usize, A, T> {
    /// The circular array buffer.
    pub(crate) array: A,
    /// The length of elements for each axis.
    pub(crate) shape: [usize; N],
    /// Contiguous sections of memory for each axis.
    pub(crate) strides: Strides<N>,
    /// The offset of each axis.
    pub(crate) offset: [usize; N],

    _phantom: PhantomData<T>,
}

impl<const N: usize, A, T> CircularArray<N, A, T>
where
    A: AsRef<[T]>,
{
    /// Create a new `CircularArray` from the given buffer.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArray;
    /// let shape = [3, 3, 3];
    /// let array = Vec::from_iter(0..shape.iter().product());
    ///
    /// let circular_array = CircularArray::new(shape, array);
    /// ```
    pub fn new(shape: [usize; N], array: A) -> CircularArray<N, A, T> {
        Self::new_offset(shape, [0; N], array)
    }

    /// Create a new `CircularArray` from the given buffer and `offset`.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArray;
    /// let shape = [3, 3, 3];
    /// let array = Vec::from_iter(0..shape.iter().product());
    ///
    /// // Offset by 1 on axis 0.
    /// let circular_array = CircularArray::new_offset(shape, [1, 0, 0], array);
    /// ```
    pub fn new_offset(shape: [usize; N], offset: [usize; N], array: A) -> CircularArray<N, A, T> {
        assert!(
            array.as_ref().len() == shape.iter().product(),
            "Element length does not match shape"
        );

        let array = array;
        let strides = Strides::new(&shape);

        CircularArray {
            array,
            strides,
            shape,
            offset,
            _phantom: PhantomData,
        }
    }

    /// Get the array shape.
    pub fn shape(&self) -> &[usize; N] {
        &self.shape
    }

    #[cfg(feature = "strides")]
    /// Get the array [`Strides`](crate::strides::Strides).
    pub fn strides(&self) -> &Strides<N> {
        &self.strides
    }

    /// Get the array offset.
    ///
    /// This is not always incremented sequentually. Where a mutating operation
    /// inserts elements equal to the product of the array shape, the offset will
    /// be set to `[0; N]`.
    pub fn offset(&self) -> &[usize; N] {
        &self.offset
    }

    /// Get a mutable reference to the array offset.
    ///
    /// Manually mutating the offset is **not** recommended unless clearing data. See
    /// also [`CircularArray::data_mut`].
    pub fn offset_mut(&mut self) -> &mut [usize; N] {
        &mut self.offset
    }

    /// Get the number of elements in the array.
    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }

    /// Get the number of elements for a single slice of the buffer, for the given
    /// `axis`. Pushing `n` slices of elements onto an axis requires `n * slice_len`
    /// elements to be passed to the respective method.
    ///
    /// # Example
    /// ```
    /// # use n_circular_array::{CircularArray, CircularIndex};
    /// let mut array = CircularArray::new([4, 3, 2], vec![
    ///      0,  1,  2,  3,
    ///      4,  5,  6,  7,
    ///      8,  9, 10, 11,
    ///
    ///     12, 13, 14, 15,
    ///     16, 17, 18, 19,
    ///     20, 21, 22, 23,
    /// ]);
    ///
    /// // A single slice of axis 0 is 6 elements.
    /// assert_eq!(array.slice_len(0), 6);
    /// assert_eq!(array.iter_index(0, 0).cloned().collect::<Vec<_>>(), [
    ///      0,
    ///      4,
    ///      8,
    ///     12,
    ///     16,
    ///     20
    /// ]);
    ///
    /// // A single slice of axis 1 is 8 elements.
    /// assert_eq!(array.slice_len(1), 8);
    /// assert_eq!(array.iter_index(1, 0).cloned().collect::<Vec<_>>(), [
    ///      0,  1,  2,  3,
    ///     12, 13, 14, 15
    /// ]);
    ///
    /// // A single slice of axis 2 is 12 elements.
    /// assert_eq!(array.slice_len(2), 12);
    /// assert_eq!(array.iter_index(2, 0).cloned().collect::<Vec<_>>(), [
    ///      0,  1,  2,  3,
    ///      4,  5,  6,  7,
    ///      8,  9, 10, 11,
    /// ]);
    /// ```
    pub fn slice_len(&self, axis: usize) -> usize {
        self.shape
            .iter()
            .enumerate()
            .fold(1, |acc, (i, sh)| if i == axis { acc } else { acc * sh })
    }

    /// Drop the `CircularArray`, returning the inner buffer. Note that data is
    /// returned without applying any normalizing operations.
    pub fn take(self) -> A {
        self.array
    }

    /// Get a reference to the inner buffer `A`.
    ///
    /// This may be useful for operations where element order is arbitrary. See
    /// also [`CircularIndex::iter_raw`](crate::CircularIndex::iter_raw), [`CircularIndex::iter_index_raw`](crate::CircularIndex::iter_index_raw) and
    /// [`CircularIndex::iter_slice_raw`](crate::CircularIndex::iter_slice_raw).
    pub fn data(&self) -> &A {
        &self.array
    }

    /// Get a mutable reference to the inner buffer `A`.
    ///
    /// Manually mutating data is **not** recommended unless clearing data. See
    /// also [`CircularArray::offset_mut`].
    pub fn data_mut(&mut self) -> &mut A {
        &mut self.array
    }
}

impl<const N: usize, T> CircularArray<N, Vec<T>, T> {
    /// Create a new [`CircularArrayVec`] from an iterator.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArrayVec;
    /// let shape = [3, 3, 3];
    /// let circular_array = CircularArrayVec::from_iter(shape, 0..shape.iter().product());
    /// ```
    pub fn from_iter(shape: [usize; N], iter: impl Iterator<Item = T>) -> Self {
        let array = iter.collect::<Vec<T>>();
        Self::new_offset(shape, [0; N], array)
    }

    /// Create a new [`CircularArrayVec`] from an iterator with the given `offset`.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArrayVec;
    /// let shape = [3, 3, 3];
    /// // Offset by 1 on axis 0.
    /// let circular_array = CircularArrayVec::from_iter_offset(shape, [1, 0, 0], 0..shape.iter().product());
    /// ```
    pub fn from_iter_offset(
        shape: [usize; N],
        offset: [usize; N],
        iter: impl Iterator<Item = T>,
    ) -> Self {
        let array = iter.collect::<Vec<T>>();
        Self::new_offset(shape, offset, array)
    }
}

impl<const N: usize, T> CircularArray<N, Box<[T]>, T> {
    /// Create a new [`CircularArrayBox`] from an iterator.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArrayBox;
    /// let shape = [3, 3, 3];
    /// let circular_array = CircularArrayBox::from_iter(shape, 0..shape.iter().product());
    /// ```
    pub fn from_iter(shape: [usize; N], iter: impl Iterator<Item = T>) -> Self {
        let array = iter.collect::<Vec<T>>().into_boxed_slice();
        Self::new_offset(shape, [0; N], array)
    }

    /// Create a new [`CircularArrayBox`] from an iterator with the given `offset`.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArrayBox;
    /// let shape = [3, 3, 3];
    /// // Offset by 1 on axis 0.
    /// let circular_array = CircularArrayBox::from_iter_offset(shape, 0..shape.iter().product(), [1, 0, 0]);
    /// ```
    pub fn from_iter_offset(
        shape: [usize; N],
        iter: impl Iterator<Item = T>,
        offset: [usize; N],
    ) -> Self {
        let array = iter.collect::<Vec<T>>().into_boxed_slice();
        Self::new_offset(shape, offset, array)
    }
}
