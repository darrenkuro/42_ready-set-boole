pub fn adder(a: u32, b: u32) -> u32 {
    let result: u32 = a ^ b;
    match (a & b) << 1 {
        0 => result,
        carry => adder(result, carry),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_general() {
        let mut i: u32 = 1;
        let mut j: u32 = std::u32::MAX - 1;
        while i != 0 {
            assert_eq!(adder(i, j), i + j);
            i <<= 1;
            j >>= 1;
        }
    }

    #[test]
    fn adder_limits() {
        assert_eq!(
            adder(std::u32::MAX, std::u32::MAX),
            std::u32::MAX.wrapping_add(std::u32::MAX)
        );
        assert_eq!(
            adder(std::u32::MIN, std::u32::MIN),
            std::u32::MIN + std::u32::MIN
        )
    }
}
