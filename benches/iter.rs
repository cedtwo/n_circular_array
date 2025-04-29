#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

use n_circular_array::{CircularArrayBox, CircularArrayIndex};

/// Bench iter methods for an array of `d` dimensions of `n` elements.
macro_rules! bench {
    (
        $name:ident,
        $d:literal,
        $n:literal
    ) => {
        mod $name {
            use super::*;

            bench_iter!(iter, $d, $n);
            bench_iter!(iter_raw, $d, $n);
            bench_iter!(iter_index(0, 1), $d, $n);
            bench_iter!(iter_index_raw(0, 1), $d, $n);
            bench_iter!(iter_range(0, 1..$d), $d, $n);
            bench_iter!(iter_range_raw(0, 1..$d), $d, $n);
        }
    };
}

/// Bench a specified push method for an array of `d` dimensions of `n` elements.
macro_rules! bench_iter {
    (
        $method:ident $( ( $( $arg:expr ),* ) )? ,
        $d:literal,
        $n:literal
    ) => {
        #[bench]
        fn $method(bencher: &mut Bencher) {
            const SHAPE: [usize; $d] = [$n; $d];

            let m = CircularArrayBox::from_iter(SHAPE, 0..SHAPE.iter().product::<usize>());

            bencher.iter(|| {
                m.$method( $( $( $arg ),* )? ).for_each(|i| {
                    black_box(i);
                });
            });

            black_box(m);
        }
    };
}

bench!(d2_n05, 2, 5);
bench!(d2_n10, 2, 10);
bench!(d3_n05, 3, 5);
bench!(d3_n10, 3, 10);
bench!(d4_n05, 4, 5);
bench!(d4_n10, 4, 10);
