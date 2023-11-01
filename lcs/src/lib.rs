pub fn lcs(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    // Create a matrix to store the lengths of LCS for substrings
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // Fill the matrix using dynamic programming
    for i in 1..=len1 {
        for j in 1..=len2 {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // The length of LCS is stored in dp[len1][len2]
    dp[len1][len2]
}