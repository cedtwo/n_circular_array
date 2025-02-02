use std::ops::{Deref, DerefMut};

/// The strides for an `N` dimension array.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Strides<const N: usize>([usize; N]);

impl<const N: usize> Strides<N> {
    /// Create `Strides` for the given `shape`.
    pub(crate) fn new(shape: &[usize; N]) -> Self {
        let mut array = [1; N];
        for i in 1..N {
            array[i] = array[i - 1] * shape[i - 1];
        }

        Strides(array)
    }

    /// Apply the strides to an `N` dimension index.
    pub(crate) fn apply_to_index(&self, index: [usize; N]) -> usize {
        index
            .iter()
            .zip(self.iter())
            .map(|(idx, stride)| idx * stride)
            .sum::<usize>()
    }
}

impl<const N: usize> Deref for Strides<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Strides<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
