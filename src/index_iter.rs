use std::array;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// The lower and upper (inclusive) limits of an index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    lower: usize,
    upper: usize,
}

impl From<(usize, usize)> for Bounds {
    fn from((lower, upper): (usize, usize)) -> Self {
        assert!(lower <= upper);
        Self { lower, upper }
    }
}

impl Bounds {
    /// Create a pair of inclusive `Bounds`.
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(lower <= upper);
        Self { lower, upper }
    }

    /// Split the bounds at the given `index`, returning a pair of `Bounds`.
    /// Panics if the given index is `0`, or outside of the `lower` and `upper`
    /// bounds.
    pub fn split_at(self, index: usize) -> (Bounds, Bounds) {
        debug_assert!(index > self.lower);
        debug_assert!(index <= self.upper);
        (
            Bounds::new(self.lower, index - 1),
            (Bounds::new(index, self.upper)),
        )
    }

    /// Get the `n`th value within the `Bounds`, or `None` if out of bounds.
    pub fn get(&self, n: usize) -> Option<usize> {
        let value = self.lower + n;

        if value <= self.upper {
            Some(value)
        } else {
            None
        }
    }

    /// Get the lower bounds.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Get the upper bounds, or return `None` if they do not differ from the upper
    /// bounds.
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Return the number of (range inclusive) indices within the bounds.
    pub fn len(&self) -> usize {
        self.upper - self.lower + 1
    }
}

/// A manually iterated index set confined within limits of the bounds within.
#[derive(Debug)]
struct BoundedIndex {
    /// The current iteration index.
    i: usize,
    /// Exhaust the entire range on iteration.
    exhaust: bool,

    /// The upper index range.
    bounds_a: Bounds,
    /// The lower index range, if any.
    bounds_b: Option<Bounds>,
}

struct IndexSet(BoundedIndex);

impl IndexSet {
    /// Get a reference to the inner `BoundedIndex`.
    fn inner(&self) -> &BoundedIndex {
        &self.0
    }

    /// Get a mutable reference to the inner `BoundedIndex`.
    fn inner_mut(&mut self) -> &mut BoundedIndex {
        &mut self.0
    }
}

impl Iterator for IndexSet {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get();
        self.incr();

        item
    }
}

impl ExactSizeIterator for IndexSet {
    fn len(&self) -> usize {
        self.inner().bounds_a.len()
            + self
                .inner()
                .bounds_b
                .map(|bounds| bounds.len())
                .unwrap_or(0)
    }
}

// TODO: Rename
trait IndexIteratorType: ExactSizeIterator {
    /// Get the current iteration index.
    fn i(&self) -> usize;

    /// Get a mutable reference to the current iteration index.
    fn i_mut(&mut self) -> &mut usize;

    /// Return the current value without advancing the iterator.
    fn get(&self) -> Option<Self::Item>;

    /// Increment the iteration index.
    fn incr(&mut self) {
        *self.i_mut() += 1;
    }

    /// Returns `true` if iteration has finished.
    fn is_finished(&self) -> bool {
        self.i() >= self.len()
    }

    /// Reset the iterator.
    fn reset(&mut self) {
        *self.i_mut() = 0;
    }
}

impl IndexIteratorType for IndexSet {
    fn i(&self) -> usize {
        self.0.i
    }

    fn i_mut(&mut self) -> &mut usize {
        &mut self.0.i
    }

    fn get(&self) -> Option<Self::Item> {
        if self.i() < self.inner().bounds_a.len() {
            self.inner().bounds_a.get(self.i())
        } else {
            self.inner()
                .bounds_b
                .as_ref()
                .map(|bounds| bounds.get(self.i() - self.inner().bounds_a.len()))
                .flatten()
        }
    }
}

impl BoundedIndex {
    /// Create a new `BoundedIndex` from a single set of `Bounds`.
    fn new(bounds: Bounds) -> Self {
        Self {
            i: 0,
            exhaust: false,
            bounds_a: bounds,
            bounds_b: None,
        }
    }

    /// Create a new `BoundedIndex`, split at the offset. Iteration begins at
    /// `offset`.
    fn new_offset(bounds: Bounds, offset: usize) -> Self {
        debug_assert!(bounds.lower < offset && offset <= bounds.upper);

        let (b, a) = bounds.split_at(offset);

        Self {
            i: 0,
            exhaust: false,
            bounds_a: a,
            bounds_b: Some(b),
        }
    }

    /// Create a new `BoundedIndex` from two sets of bounds. Iteration begins at
    /// the "`a.lower`".
    fn new_split(a: Bounds, b: Bounds) -> Self {
        debug_assert!(a.lower <= a.upper && b.lower <= b.upper);

        Self {
            i: 0,
            exhaust: false,
            bounds_a: a,
            bounds_b: Some(b),
        }
    }

    /// Return the number of (range inclusive) indices across all bounds.
    fn len(&self) -> usize {
        self.bounds_a.len() + self.bounds_b.map(|bounds| bounds.len()).unwrap_or(0)
    }

    fn set_exhaustive(&mut self) {
        self.exhaust = true;
    }

    /// Increment the index.
    fn incr(&mut self) {
        self.i += 1;
    }

    /// Get the current index.
    fn current(&self) -> Option<Bounds> {
        match self.exhaust {
            true => self.get_bound(),
            false => self.get_index(),
        }
    }

    fn get_index(&self) -> Option<Bounds> {
        let bounds = if self.i < self.bounds_a.len() {
            self.bounds_a.get(self.i)
        } else {
            self.bounds_b
                .as_ref()
                .map(|bounds| bounds.get(self.i - self.bounds_a.len()))
                .flatten()
        };

        bounds.map(|idx| Bounds::new(idx, idx))
    }

    fn get_index_2(&self) -> Option<usize> {
        let bounds = if self.i < self.bounds_a.len() {
            self.bounds_a.get(self.i)
        } else {
            self.bounds_b
                .as_ref()
                .map(|bounds| bounds.get(self.i - self.bounds_a.len()))
                .flatten()
        };

        bounds
    }

    fn get_bound(&self) -> Option<Bounds> {
        debug_assert!(self.i == 0 || self.i == self.bounds_a.len());

        match self.i {
            0 => Some(self.bounds_a),
            i if i == self.bounds_a.len() => self.bounds_b,
            _ => panic!("Index not aligned to bound"),
        }
    }

    fn next_index(&mut self) -> Option<Bounds> {
        let bounds = self.get_index();
        self.incr();

        bounds
    }

    fn next_index_2(&mut self) -> Option<usize> {
        let bounds = self.get_index_2();
        self.incr();

        bounds
    }

    fn next_bound(&mut self) -> Option<Bounds> {
        // debug_assert!(self.i == 0 || self.i == self.bounds_a.len());

        match self.i {
            0 => {
                self.i = self.bounds_a.len();
                Some(self.bounds_a)
            }
            i if i == self.bounds_a.len() => {
                self.i = self.len();
                // self.i = self.i + self.bounds_b.map(|bounds| bounds.len()).unwrap_or(0);
                self.bounds_b
            }
            i if i == self.len() => None,
            _ => panic!("Index not aligned to bound"),
        }
    }

    // TODO: Remove
    fn is_exhaustive(&self) -> bool {
        // TODO: This is bad. There is no guarantee that the upper bounds equal the shape.
        self.bounds_b.is_none() && self.bounds_a.lower == 0
    }

    fn is_finished(&self) -> bool {
        self.i >= self.len()
    }

    /// Reset the iterator.
    fn reset(&mut self) {
        self.i = 0;
    }
}

// TODO: Remove
impl Iterator for BoundedIndex {
    type Item = Bounds;

    fn next(&mut self) -> Option<Self::Item> {
        match self.exhaust {
            true => self.next_bound(),
            false => self.next_index(),
        }
    }
}

/// An index iterator. Derives indices from the Cartesian product of the sets of
/// `BoundedIndex` within.
#[derive(Debug)]
pub struct IndexIterator<const D: usize>([BoundedIndex; D]);

impl<const D: usize> IndexIterator<D> {
    /// Create a new index for iteration over the entirety of the given bounds.
    fn new(indices: [BoundedIndex; D]) -> Self {
        IndexIterator(indices)
    }

    fn inner(&self) -> &[BoundedIndex; D] {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut [BoundedIndex; D] {
        &mut self.0
    }
}

impl<const D: usize> Iterator for IndexIterator<D> {
    type Item = ([usize; D], [usize; D]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner().iter().all(|bounds| bounds.is_finished()) {
            None
        } else {
            let mut exhaust = true;
            let mut finished = true;

            let mut result = ([0; D], [0; D]);

            self.inner_mut()
                .iter_mut()
                .enumerate()
                .for_each(|(i, idx)| {
                    println!("AXIS {i}");
                    // TODO: This should be done in advance.
                    if exhaust {
                        idx.set_exhaustive();
                    }
                    let bounds = if finished {
                        match idx.next() {
                            Some(bounds) => {
                                println!("    Next returned bounds");
                                bounds
                            }
                            None => {
                                println!("    Next returned None. Resetting");
                                idx.reset();
                                idx.next().expect("No bounds returned from iterator")
                            }
                        }
                    } else {
                        println!("    Getting current index -----------------------------");
                        match idx.current() {
                            Some(bounds) => bounds,
                            None => {
                                idx.reset();
                                println!("        Current bounds reset");
                                idx.current().expect("No current bounds")
                            }
                        }
                    };

                    result.0[i] = bounds.lower;
                    result.1[i] = bounds.upper;

                    exhaust = exhaust & idx.is_exhaustive();
                    finished = finished && idx.is_finished();
                });

            Some(result)
        }
    }
}

// /// An index iterator. Derives indices from the Cartesian product of the sets of
// /// `BoundedIndex` within.
// #[derive(Debug)]
// pub struct IndexIterator_2<const D: usize, I>([I; D]);

// impl<const D: usize> IndexIterator_2<D> {
//     /// Create a new index for iteration over the entirety of the given bounds.
//     fn new(indices: [BoundedIndex; D]) -> Self {
//         IndexIterator(indices)
//     }

//     fn inner(&self) -> &[BoundedIndex; D] {
//         &self.0
//     }

//     fn inner_mut(&mut self) -> &mut [BoundedIndex; D] {
//         &mut self.0
//     }
// }

// struct IndexIter(BoundedIndex);

// impl Iterator for IndexIter {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next_index_2()
//     }
// }

// impl<const D: usize, I> IndexSet<D, I> {
//     fn new<El: Into<I>>(indices: [El; D]) -> Self {
//         let indices = indices.map(|el| el.into());
//         IndexSet(indices)
//     }

//     fn inner(&self) -> &[I; D] {
//         &self.0
//     }

//     fn inner_mut(&mut self) -> &mut [I; D] {
//         &mut self.0
//     }
// }

// /// An iterator that uses the cartesian product of indices to return ranges.
// struct CartesianIndexIterator<const D: usize>([RangeBounds; D]);

// impl<const D: usize> Iterator for CartesianIndexIterator<D> {
//     type Item = [usize; D];

//     fn next(&mut self) -> Option<Self::Item> {}
// }

// #[cfg(test)]
// mod test {

//     use super::*;

//     #[test]
//     fn test_iter_indices() {
//         let shape = [4, 3, 2];
//         let strides = Strides::new(&shape);

//         let range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         let mut idx = CartesianIdx::new_offset(range, [1, 1, 1]);
//         #[rustfmt::skip]
//         assert_eq!(idx.iter_indices(&strides).collect::<Vec<_>>(), [
//             17, 18, 19, 16,
//             21, 22, 23, 20,
//             13, 14, 15, 12,

//              5,  6,  7,  4,
//              9, 10, 11,  8,
//              1,  2,  3,  0
//         ]);

//         let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         range[0] = Bounds::new(1, 1);
//         let mut idx = CartesianIdx::new_offset(range, [1, 1, 1]);
//         #[rustfmt::skip]
//         assert_eq!(idx.iter_indices(&strides).collect::<Vec<_>>(), [
//             17, 21, 13,
//              5,  9,  1
//         ]);

//         let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         range[1] = Bounds::new(1, 1);
//         let mut idx = CartesianIdx::new_offset(range, [1, 1, 1]);
//         #[rustfmt::skip]
//         assert_eq!(idx.iter_indices(&strides).collect::<Vec<_>>(), [
//             17, 18, 19, 16,
//              5,  6,  7,  4,
//         ]);

//         let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         range[2] = Bounds::new(1, 1);
//         let mut idx = CartesianIdx::new_offset(range, [1, 1, 1]);
//         #[rustfmt::skip]
//         assert_eq!(idx.iter_indices(&strides).collect::<Vec<_>>(), [
//             17, 18, 19, 16,
//             21, 22, 23, 20,
//             13, 14, 15, 12,
//         ]);
//     }

//     #[test]
//     fn test_iter_ranges() {
//         let shape = [4, 3, 2];
//         let strides = Strides::new(&shape);

//         let range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         let mut idx = CartesianIdx::new_offset(range, [1, 1, 1]);
//         #[rustfmt::skip]
//         assert_eq!(idx.iter_ranges(&strides).collect::<Vec<_>>(), [
//             17..20, 16..17,
//             21..24, 20..21,
//             13..16, 12..13,

//              5.. 8,  4..5,
//              9..12,  8..9,
//              1.. 4,  0..1
//         ]);

//         // let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         // range[0] = Bounds::new(1, 1);
//         // let mut idx = ArrayIdx::new_offset(range, [1, 1, 1]);
//         // #[rustfmt::skip]
//         // assert_eq!(idx.iter_ranges(&strides).collect::<Vec<_>>(), [
//         //     17, 21, 13,
//         //      5,  9,  1
//         // ]);

//         // let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         // range[1] = Bounds::new(1, 1);
//         // let mut idx = ArrayIdx::new_offset(range, [1, 1, 1]);
//         // #[rustfmt::skip]
//         // assert_eq!(idx.iter_ranges(&strides).collect::<Vec<_>>(), [
//         //     17, 18, 19, 16,
//         //      5,  6,  7,  4,
//         // ]);

//         // let mut range = array::from_fn(|i| Bounds::new(0, shape[i] - 1));
//         // range[2] = Bounds::new(1, 1);
//         // let mut idx = ArrayIdx::new_offset(range, [1, 1, 1]);
//         // #[rustfmt::skip]
//         // assert_eq!(idx.iter_ranges(&strides).collect::<Vec<_>>(), [
//         //     17, 18, 19, 16,
//         //     21, 22, 23, 20,
//         //     13, 14, 15, 12,
//         // ]);
//     }
// }

// pub struct OffsetIter2<const D: usize> {
//     start: [usize; D],
//     end: [usize; D],

//     shape: [usize; D],
//     strides: [usize; D],
// }

// impl<const D: usize> Iterator for OffsetIter2<D> {
//     type Item = SplitRange;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.start != self.end {
//             let start = self
//                 .start
//                 .iter()
//                 .zip(self.strides.iter())
//                 .map(|(a, b)| a * b)
//                 .sum::<usize>();

//             self.start
//                 .iter_mut()
//                 .zip((self.end.iter().zip(self.shape.iter())))
//                 .for_each(|(start, (end, shape))| {
//                     if *start < *end {
//                         *start = (*start + 1) % shape;
//                     }
//                 });
//             let end = self
//                 .start
//                 .iter()
//                 .zip(self.strides.iter())
//                 .map(|(a, b)| a * b)
//                 .sum::<usize>();

//             println!("Range is: {start}..{end}");
//             Some(SplitRange { i: start, start, end, upper_bound: self.shape[axis] })
//             None
//         } else {
//             None
//         }
//     }
// }

// pub struct SplitRange {
//     i: usize,
//     start: usize,
//     end: usize,
//     upper_bound: usize,
// }

// impl Iterator for SplitRange {
//     type Item = Range<usize>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i != self.end {
//             if self.end < self.i {
//                 let start = self.i;
//                 let end = self.upper_bound;
//                 self.i = 0;

//                 Some(start..end)
//             } else {
//                 let start = self.i;
//                 let end = self.end;
//                 self.i = self.end;

//                 Some(start..end)
//             }
//         } else {
//             None
//         }
//     }
// }

// #[test]
// fn offset() {
//     let strides = [1, 3, 9];
//     let lanes = [9, 3, 1];

//     let shape = [3, 3, 3];

//     // 0, 1, 2
//     // 3, 4, 5,
//     // 6, 7, 8

//     // 4, 5, 3
//     // 7, 8, 6
//     // 1, 2, 0

//     // This is an iteration of the entire array. Or, a slice of axis 2.

//     fn into_index(offset: [usize; 3], strides: [usize; 3]) -> usize {
//         offset
//             .iter()
//             .zip(strides.iter())
//             .map(|(a, b)| a * b)
//             .sum::<usize>()
//     }

//     let offset = [1, 1, 0];

//     // OffsetIter2 {
//     //     start: [1, 1, 0],
//     //     end: [1, 2, 0],
//     //     shape: [3, 3, 3],
//     //     strides: [1, 3, 9],
//     // };

//     let iter = OffsetIter2 {
//         start: [0, 1, 0],
//         end: [0, 2, 0],
//         shape: [3, 3, 3],
//         strides: [1, 3, 7],
//     };

//     println!("contents: {:?}", iter.collect::<Vec<_>>());

//     // let start = into_index(offset, strides);
//     // let end = into_index([1, (1 + shape[1]) % shape[1], 0], strides);
//     // println!("Start: {start}, end: {end}");
//     // let iter = OffsetIter::<3>::new(0, shape[1], offset[1], shape[1]);
//     // println!("Items: {:?}", iter.collect::<Vec<_>>());

//     panic!();
// }

// #[cfg(test)]
// mod test_iter_indices {

//     use super::*;

//     // TODO: Dont get index iter. just call iter_index() in the macro below.

//     /// Test the iteration of indices for the given `axis` and `index`.
//     macro_rules! test_iter_indices {
//         (
//             $m:ident,
//             $axis:literal,
//             $range:expr,
//             $assert:expr
//         ) => {
//             assert_eq!(
//                 $m.index_iter($axis, $range)
//                     .iter_indices()
//                     .collect::<Vec<_>>(),
//                 $assert
//             );
//         };
//     }

//     #[test]
//     fn test_axis_0() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_indices!(m, 0, 0, [
//              0,  3,  6,
//              9, 12, 15,
//             18, 21, 24
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 0, 1, [
//              1,  4,  7,
//             10, 13, 16,
//             19, 22, 25
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 0, 2, [
//              2,  5,  8,
//             11, 14, 17,
//             20, 23, 26
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 0, 0..2, [
//              0,  1,     3,  4,     6,  7,
//              9, 10,    12, 13,    15, 16,
//             18, 19,    21, 22,    24, 25
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 0, 1..3, [
//              1,  2,     4,  5,     7,  8,
//             10, 11,    13, 14,    16, 17,
//             19, 20,    22, 23,    25, 26
//         ]);
//     }

//     #[test]
//     fn test_axis_1() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_indices!(m, 1, 0, [
//              0,  1,  2,
//              9, 10, 11,
//             18, 19, 20
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 1, 1, [
//              3,  4,  5,
//             12, 13, 14,
//             21, 22, 23
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 1, 2, [
//              6,  7,  8,
//             15, 16, 17,
//             24, 25, 26
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 1, 0..2, [
//              0,  1,  2,     3,  4,  5,
//              9, 10, 11,    12, 13, 14,
//             18, 19, 20,    21, 22, 23
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 1, 1..3, [
//              3,  4,  5,     6,  7,  8,
//             12, 13, 14,    15, 16, 17,
//             21, 22, 23,    24, 25, 26
//         ]);
//     }

//     #[test]
//     fn test_axis_2() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_indices!(m, 2, 0, [
//              0, 1, 2,
//              3, 4, 5,
//              6, 7, 8
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 2, 1, [
//              9, 10, 11,
//             12, 13, 14,
//             15, 16, 17
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 2, 2, [
//             18, 19, 20,
//             21, 22, 23,
//             24, 25, 26
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 2, 0..2, [
//              0, 1, 2,
//              3, 4, 5,
//              6, 7, 8,

//              9, 10, 11,
//             12, 13, 14,
//             15, 16, 17
//         ]);
//         #[rustfmt::skip]
//         test_iter_indices!(m, 2, 1..3, [
//              9, 10, 11,
//             12, 13, 14,
//             15, 16, 17,

//             18, 19, 20,
//             21, 22, 23,
//             24, 25, 26
//         ]);
//     }
// }

// TODO: Rename to Index
// enum CartesianIdx {
//     Single(usize),
//     Bounded(usize, usize),
// }

// pub struct CartesianBounds {
//     i: usize,
//     bounds: Bounds,
//     offset: usize,
// }

// impl CartesianBounds {
//     pub fn new(bounds: Bounds) -> Self {
//         let i = bounds.lower;
//         let offset = bounds.lower;

//         Self { i, bounds, offset }
//     }
//     pub fn new_with_offset(bounds: Bounds, offset: usize) -> Self {
//         let i = offset;

//         Self { i, bounds, offset }
//     }

//     pub fn exhausted(&self) -> bool {
//         println!("{} == {}", self.i, self.offset);
//         self.i == self.offset
//     }
// }

// impl Iterator for CartesianBounds {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.i;

//         self.i = match self.i {
//             i if i == self.offset => self.bounds.upper,
//             i if i == self.bounds.upper => self.bounds.lower,
//             i if i == self.bounds.lower && self.offset > 1 => self.offset - 1,
//             i if i == self.bounds.lower || i == self.offset - 1 => self.offset,
//             _ => unreachable!(),
//         };

//         Some(i)
//     }
// }

// pub enum CartesianRange {
//     /// A fixed range or index.
//     Fixed(Bounds),
//     /// A split range or indices.
//     Split(Bounds, Bounds),
// }

// struct CartesianBounds {
//     range: CartesianRange,
//     i: usize,
// }

// impl Iterator for CartesianBounds {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.range {
//             CartesianRange::Fixed(bound) => {
//                 let i = match self.i {
//                     0 => Some(bound.lower()),
//                     _ => bound.upper(),
//                 };
//                 self.i = (self.i + 1) % 2;

//                 i
//             }
//             CartesianRange::Split(bound0, bound1) => {
//                 let i = match self.i {
//                     0 => Some(bound0.lower()),
//                     1 => bound0.upper().or(Some(bound1.lower())),
//                     2 if bound0.upper().is_some() => Some(bound1.lower()),
//                     _ => bound1.upper(),
//                 };

//                 i
//             }
//         }
//     }
// }

// struct CartesianBounds {
//     array: [usize; 4],
//     len: usize,
//     i: usize,
// }

// impl CartesianBounds {
//     pub fn new(bounds: Bounds) -> Self {
//         Self {
//             array: [bounds.lower, bounds.upper, 0, 0],
//             len: 2,
//             i: 0,
//         }
//     }

//     pub fn new_offset(bounds: Bounds, offset: usize) -> Self {
//         debug_assert!(offset > bounds.lower && offset < bounds.upper);

//         match offset - bounds.lower {
//             diff if diff == 1 => Self {
//                 array: [bounds.lower, offset, bounds.upper, 0],
//                 len: 3,
//                 i: offset,
//             },

//             _ => Self {
//                 array: [bounds.lower, offset - 1, offset, bounds.upper],
//                 len: 4,
//                 i: offset,
//             },
//         }
//     }

//     // TODO: Is this the proper way to do this?
//     /// Get the current index.
//     fn value(&self) -> Option<&usize> {
//         self.array.get(self.i)
//     }

//     fn last_index(&self) -> usize {
//         self.array[self.len - 1]
//     }

//     /// Reset the iterator.
//     fn reset(&mut self) {
//         self.i = 0;
//     }

//     /// Increment the iterator.
//     fn incr(&mut self) {
//         self.i += 1;
//         // self.i = (self.i + 1) % self.len;
//     }
// }

// impl Iterator for CartesianBounds {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i < self.len {
//             let item = self.array[self.i];
//             self.incr();

//             Some(item)
//         } else {
//             None
//         }
//     }
// }

// struct CartIndex<const D: usize>([CartesianBounds; D]);

// impl<const D: usize> Iterator for CartIndex<D> {
//     type Item = [usize; D];

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut finished = true;

//         let index = array::from_fn(|i| {
//             match finished {
//                 true => match self.0[i].next() {
//                     Some(idx) => {
//                         finished = false;
//                         idx
//                     }
//                     None => {
//                         self.0[i].reset();
//                         self.0[i].last_index()
//                     }
//                 },
//                 // TODO: bad unwrap
//                 false => *self.0[i].value().unwrap(),
//             }
//             // let value = self.0[i].value();
//             // if exhausted {
//             //     self.0[i].next();
//             // }
//             // exhausted = exhausted & self.0[i].exhausted();
//             // println!("Index is {}", value);
//             // value
//         });

//         // println!("Exhausted? {exhausted}");

//         match !finished {
//             true => Some(index),
//             false => None,
//         }
//     }
// }

// #[test]
// fn test_cart2() {
//     let index = CartIndex([
//         CartesianBounds::new(Bounds::new(0, 4)),
//         CartesianBounds::new(Bounds::new(0, 3)),
//         CartesianBounds::new(Bounds::new(0, 2)),
//     ]);

//     println!("{:?}", index.collect::<Vec<[usize; 3]>>());
//     panic!();
// }

// #[cfg(test)]
// mod test_iter_ranges {

//     use super::*;

//     /// Test the iteration of ranges for the given `axis` and `index`.
//     macro_rules! test_iter_ranges {
//         (
//             $m:ident,
//             $axis:literal,
//             $range:expr,
//             $assert:expr
//         ) => {
//             assert_eq!(
//                 $m.index_iter($axis, $range)
//                     .iter_ranges()
//                     .collect::<Vec<_>>(),
//                 $assert
//             );
//         };
//     }

//     #[test]
//     fn test_axis_0() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_ranges!(m, 0, 0, [
//              0.. 1,  3.. 4,  6..7,
//              9..10, 12..13, 15..16,
//             18..19, 21..22, 24..25
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 0, 1, [
//              1.. 2,  4.. 5,  7.. 8,
//             10..11, 13..14, 16..17,
//             19..20, 22..23, 25..26
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 0, 2, [
//              2.. 3,  5.. 6,  8.. 9,
//             11..12, 14..15, 17..18,
//             20..21, 23..24, 26..27
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 0, 0..2, [
//              0.. 2,     3.. 5,     6.. 8,
//              9..11,    12..14,    15..17,
//             18..20,    21..23,    24..26
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 0, 1..3, [
//              1.. 3,     4.. 6,     7.. 9,
//             10..12,    13..15,    16..18,
//             19..21,    22..24,    25..27
//         ]);
//     }

//     #[test]
//     fn test_axis_1() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_ranges!(m, 1, 0, [
//              0..3,
//              9..12,
//             18..21
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 1, 1, [
//              3..6,
//             12..15,
//             21..24
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 1, 2, [
//              6..9,
//             15..18,
//             24..27
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 1, 0..2, [
//              0.. 6,
//              9..15,
//             18..24
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 1, 1..3, [
//              3.. 9,
//             12..18,
//             21..27
//         ]);
//     }

//     #[test]
//     fn test_axis_2() {
//         let iter = (0..3 * 3 * 3).into_iter();
//         let m = CircularArray::from_iter([3, 3, 3], iter);

//         #[rustfmt::skip]
//         test_iter_ranges!(m, 2, 0, [
//              0..9
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 2, 1, [
//              9..18
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 2, 2, [
//             18..27
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 2, 0..2, [
//             0..18
//         ]);
//         #[rustfmt::skip]
//         test_iter_ranges!(m, 2, 1..3, [
//             9..27
//         ]);
//     }
// }
