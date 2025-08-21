# n_circular_array

## N Circular Array
An n-dimensional circular array.

### Features

- Fixed dimension arrays of any size.
- Element retrieval by `N` dimensional index, range or slice.
- Element insertion to the front or back of any axis.
- `N` dimensional translation over a source array.
- Support for external types through `AsRef<[T]>` and `AsMut<[T]>`.
- Optimized for contiguous memory.
- Thorough testing for arrays of smaller dimensionality.
- No external dependencies.

### Mutation

`n_circular_array` supports the following mutating operations:
- Insert elements to either side of an axis.
- Translate over a source array.
- Mutate individual elements.

#### Insertion

Elements are inserted by providing a **row-major** slice or iterator with a length
equal to an **exact** multiple of the given axis length. That is, a call to insert
two rows must be provided **exactly** two rows of elements.

```rust

// A 2-dimensional circular array of 3*3 elements.
let mut array = CircularArrayVec::new([3, 3], vec![
    0, 1, 2,
    3, 4, 5,
    6, 7, 8
]);

// Push the elements 9..12 (one row) to the front of axis 1.
array.push_front(1, &[
     9, 10, 11,
]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
     3,  4,  5,
     6,  7,  8,
     9, 10, 11,
]);

// Push the elements 12..18 (two columns) to the front of axis 0.
let axis_len = array.shape()[0];
array.push_front(0, &[12, 13, 14, 15, 16, 17]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
      5, 12, 13,
      8, 14, 15,
     11, 16, 17,
]);

// Push the elements 19..22 (one row) to the back of axis 1.
array.push_back(1, &[
     19, 20, 21,
]);
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
     19, 20, 21,
      5, 12, 13,
      8, 14, 15,
]);
```

#### Translation

Translation methods simplify mapping the elements of a *source* array to the circular
array. Translation methods expect the array `origin`, or the position of the
`[0; N]` element within the source array, and a translation on an axis. The provided
`el_fn` function will recieve contiguous `[Range<usize>; N]` slices for mapping
the new elements from the source to the circular array. `CircularArray` **only**
handles slicing and mutation, and translation logic (the current translation, out of
bound translation etc.) must be maintained by the user.

In the following example, rather than passing the `[Range<usize>; N]` slice to a
3rd-party crate, we define the source array [`Strides`], then call [`Strides::flatten_range`]
to get a single contiguous range for slicing (requires feature `strides`).
```rust
// A [5, 5] source array.
let src = [
     0,  1,  2,  3,  4,
     5,  6,  7,  8,  9,
    10, 11, 12, 13, 14,
    15, 16, 17, 18, 19,
    20, 21, 22, 23, 24,
];
// Strides used for flattening `N` dimensional indices.
let src_strides = Strides::new(&[5, 5]);

// Slice function.
let el_fn = |mut index: [Range<usize>; 2]| {
    &src[src_strides.flatten_range(index)]
};

// A [3, 3] circular array positioned at `[0, 0]`.
let mut origin = [0, 0];
let mut dst = CircularArray::new([3, 3], vec![
     0,  1,  2,
     5,  6,  7,
    10, 11, 12
]);

// Translate by 2 on axis 0 (Pushes 2 columns to front of axis 0).
let axis = 0;
let n = 2;
dst.translate_front(axis, n, origin, el_fn);
origin[axis] += n as usize;

assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
     2,  3,  4,
     7,  8,  9,
    12, 13, 14,
]);

// Translate by 1 on axis 1 (Pushes 1 row to front of axis 1).
let axis = 1;
let n = 1;
dst.translate_front(axis, n, origin, el_fn);
origin[axis] += n as usize;

assert_eq!(dst.iter().cloned().collect::<Vec<usize>>(), &[
     7,  8,  9,
    12, 13, 14,
    17, 18, 19,
]);

assert_eq!(origin, [2, 1]);
```

### Indexing and Slicing

`n_circular_array` supports the following indexing operations:
- Access elements by axis slice.
- Access elements by `N` dimensional slice.
- Access individual elements by index.

### Slicing an axis

All elements of an axis can be iterated over by index or range. Calling
[`CircularIndex::iter_index`] returns an iterator of elements of a shape
equal to the shape of the circular array, with the specified axis set to `1`.
Calling [`CircularIndex::iter_range`] returns an iterator of elements of a
shape equal to the shape of the circular array, with the specified axis set to
the length of the given range.

```rust

// A 3-dimensional circular array of 3*3*2 elements.
let array = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17,
]);

// Iterate over index 1 of axis 0 (shape [1, 3, 2]).
assert_eq!(array.iter_index(0, 1).cloned().collect::<Vec<usize>>(), &[
     1,
     4,
     7,

    10,
    13,
    16,
]);
// Iterate over indices 1..3 of axis 1 (shape [3, 2, 2]).
assert_eq!(array.iter_range(1, 1..3).cloned().collect::<Vec<usize>>(), &[
     3,  4,  5,
     6,  7,  8,

    12, 13, 14,
    15, 16, 17,
]);
```

### Slicing the array

Calling [`CircularIndex::iter_slice`] can be used to iterate over an `N`
dimensional slice of the array. This can be used to limit iteration to an
exact subset of elements.

```rust

// A 3-dimensional circular array of 3*3*2 elements.
let array = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17,
]);

// Iterate over:
//     - index 1 of axis 0,
//     - range 0..3 of axis 1 (all elements),
//     - index 1 of axis 2.
// (shape [1, 2, 1], equivalent to [2]).
assert_eq!(array.iter_slice([1..2, 0..3, 1..2]).cloned().collect::<Vec<usize>>(), &[
    10,
    13,
    16,
]);
// Iterate over:
//     - range 0..2 of axis 0,
//     - range 1..3 of axis 1,
//     - index 0 of axis 2.
// (shape [2, 2, 1], equivalent to [2, 2]).
assert_eq!(array.iter_slice([0..2, 1..3, 0..1]).cloned().collect::<Vec<usize>>(), &[
     3,  4,
     6,  7,
]);
```

`n_circular_array` resizing or reshaping functionality can be achieved by using
[`CircularIndex::iter_slice`] and collecting into a new array.

```rust

// A 3-dimensional circular array of 3*3*2 elements.
let array3 = CircularArrayVec::new([3, 3, 2], vec![
     0,  1,  2,
     3,  4,  5,
     6,  7,  8,

     9, 10, 11,
    12, 13, 14,
    15, 16, 17,
]);

// Iterate over:
//     - range 0..2 of axis 0,
//     - range 1..3 of axis 1,
//     - index 0 of axis 2.
// (shape [2, 2, 1], equivalent to [2, 2]).
let iter = array3.iter_slice([0..2, 1..3, 0..1]).cloned();

// A 2-dimensional circular array of 3*2 elements.
let array2 = CircularArrayVec::from_iter([2, 2], iter);

assert_eq!(array2.iter().cloned().collect::<Vec<usize>>(), &[
     3,  4,
     6,  7,
]);
```

#### Index and IndexMut

Finally, `n_circular_array` supports [`std::ops::Index`] and [`std::ops::IndexMut`]
taking an `N` dimensional index (`[usize; N]`) as argument.

```rust

// A 2-dimensional circular array of 3*3 elements.
let mut array = CircularArrayVec::new([3, 3], vec![
    0, 1, 2,
    3, 4, 5,
    6, 7, 8
]);

array[[1, 1]] += 10;
assert_eq!(array.iter().cloned().collect::<Vec<usize>>(), &[
    0,  1, 2,
    3, 14, 5,
    6,  7, 8
]);
```

## Features

Feature | Description
---|---|---
`strides` | Exports [`Strides`] for flattening `N` dimensional indices during translation.

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
