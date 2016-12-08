#[inline(never)]
fn calc(v: f32) -> f32 {
    let a = v * 1.234_f32;
    let b = 5.0_f32;

    a / b
}

fn main() {
    let v: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let c = calc(v);

    println!("{:?}", c);
}
