use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

use crate::span::UnboundSpan;
use crate::strides::Strides;

/// A raw index of `N` dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RawIndex<const N: usize>([usize; N]);

impl<const N: usize> Default for RawIndex<N> {
    fn default() -> Self {
        RawIndex([0; N])
    }
}

impl<const N: usize> Index<usize> for RawIndex<N> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for RawIndex<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Deref for RawIndex<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for RawIndex<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A contiguous raw index span across `N` dimensions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct RawIndexSpan<const N: usize>(pub(crate) [UnboundSpan; N]);

impl<const N: usize> Debug for RawIndexSpan<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start, end) = self.split_bounds();
        f.debug_struct("RawIndexSpan")
            .field("start", &start)
            .field("end", &end)
            .finish()
    }
}

impl<const N: usize> RawIndexSpan<N> {
    /// Split the `RawIndexSpan` into a bounding `RawIndex` pair.
    pub(crate) fn split_bounds(self) -> (RawIndex<N>, RawIndex<N>) {
        let mut result = (RawIndex::default(), RawIndex::default());
        self.iter().enumerate().for_each(|(i, span)| {
            result.0[i] = span.start;
            result.1[i] = span.end;
        });

        result
    }

    /// Split the `RawIndexSpan` into a ranges. Offsets ranges by the given `origin`.
    pub(crate) fn into_ranges(self, origin: [usize; N]) -> [Range<usize>; N] {
        std::array::from_fn(|i| self[i].into_range(origin[i]))
    }
}

impl<const N: usize> PartialEq<([usize; N], [usize; N])> for RawIndexSpan<N> {
    fn eq(&self, other: &([usize; N], [usize; N])) -> bool {
        self.iter()
            .zip(other.0.iter().zip(other.1.iter()))
            .all(|(span, tuple)| span.start == *tuple.0 && span.end == *tuple.1)
    }
}

impl<const N: usize> Default for RawIndexSpan<N> {
    fn default() -> Self {
        RawIndexSpan([UnboundSpan::default(); N])
    }
}

impl<const D: usize> From<[UnboundSpan; D]> for RawIndexSpan<D> {
    fn from(span: [UnboundSpan; D]) -> Self {
        RawIndexSpan(span)
    }
}

impl<const N: usize> Index<usize> for RawIndexSpan<N> {
    type Output = UnboundSpan;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for RawIndexSpan<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Deref for RawIndexSpan<N> {
    type Target = [UnboundSpan; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for RawIndexSpan<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Iterator adaptor for `RawIndexSpan` type conversion.
pub(crate) trait RawIndexAdaptor<'a, const N: usize> {
    /// Flatten `RawIndexSpan` types into `usize` elements.
    #[allow(dead_code)]
    fn into_flat_indices(self, strides: &'a Strides<N>)
        -> impl Iterator<Item = usize> + Clone + 'a;

    /// Return `RawIndexSpan`s as `N` dimensional index ranges. Takes an `origin`
    /// for offsetting ranges.
    fn into_ranges(self, origin: [usize; N]) -> impl Iterator<Item = [Range<usize>; N]>;

    /// Flatten `RawIndexSpan` into contiguous `usize` ranges.
    fn into_flat_ranges(
        self,
        strides: &'a Strides<N>,
    ) -> impl Iterator<Item = Range<usize>> + Clone + 'a;
}

impl<'a, const N: usize, T: Iterator<Item = RawIndexSpan<N>> + Clone + 'a> RawIndexAdaptor<'a, N>
    for T
{
    fn into_flat_indices(
        self,
        strides: &'a Strides<N>,
    ) -> impl Iterator<Item = usize> + Clone + 'a {
        self.flat_map(|span| {
            let (start, end) = span.split_bounds();
            strides.offset_index(*start)..strides.offset_index(*end) + 1
        })
    }

    fn into_ranges(self, origin: [usize; N]) -> impl Iterator<Item = [Range<usize>; N]> {
        self.map(move |span| span.into_ranges(origin))
    }

    fn into_flat_ranges(
        self,
        strides: &'a Strides<N>,
    ) -> impl Iterator<Item = Range<usize>> + Clone + 'a {
        self.map(|span| {
            let (start, end) = span.split_bounds();
            strides.offset_index(*start)..strides.offset_index(*end) + 1
        })
    }
}
