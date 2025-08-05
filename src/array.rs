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
        Self::new_offset(shape, array, [0; N])
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
    /// let circular_array = CircularArray::new_offset(shape, array, [1, 0, 0]);
    /// ```
    pub fn new_offset(shape: [usize; N], array: A, offset: [usize; N]) -> CircularArray<N, A, T> {
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

    /// Get the shape array.
    pub fn shape(&self) -> &[usize; N] {
        &self.shape
    }

    /// Get the offset array.
    pub fn offset(&self) -> &[usize; N] {
        &self.offset
    }

    /// Get the number of elements in the array.
    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }

    /// Get the number of elements for a single slice of the buffer, for the given
    /// `axis`. Pushing `n` slices of elements onto an axis requires `n * slice_len`
    /// elements to be passed to the respective method.
    pub fn slice_len(&self, axis: usize) -> usize {
        self.shape
            .iter()
            .enumerate()
            .fold(1, |acc, (i, sh)| if i == axis { acc } else { acc * sh })
    }

    /// Get the offset value after adding `n` slices to the given `axis`.
    pub fn next_offset(&self, axis: usize, n: isize) -> usize {
        (((self.shape[axis] + self.offset[axis]) as isize + n) % self.shape[axis] as isize) as usize
    }

    /// Drop the `CircularArray`, returning the inner buffer.
    pub fn take(self) -> A {
        self.array
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
        Self::new_offset(shape, array, [0; N])
    }

    /// Create a new [`CircularArrayVec`] from an iterator with the given `offset`.
    ///
    /// # Examples
    /// ```
    /// # use n_circular_array::CircularArrayVec;
    /// let shape = [3, 3, 3];
    /// // Offset by 1 on axis 0.
    /// let circular_array = CircularArrayVec::from_iter_offset(shape, 0..shape.iter().product(), [1, 0, 0]);
    /// ```
    pub fn from_iter_offset(
        shape: [usize; N],
        iter: impl Iterator<Item = T>,
        offset: [usize; N],
    ) -> Self {
        let array = iter.collect::<Vec<T>>();
        Self::new_offset(shape, array, offset)
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
        Self::new_offset(shape, array, [0; N])
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
        Self::new_offset(shape, array, offset)
    }
}
