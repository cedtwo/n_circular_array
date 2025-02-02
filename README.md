# N Circular Array
An n-dimensional circular array.

## Features

- Fixed dimension arrays of any size.
- Element insertion to the front or back of any dimension.
- Indexing, range and slicing operations.
- Performant operations for sequentual `Copy` type elements.

## Usage

```rust
// A 1-dimensional circular array of 6 elements.
let mut array = CircularArrayVec::new([6], vec![0, 1, 2, 3, 4, 5]);

array.push_front(0, &[6, 7]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[2, 3, 4, 5, 6, 7]);
array.push_back(0, &[0, 1]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[0, 1, 2, 3, 4, 5]);

// A 2-dimensional circular array of 3^2 elements.
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
```

## Mutation

`n_circular_array` allows for mutating single elements, or inserting any number
of slices to an axis. Insertion operations expect elements arranged as a **row-major**
slice. That is, insertion of two columns arranged as a row-major contiguous
slice would be the elements of column one, interspersed by those of column two.
This is the default behaviour when slicing into `ndarray` or `nalgebra` arrays.

```rust

// A 2-dimensional circular array of 3^2 elements.
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
```
See `[CircularArrayMut]`.

## Indexing

`n_circular_array` allows for elements to be accessed by index or slice.

```rust

// A 2-dn_n_imensional circular array of 3 * 3 * 2 elements.
let mut array = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17
]);

// Get the first element of axis 2 (equivalent to `array.get([0, 0, 2])`).
assert_eq!(array[[0, 0, 1]], 9);

// Get the second and third row.
assert_eq!(array.iter_range(1, 1..3).cloned().collect::<Vec<_>>(), &[
     3,  4,  5,
     6,  7,  8,

    12, 13, 14,
    15, 16, 17
]);

// Get the third row of the second slice of axis 2.
assert_eq!(array.iter_slice([0..3, 2..3, 1..2]).cloned().collect::<Vec<_>>(), &[
    15, 16, 17
]);
```
See `[CircularArrayIndex]` and `[CircularArrayIndexMut]`.

## Resizing/Reshaping

No resizing or reshaping operations are offered, however the same functionality
can be achieved by iterating and collecting into a new `array`. No method is
offered for this functionality to make the performance implications explicit.

```rust
// A 2-dimensional circular array of 3 * 3 * 2 elements.
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

// Iterate over index 1 of axis 2 into a new array of size [3, 3].
let iter = array.iter_slice([0..3, 0..3, 1..2]).cloned();
let array_2 = CircularArrayVec::from_iter([3, 3], iter);

assert_eq!(array_2.iter().cloned().collect::<Vec<_>>(), &[
    10, 11, 12,
    13, 14, 15,
    16, 17, 18
]);
assert_eq!(array_2.offset(), &[0, 0]);
```

# Performance

The inner dimensions of any `n > 1` array are impacted the most by cache locality
(or a lack thereof). Wrapping contigous slices over the bounds of an axis further
reduces cache locality. Where possible, an array should be oriented in which the
majority of operations are performed on the outermost dimension(s). `n_circular_array`
will take contiguous slices of memory where possible. For elements that implement
`Copy`, this can result in an insertion of as little as a single call to `copy_from_slice`.


License: MIT OR Apache-2.0
