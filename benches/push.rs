#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

use n_circular_array::{CircularArrayBox, CircularArrayMut};

/// Bench push methods for an array of `d` dimensions of `n` elements.
macro_rules! bench {
    (
        $name:ident,
        $d:literal,
        $n:literal
    ) => {
        mod $name {
            use super::*;

            bench_push!(push_front, $d, $n);
            bench_push!(push_front_raw, $d, $n);
            bench_push!(push_back, $d, $n);
            bench_push!(push_back_raw, $d, $n);
        }
    };
}

/// Bench a specified push method for an array of `d` dimensions of `n` elements.
macro_rules! bench_push {
    (
        $method:ident,
        $d:literal,
        $n:literal
    ) => {
        #[bench]
        fn $method(bencher: &mut Bencher) {
            const SHAPE: [usize; $d] = [$n; $d];

            let mut m = CircularArrayBox::from_iter(SHAPE, 0..SHAPE.iter().product::<usize>());
            let slice = [99]
                .repeat(usize::pow($n, ($d - 1) as u32))
                .into_iter()
                .collect::<Vec<_>>();
            let mut axis = 0;

            bencher.iter(|| {
                axis = (axis + 1) % $d;
                m.$method(axis, &slice);
            });

            black_box(m);
        }
    };
}

bench!(d2_n3, 2, 3);
bench!(d3_n3, 3, 3);
bench!(d4_n3, 4, 3);
