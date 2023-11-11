pub fn eval_formula(formula: &str) -> bool {
    const ERROR_MSG: &str = "Syntax error encountered!";

    let mut stack: Vec<bool> = Vec::new();

    let apply_unary_op = |stack: &mut Vec<bool>, op: fn(bool) -> bool| {
        let op1 = stack.pop().expect(ERROR_MSG);
        stack.push(op(op1));
    };

    let apply_binary_op = |stack: &mut Vec<bool>, op: fn(bool, bool) -> bool| {
        let op2 = stack.pop().expect(ERROR_MSG); // rhs is on the top
        let op1 = stack.pop().expect(ERROR_MSG);
        stack.push(op(op1, op2));
    };

    for c in formula.chars() {
        match c {
            '1' => stack.push(true),
            '0' => stack.push(false),
            '!' => apply_unary_op(&mut stack, |a| !a),
            '&' => apply_binary_op(&mut stack, |a, b| a & b),
            '|' => apply_binary_op(&mut stack, |a, b| a | b),
            '^' => apply_binary_op(&mut stack, |a, b| a ^ b),
            '>' => apply_binary_op(&mut stack, |a, b| !a | b),
            '=' => apply_binary_op(&mut stack, |a, b| a == b),
            _ => panic!("{ERROR_MSG}"),
        }
    }

    match stack.len() {
        1 => stack.pop().expect(ERROR_MSG),
        _ => panic!("{ERROR_MSG}"),
    }
}

#[cfg(test)]
mod eval_formula_test {
    use super::*;
    use std::panic;

    #[test]
    fn general() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));

        assert!(!eval_formula("1!"));
        assert!(eval_formula("0!"));

        assert!(eval_formula("101|&"));
        assert!(eval_formula("1011||="));
        assert!(!eval_formula("010&1|&"));

        assert!(!eval_formula("10>"));
        assert!(eval_formula("01>"));
        assert!(eval_formula("11="));
        assert!(!eval_formula("01="));
        assert!(!eval_formula("11^"));
        assert!(eval_formula("10^"));
    }

    #[test]
    #[should_panic]
    fn non_empty_stack_1() {
        eval_formula("11!");
    }

    #[test]
    #[should_panic]
    fn non_empty_stack_2() {
        eval_formula("10&1");
    }

    #[test]
    fn panic_cases() {
        let mut result;
        result = panic::catch_unwind(|| {
            eval_formula("!");
        });
        assert!(result.is_err(), "The code did not panic");

        result = panic::catch_unwind(|| {
            eval_formula("&");
        });
        assert!(result.is_err(), "The code did not panic");

        result = panic::catch_unwind(|| {
            eval_formula("11&|");
        });
        assert!(result.is_err(), "The code did not panic");

        result = panic::catch_unwind(|| {
            eval_formula("11>=");
        });
        assert!(result.is_err(), "The code did not panic");
    }
}
