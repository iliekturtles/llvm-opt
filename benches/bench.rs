#![feature(test)]

extern crate llvm_opt;
extern crate test;

use llvm_opt::{V, P};
use test::{Bencher};

#[derive(Clone, Copy, Debug)]
struct X;

#[bench]
fn div_v32(b: &mut Bencher) {
    let i = test::black_box(1000);
    let s = test::black_box(V::new(0.0_f32));
    let v = test::black_box(V::new(123.456_f32));

    b.iter(|| {
        (0..i).fold(s, |a, _| a / v)
    });
}

#[bench]
fn div_p32(b: &mut Bencher) {
    let i = test::black_box(1000);
    let s = test::black_box(P::<X>::new(0.0_f32));
    let v = test::black_box(P::<X>::new(123.456_f32));

    b.iter(|| {
        (0..i).fold(s, |a, _| a / v)
    });
}

#[bench]
fn div_f32(b: &mut Bencher) {
    let i = test::black_box(1000);
    let s = test::black_box(0.0_f32);
    let v = test::black_box(123.456_f32);

    b.iter(|| {
        (0..i).fold(s, |a, _| a / v)
    });
}
