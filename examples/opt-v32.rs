extern crate llvm_opt;

use llvm_opt::{V};

#[inline(never)]
fn calc(v: f32) -> V {
    let a = V::new(v * 1.234_f32);
    let b = V::new(5.0_f32);

    a / b
}

fn main() {
    let v: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let c = calc(v);

    println!("{:?}", c.get_value());
}
