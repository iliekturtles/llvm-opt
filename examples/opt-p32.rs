extern crate llvm_opt;

use llvm_opt::{P};

#[derive(Clone, Copy, Debug)]
pub struct X;

#[inline(never)]
fn calc(v: f32) -> P<X> {
    let a = P::<X>::new(v * 1.234_f32);
    let b = P::<X>::new(5.0_f32);

    a / b
}

fn main() {
    let v: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let c = calc(v);

    println!("{:?}", c.get_value());
}
