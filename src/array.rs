use std::marker::PhantomData;

use crate::strides::Strides;

/// A `CircularArray` with elements stored in a `Vec`.
pub type CircularArrayVec<const N: usize, T> = CircularArray<N, Vec<T>, T>;
/// A `CircularArray` with elements stored in a `Box`.
pub type CircularArrayBox<const N: usize, T> = CircularArray<N, Box<[T]>, T>;

/// A circular array of `D` dimensions holding elements of type `T`.
pub struct CircularArray<const N: usize, A, T> {
    pub(crate) array: A,
    /// The length of elements for each axis.
    pub(crate) shape: [usize; N],
    /// Contiguous sections of memory for each axis.
    pub(crate) strides: Strides<N>,
    pub(crate) offset: [usize; N],

    _phantom: PhantomData<T>,
}

impl<const N: usize, A, T> CircularArray<N, A, T>
where
    A: AsRef<[T]>,
{
    pub fn new(shape: [usize; N], array: A) -> CircularArray<N, A, T> {
        Self::new_offset(shape, array, [0; N])
    }

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

    /// Get the number of elements for a single slice of the array, for the given
    /// `axis`. Pushing `n` slices of elements onto an axis requires `n * slice_len`
    /// elements to be passed to the respective method.
    pub fn slice_len(&self, axis: usize) -> usize {
        self.shape
            .iter()
            .enumerate()
            .fold(1, |acc, (i, sh)| if i == axis { acc } else { acc * sh })
    }

    /// Get the offset value after adding `n` to the given `axis`.
    pub fn next_offset(&self, axis: usize, n: isize) -> usize {
        (((self.shape[axis] + self.offset[axis]) as isize + n) % self.shape[axis] as isize) as usize
    }

    /// Consume the `CircularArray`, returning the inner collection.
    pub fn take(self) -> A {
        self.array
    }
}

impl<const N: usize, T> CircularArray<N, Vec<T>, T> {
    pub fn from_iter(shape: [usize; N], iter: impl Iterator<Item = T>) -> Self {
        let array = iter.collect::<Vec<T>>();
        Self::new_offset(shape, array, [0; N])
    }

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
    pub fn from_iter(shape: [usize; N], iter: impl Iterator<Item = T>) -> Self {
        let array = iter.collect::<Vec<T>>().into_boxed_slice();
        Self::new_offset(shape, array, [0; N])
    }

    pub fn from_iter_offset(
        shape: [usize; N],
        iter: impl Iterator<Item = T>,
        offset: [usize; N],
    ) -> Self {
        let array = iter.collect::<Vec<T>>().into_boxed_slice();
        Self::new_offset(shape, array, offset)
    }
}
