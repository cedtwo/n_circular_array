# n_circular_array

## N Circular Array
An n-dimensional circular array.

### Features

- Fixed dimension arrays of any size.
- Element insertion to the front or back of any dimension.
- Indexing, range and slicing operations.
- Optimized for contiguous memory.
- Support for external types through `AsRef<[T]>` and `AsMut<[T]>`.
- Thorough testing for arrays of smaller dimensionality.
- No external dependencies.

### Usage

The following example demonstrates the basic functionality offered by this
crate.

```rust
// A 1-dimensional circular array of 6 elements.
let mut array = CircularArrayVec::new([6], vec![0, 1, 2, 3, 4, 5]);

array.push_front(0, &[6, 7]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[2, 3, 4, 5, 6, 7]);
array.push_back(0, &[0, 1]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[0, 1, 2, 3, 4, 5]);

// A 2-dimensional array of 3*3 elements.
let mut array = CircularArrayVec::new([3, 3], vec![
    0, 1, 2,
    3, 4, 5,
    6, 7, 8
]);

// Push to the front of axis 0.
array.push_front(0, &[9, 10, 11]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
    1, 2, 9,
    4, 5, 10,
    7, 8, 11
]);

// Push to the back of axis 1.
array.push_back(1, &[12, 13, 14]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
    12, 13, 14,
     1,  2,  9,
     4,  5, 10
]);

// Iterate over index 1 of axis 0 (The second column).
assert_eq!(array.iter_index(0, 1).cloned().collect::<Vec<usize>>(), &[
    13,
     2,
     5
]);
```

### Mutation

`n_circular_array` allows for mutating single elements, or inserting any number
of slices to an axis. Insertion operations expect elements of **row-major**
ordering. Operations accept either an array slice `&[T]`, or an `ExactSizeIterator`
of `&T` elements for `_iter` suffixed methods.
```rust

// A 2-dimensional circular array of 3*2 elements.
let mut array = CircularArrayVec::new([3, 3], vec![
    0, 1, 2,
    3, 4, 5,
    6, 7, 8
]);

// Push two columns to the front of axis 0.
array.push_front(0, &[
     9, 10,
    11, 12,
    13, 14
]);

// Mutate the last element of the array (equivalent to `array.get_mut([2, 2])`).
assert_eq!(array[[2, 2]], 14);
array[[2, 2]] = 99;

assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
    2,  9, 10,
    5, 11, 12,
    8, 13, 99
]);

// Push two rows of zero to the front of axis 1.
let axis_len = array.shape()[1];
array.push_front_iter(1, std::iter::repeat(&0).take(2 * axis_len));

assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
    8, 13, 99,
    0,  0,  0,
    0,  0,  0,
]);
```
See `[CircularArrayMut]`.

### Indexing

`n_circular_array` allows for elements to be accessed by index or slice. Note
that indexing operations take a fixed size array of `N` indices/ranges where `N`
is the dimensionality of the array.

```rust

// A 3-dimensional array of 3*3*2 elements.
let mut array = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17
]);

// Get the first element at index 1 of axis 2 (equivalent to `array.get([0, 0, 1])`).
assert_eq!(array[[0, 0, 1]], 9);

// Get the second and third row.
assert_eq!(array.iter_range(1, 1..3).cloned().collect::<Vec<_>>(), &[
     3,  4,  5,
     6,  7,  8,

    12, 13, 14,
    15, 16, 17
]);

// All columns of row 2, slice 1.
assert_eq!(array.iter_slice([0..3, 2..3, 1..2]).cloned().collect::<Vec<_>>(), &[
    15, 16, 17
]);
```
See `[CircularArrayIndex]` and `[CircularArrayIndexMut]`.

### Resizing/Reshaping

Resizing or reshaping can be achieved by iterating and collecting into a new
`CircularArray`. This functionality is not offered from within the crate to make the
performance implications explicit.

```rust
// A 3-dimensional array of 3*3*2 elements.
let mut array = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17
]);

// Insert a row at index 0.
array.push_front(0, &[3, 6, 9, 12, 15, 18]);
assert_eq!(array.iter().cloned().collect::<Vec<_>>(), &[
     1,  2,  3,
     4,  5,  6,
     7,  8,  9,

    10, 11, 12,
    13, 14, 15,
    16, 17, 18
]);
assert_eq!(array.offset(), &[1, 0, 0]);

// Iterate over index 1 of axis 2 into a 2-dimensional array of shape [3, 3].
let iter = array.iter_slice([0..3, 0..3, 1..2]);
// Operations return `ExactSizeIterator` implementations.
assert_eq!(iter.len(), 9);
let array_2 = CircularArrayVec::from_iter([3, 3], iter.cloned());

assert_eq!(array_2.iter().cloned().collect::<Vec<_>>(), &[
    10, 11, 12,
    13, 14, 15,
    16, 17, 18
]);
assert_eq!(array_2.offset(), &[0, 0]);
```

## Performance

Wrapping contigous slices over the bounds of an axis reduces cache locality,
especially for the innermost dimensions of any `n > 1` array. Where possible,
an array should be oriented where the majority of operations are performed on the
outermost dimension(s). This will allow `n_circular_array` to take contiguous
slices of memory where possible, which can result in operations being reduced to
as little as a single iteration over a contiguous slice, or a single call to
`copy_from_slice` during mutation.

External types implementing `AsRef<[T]>` and `AsMut<[T]>` may also improve performance
over `Vec<T>` or `Box<T>`. If necessary, `AsRef<[T]>` and `AsMut<[T]>` can be delegated
to `unsafe` methods, although this is discouraged.

Finally, for smaller arrays, avoiding a circular array and simply copying (or cloning)
an array window may outperform `n_circular_array`. Benchmark if unsure whether
your use case benefits from `n_circular_array`.

License: MIT OR Apache-2.0
