pub fn subset_sum(arr: &[u64], target_sum: u64) -> bool {
    let n = arr.len();
    
    // Create a DP matrix to store subset sum possibilities
    let mut dp = vec![vec![false; (target_sum + 1) as usize]; n + 1];

    // Base case: if the target sum is 0, it's always possible to form it with an empty subset
    for i in 0..=n {
        dp[i][0] = true;
    }

    // Fill the DP matrix
    for i in 1..=n {
        for j in 1..=target_sum as usize {
            if arr[i - 1] <= j as u64 {
                // If the current element can be included, check if there is a subset with the remaining sum
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - arr[i - 1] as usize];
            } else {
                // If the current element is larger than the current sum, exclude it
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    // The final result is stored in dp[n][target_sum]
    dp[n][target_sum as usize]
}