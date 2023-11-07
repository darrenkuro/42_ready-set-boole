fn adder(a: u32, b: u32) -> u32 {
    let result = a ^ b;
    match (a & b) << 1 {
        0 => result,
        carry => adder(result, carry),
    }
}

fn main() {
    let x = adder(3, 4);
    println!("test is {x}");
}
