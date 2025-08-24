#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

use n_circular_array::{CircularArrayBox, CircularMut};

/// Bench push slice methods for an array of `d` dimensions of `n` elements.
macro_rules! bench_push_slice {
    (
        $name:ident,
        $d:literal,
        $n:literal
    ) => {
        mod $name {
            use super::*;

            bench_push_slice_method!(push_front, $d, $n);
            bench_push_slice_method!(push_front_raw, $d, $n);
            bench_push_slice_method!(push_back, $d, $n);
            bench_push_slice_method!(push_back_raw, $d, $n);
        }
    };
}

/// Bench a specified push slice method for an array of `d` dimensions of `n` elements.
macro_rules! bench_push_slice_method {
    (
        $method:ident,
        $d:literal,
        $n:literal
    ) => {
        #[bench]
        fn $method(bencher: &mut Bencher) {
            const SHAPE: [usize; $d] = [$n; $d];

            let mut m = CircularArrayBox::from_iter(SHAPE, 0..SHAPE.iter().product::<usize>());
            let slice = [99].repeat(usize::pow($n, ($d - 1) as u32));
            let mut axis = 0;

            bencher.iter(|| {
                axis = (axis + 1) % $d;
                m.$method(axis, &slice);
            });

            black_box(m);
        }
    };
}

/// Bench push methods for an array of `d` dimensions of `n` elements.
macro_rules! bench_push_iter {
    (
        $name:ident,
        $d:literal,
        $n:literal
    ) => {
        mod $name {
            use super::*;

            bench_push_iter_method!(push_front_iter, $d, $n);
            bench_push_iter_method!(push_front_raw_iter, $d, $n);
            bench_push_iter_method!(push_back_iter, $d, $n);
            bench_push_iter_method!(push_back_raw_iter, $d, $n);
        }
    };
}

/// Bench a specified push iterator method for an array of `d` dimensions of `n` elements.
macro_rules! bench_push_iter_method {
    (
        $method:ident,
        $d:literal,
        $n:literal
    ) => {
        #[bench]
        fn $method(bencher: &mut Bencher) {
            const SHAPE: [usize; $d] = [$n; $d];

            let mut m = CircularArrayBox::from_iter(SHAPE, 0..SHAPE.iter().product::<usize>());
            let slice = [99].repeat(usize::pow($n, ($d - 1) as u32));
            let mut axis = 0;

            bencher.iter(|| {
                axis = (axis + 1) % $d;
                m.$method(axis, &slice);
            });

            black_box(m);
        }
    };
}

/// Bench push methods for an array of `d` dimensions of `n` elements.
#[allow(unused)]
macro_rules! bench_translate {
    (
        $name:ident,
        $d:literal,
        $n:literal
    ) => {
        mod $name {
            use super::*;
            use n_circular_array::Strides;

            bench_translate_method!(translate_front, $d, $n);
            bench_translate_method!(translate_back, $d, $n);
        }
    };
}

/// Bench a specified push slice method for an array of `d` dimensions of `n` elements.
#[allow(unused)]
macro_rules! bench_translate_method {
    (
        $method:ident,
        $d:literal,
        $n:literal
    ) => {
        #[bench]
        fn $method(bencher: &mut Bencher) {
            const DST_SHAPE: [usize; $d] = [$n; $d];

            let mut dst =
                CircularArrayBox::from_iter(DST_SHAPE, 0..DST_SHAPE.iter().product::<usize>());

            let src = [99].repeat(usize::pow($n, ($d - 1) as u32));
            let src_fn = |idx: [std::ops::Range<usize>; $d]| &src[0..idx[0].len()];

            let mut axis = 0;
            let mut origin = [$d - 1; $d];

            bencher.iter(|| {
                axis = (axis + 1) % $d;
                origin[axis] = origin[axis] + 1 % $d;

                dst.$method(axis, $d - 1, origin, src_fn);
            });

            black_box(dst);
        }
    };
}

mod push_slice {
    use super::*;

    bench_push_slice!(d2_n05, 2, 5);
    bench_push_slice!(d2_n10, 2, 10);
    bench_push_slice!(d3_n05, 3, 5);
    bench_push_slice!(d3_n10, 3, 10);
    bench_push_slice!(d4_n05, 4, 5);
    bench_push_slice!(d4_n10, 4, 10);
}

mod push_iter {
    use super::*;

    bench_push_iter!(d2_n05, 2, 5);
    bench_push_iter!(d2_n10, 2, 10);
    bench_push_iter!(d3_n05, 3, 5);
    bench_push_iter!(d3_n10, 3, 10);
    bench_push_iter!(d4_n05, 4, 5);
    bench_push_iter!(d4_n10, 4, 10);
}

#[cfg(feature = "strides")]
mod translate {
    use super::*;

    bench_translate!(d2_n05, 2, 5);
    bench_translate!(d2_n10, 2, 10);
    bench_translate!(d3_n05, 3, 5);
    bench_translate!(d3_n10, 3, 10);
    bench_translate!(d4_n05, 4, 5);
    bench_translate!(d4_n10, 4, 10);
}
