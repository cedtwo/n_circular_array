use std::ops::{Deref, DerefMut, Range};

/// The strides of an `N` dimension array.
#[derive(Debug, Clone, Copy)]
pub struct Strides<const N: usize>([usize; N]);

impl<const N: usize> Strides<N> {
    /// Create `Strides` for the given `shape`.
    pub fn new(shape: &[usize; N]) -> Self {
        let mut array = [1; N];
        for i in 1..N {
            array[i] = array[i - 1] * shape[i - 1];
        }

        Strides(array)
    }

    /// Multiply an `N` dimensional index by the strides.
    pub(crate) fn offset_index(&self, index: [usize; N]) -> usize {
        index
            .iter()
            .zip(self.iter())
            .map(|(idx, stride)| idx * stride)
            .sum::<usize>()
    }

    /// Flatten an `N` dimensional **contiguous** index range into a contiguous
    /// `Range<usize>`.
    ///
    /// This method is used for mapping between a *source* array to the *destination*
    /// `CircularArray`. As such, it expects a range **only** contiguous on axis `0`.
    pub fn flatten_range(&self, mut index_range: [Range<usize>; N]) -> Range<usize> {
        debug_assert!(
            index_range.iter().skip(1).all(|range| range.len() == 1),
            "Unexpected index_range shape"
        );

        let cont_range = std::mem::take(&mut index_range[0]);
        let offset = self.offset_index(index_range.map(|range| range.start as usize));

        cont_range.start as usize + offset..cont_range.end as usize + offset
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
