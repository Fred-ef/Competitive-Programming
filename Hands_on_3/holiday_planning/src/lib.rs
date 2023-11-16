pub fn plan(attractions: Vec<Vec<usize>>) -> usize {
    // Get number of cities and number of days
    let cities = attractions.len();
    let days = attractions[0].len();

    // Initialize the prefix sum matrix
    let mut prefixes: Vec<Vec<usize>> = vec![vec![0; days + 1]; cities + 1];

    // Compute prefix sums for each city
    for i in 1..cities + 1 {
        for j in 1..days + 1 {
            prefixes[i][j] = prefixes[i][j - 1] + attractions[i - 1][j - 1];
        }
    }

    // Initialize the dynamic programming matrix
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; cities + 1]; days + 1];

    // Fill in the dynamic programming matrix
    for d in 1..days + 1 {
        for i in 1..cities + 1 {
            let mut max = 0; // we have no negative values, so 0 is a legal value
            for k in 0..=d {
                let curr = prefixes[i][k] + matrix[d - k][i - 1];
                if curr > max {
                    max = curr;
                }
            }
            matrix[d][i] = max;
        }
    }

    // Return the result for the full holiday
    matrix[days][cities]
}
