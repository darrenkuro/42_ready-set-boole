/// Generate all possible combinations of truth values for `variable_count` variables.
pub fn generate_combinations(variable_count: usize) -> Vec<Vec<bool>> {
    // There are 2^n possible combinations for n variables
    let total_combinations = 1 << variable_count; // 1 << n is the same as 2^n

    // Iterate over each possible combination
    (0..total_combinations)
        .map(|i| {
            // For each combination, create a vector of truth values
            (0..variable_count)
                .map(|bit| {
                    // Check the truth value for each variable
                    // by checking each bit's status (0 or 1)
                    (i >> bit) & 1 == 1
                })
                .collect()
        })
        .collect()
}
