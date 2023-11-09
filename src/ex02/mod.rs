pub fn gray_code(n: u32) -> u32 {
	n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_general() {
        assert_eq!(gray_code(10), 15);
		assert_eq!(gray_code(16), 0b11000);
		assert_eq!(gray_code(17), 0b11001);
		assert_eq!(gray_code(18), 0b11011);
		assert_eq!(gray_code(19), 0b11010);
		assert_eq!(gray_code(20), 0b11110);
    }
}
