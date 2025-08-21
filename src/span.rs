use std::ops::Range;
use std::ops::{Add, Rem, Sub};

/// A span of inclusive elements within an axis. In contrast to [`BoundSpan`], all
/// elements are guaranteed to be within axis bounds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnboundSpan {
    /// The first element of the span.
    pub(crate) start: usize,
    /// The last element of the span.
    pub(crate) end: usize,
}

impl UnboundSpan {
    /// Create a new `Span`, guaranteed to be within a contextual axis.
    pub(crate) fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    pub(crate) fn from_len(start: usize, len: usize) -> Self {
        debug_assert!(len > 0);
        let end = start + len - 1;

        UnboundSpan::new(start, end)
    }

    /// Get the number of elements within the span.
    pub(crate) fn len(&self) -> usize {
        self.end - self.start + 1
    }

    /// Get the index of the element `i` from `start`. Returns `None` if the index
    /// exceeds the `end` of the span.
    pub(crate) fn get_index(&self, i: usize) -> Option<usize> {
        Some(self.start + i).filter(|i| *i <= self.end)
    }

    /// Consume the `UnboundSpan`, returning a `Range<usize>`. Offsets ranges
    /// by the given value.
    pub(crate) fn into_range(self, offset: usize) -> Range<usize> {
        (self.start + offset)..(self.end + offset + 1)
    }
}

impl From<usize> for UnboundSpan {
    fn from(value: usize) -> Self {
        UnboundSpan {
            start: value,
            end: value,
        }
    }
}

impl From<(usize, usize)> for UnboundSpan {
    fn from((start, end): (usize, usize)) -> Self {
        UnboundSpan { start, end }
    }
}

/// A span of inclusive elements within an axis. In contrast to [`UnboundSpan`],
/// elements may wrap across axis bounds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoundSpan {
    /// The start index of the span.
    start: usize,
    /// The length of the span.
    len: usize,
    /// The upper (exclusive) bound of the span.
    bound: usize,
}

impl BoundSpan {
    /// Create a pair of inclusive `Bounds`. All `Span`s are assumed to have a
    /// `len` less than, or equal to the upper bound of an axis.
    pub(crate) fn new(start: usize, len: usize, bound: usize) -> Self {
        debug_assert!(bound > start);
        debug_assert!(len <= bound);

        assert!(len > 0);
        Self { start, bound, len }
    }

    /// Get the length of elements within the span.
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// Get the upper bound of the span.
    pub(crate) fn bound(&self) -> usize {
        self.bound
    }

    /// Returns `true` if the span is exhaustive of the axis.
    pub(crate) fn exhaustive(&self) -> bool {
        self.start == 0 && self.len == self.bound
    }

    /// Returns `true` if the span wraps across the `bound`.
    pub(crate) fn is_wrapping(&self) -> bool {
        self.start + self.len > self.bound
    }

    /// Get the span of elements on either side of the axis bounds, or return `None`
    /// if out of bounds.
    pub(crate) fn get_span(&self, i: usize) -> Option<UnboundSpan> {
        match i {
            0 => Some(UnboundSpan::new(
                self.start,
                (self.start + self.len - 1).min(self.bound - 1),
            )),
            1 if self.is_wrapping() => Some(UnboundSpan::new(
                0,
                (self.start + self.len - 1) % self.bound,
            )),
            _ => None,
        }
    }

    /// Get the index of the element `i` from `start`, wrapping over the `bound`,
    /// if any. Returns `None` if the index exceeds the `len` of the span.
    pub(crate) fn get_index(&self, i: usize) -> Option<usize> {
        if i >= self.len {
            None
        } else {
            Some((self.start + i) % self.bound)
        }
    }

    /// Get the index of the element `i` of the wrapping elements (if any), followed
    /// by the indices following `start`. Returns `None` if the index exceeds the
    /// `len` of the span.
    pub(crate) fn get_index_ordered(&self, i: usize) -> Option<usize> {
        if i >= self.len {
            None
        } else if let Some(span_len) = self.get_span(1).map(|span| span.len()) {
            if i < span_len {
                Some(i)
            } else {
                Some(self.start + i - span_len)
            }
        } else {
            self.get_index(i)
        }
    }
}

impl Add<usize> for BoundSpan {
    type Output = BoundSpan;

    fn add(self, rhs: usize) -> Self::Output {
        BoundSpan {
            start: self.start + rhs,
            len: self.len,
            bound: self.bound,
        }
    }
}

impl Sub<usize> for BoundSpan {
    type Output = BoundSpan;

    fn sub(self, rhs: usize) -> Self::Output {
        BoundSpan {
            start: self.start - rhs,
            len: self.len,
            bound: self.bound,
        }
    }
}

impl Rem<usize> for BoundSpan {
    type Output = BoundSpan;

    fn rem(self, rhs: usize) -> Self::Output {
        BoundSpan {
            start: self.start % rhs,
            len: self.len,
            bound: self.bound,
        }
    }
}
