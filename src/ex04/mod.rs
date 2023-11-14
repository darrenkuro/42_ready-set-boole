use crate::ex03::eval_formula;

// Don't print it until the end to ensure it doesn't print something then panic or exit

fn print_header(vars: &Vec<char>) {
    for c in vars.iter() {
        print!("| {c} ");
    }
    println!("| = |");

    for _ in 0..vars.len() {
        print!("|---");
    }
    println!("|---|");
}

pub fn print_truth_table(formula: &str) {
    const OPERATORS: &str = "!&|^>=";

    if !formula
        .chars()
        .all(|c| OPERATORS.contains(c) || c.is_ascii_uppercase())
    {
        panic!("Invalid character!");
    }

    let mut vars: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    vars.sort_unstable(); // unstable sort is faster
    vars.dedup(); // get unique

    print_header(&vars);

    let len = vars.len();
    for i in 0..2u32.pow(len as u32) {
        let mut formula = String::from(formula);
        for (n, c) in vars.iter().enumerate() {
            let val = if i & (1 << (len - 1 - n)) != 0 {
                print!("| 1 ");
                "1"
            } else {
                print!("| 0 ");
                "0"
            };
            formula = formula.replace(*c, val);
        }
        // cast to number to print 1/0 instead of true/false
        println!("| {} |", eval_formula(&formula) as i8);
    }
}

#[cfg(test)]
mod print_truth_table_test {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_char_1() {
        print_truth_table("Aa");
    }

    #[test]
    #[should_panic]
    fn invalid_char_1_2() {
        print_truth_table("AB?");
    }
}
