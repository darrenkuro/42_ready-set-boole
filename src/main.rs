use std::panic;

mod ex00;
mod ex01;
mod ex02;
mod ex03;

fn main() {
    println!("ex00: 1 + 2 = {}", ex00::adder(1, 2));
	println!("ex01: 1 * 2 = {}", ex01::multiplier(1, 2));
	println!("ex02: graycode({}) = {}", 18, ex02::gray_code(18));
	let _ = panic::catch_unwind(|| {
		println!("ex03: 1! = {}", ex03::eval_formula("11!"));
	});
	println!("hello");
	// panic!("test");
}
