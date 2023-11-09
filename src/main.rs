mod ex00;
mod ex01;
mod ex02;

fn main() {
    println!("ex00: 1 + 2 = {}", ex00::adder(1, 2));
	println!("ex01: 1 * 2 = {}", ex01::multiplier(1, 2));
	println!("ex02: graycode({}) = {}", 18, ex02::gray_code(18));
}
