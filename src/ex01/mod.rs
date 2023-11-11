use crate::ex00::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
	let mut a = a;
	let mut b = b;
	while b > 0 {
		if b & 1 == 1 {
			result = adder(result, a);
		}
		a <<= 1;
		b >>= 1;
	}
	result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplier_general() {
        let mut i: u32 = 1;
        let mut j: u32 = std::u32::MAX - 1;
        while i != 0 {
            assert_eq!(multiplier(i, j), i * j);
            i <<= 1;
            j >>= 1;
        }
    }

    #[ignore]
    fn multiplier_limits() {
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
