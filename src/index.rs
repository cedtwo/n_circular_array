use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::span::Span;

/// A raw index type of `N` dimensions.
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
pub(crate) struct RawIndexSpan<const N: usize>([Span; N]);

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
        RawIndexSpan([Span::default(); N])
    }
}

impl<const D: usize> From<[Span; D]> for RawIndexSpan<D> {
    fn from(span: [Span; D]) -> Self {
        RawIndexSpan(span)
    }
}

impl<const N: usize> Index<usize> for RawIndexSpan<N> {
    type Output = Span;

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
    type Target = [Span; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for RawIndexSpan<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
