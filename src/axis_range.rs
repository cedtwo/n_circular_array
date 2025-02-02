use std::{iter::from_fn, ops::Range};

/// A range of slices on an axis, possibly split over the lower or upper bound.
#[derive(Debug)]
pub enum AxisRange {
    /// A sequentual range of slices.
    Sequentual(Range<usize>),
    /// A range of slices split over a bound in row-major (element wise) order.
    Split(Range<usize>, Range<usize>),
}

impl AxisRange {
    /// Create a new sequentual axis range.
    pub(crate) fn new_sequentual(low: usize, high: usize) -> Self {
        debug_assert!(low < high);
        AxisRange::Sequentual(low..high)
    }

    /// Create a new split axis range.
    pub(crate) fn new_split(low: (usize, usize), high: (usize, usize)) -> Self {
        debug_assert!(low.0 < low.1 && low.1 < high.0 && high.0 < high.1);
        AxisRange::Split(low.0..low.1, high.0..high.1)
    }

    /// Get the end of a decreasing range.
    pub(crate) fn decr_bound(&self) -> usize {
        match self {
            AxisRange::Sequentual(range) | AxisRange::Split(_, range) => range.start,
        }
    }

    /// Get the end of an increasing range.
    pub(crate) fn incr_bound(&self) -> usize {
        match self {
            AxisRange::Sequentual(range) | AxisRange::Split(range, _) => range.end,
        }
    }

    /// Consume the `AxisRange`, returning an iterator over indices of the range(s).
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        let range_iter = |i: &mut usize, range: &Range<usize>| {
            if *i >= range.end {
                None
            } else {
                if *i < range.start {
                    *i = range.start;
                };
                *i += 1;

                Some(*i - 1)
            }
        };

        let mut i = 0;
        from_fn(move || match &self {
            AxisRange::Sequentual(range) => range_iter(&mut i, &range),
            AxisRange::Split(range0, range1) => {
                range_iter(&mut i, &range0).or_else(|| range_iter(&mut i, &range1))
            }
        })
    }
}

#[test]
fn test_iter() {
    let sequentual = AxisRange::new_sequentual(0, 10);
    let split = AxisRange::new_split((0, 10), (30, 40));

    assert_eq!(
        sequentual.iter().collect::<Vec<_>>(),
        (0..10).collect::<Vec<_>>()
    );
    assert_eq!(
        split.iter().collect::<Vec<_>>(),
        (0..10)
            .into_iter()
            .chain((30..40).into_iter())
            .collect::<Vec<_>>()
    );
}
