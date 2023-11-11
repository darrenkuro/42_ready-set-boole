mod ex00;
mod ex01;
mod ex02;
mod ex03;
mod ex04;

fn main() {
    const SEP_WIDTH: usize = 10;
    const SPACE_WIDTH: usize = 5;

    println!(
        "\n{0}{1}ex00{1}{0}",
        "=".repeat(SEP_WIDTH),
        " ".repeat(SPACE_WIDTH)
    );
    println!("1 + 2 = {}", ex00::adder(1, 2));
    println!("4242 + 666 = {}", ex00::adder(4242, 666));

    println!(
        "\n{0}{1}ex01{1}{0}",
        "=".repeat(SEP_WIDTH),
        " ".repeat(SPACE_WIDTH)
    );
    println!("11 * 22 = {}", ex01::multiplier(11, 22));
    println!("42 * 100 = {}", ex01::multiplier(42, 100));

    println!(
        "\n{0}{1}ex02{1}{0}",
        "=".repeat(SEP_WIDTH),
        " ".repeat(SPACE_WIDTH)
    );
    println!("graycode({}) = {} ({1:b})", 10, ex02::gray_code(10));
    println!("graycode({}) = {} ({1:b})", 18, ex02::gray_code(18));
    println!("graycode({}) = {} ({1:b})", 20, ex02::gray_code(20));

    println!(
        "\n{0}{1}ex03{1}{0}",
        "=".repeat(SEP_WIDTH),
        " ".repeat(SPACE_WIDTH)
    );
    println!("10& -> {}", ex03::eval_formula("10&"));
    println!("10| -> {}", ex03::eval_formula("10|"));
    println!("11> -> {}", ex03::eval_formula("11>"));
    println!("10= -> {}", ex03::eval_formula("10="));
    println!("1011||= -> {}", ex03::eval_formula("1011||="));

    println!(
        "\n{0}{1}ex04{1}{0}",
        "=".repeat(SEP_WIDTH),
        " ".repeat(SPACE_WIDTH)
    );
    println!("AB&C|");
    ex04::print_truth_table("AB&C|");

    println!("\nA!");
    ex04::print_truth_table("A!");

    println!("\nAA=");
    ex04::print_truth_table("AA=");
}
