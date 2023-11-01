# Subset Sum

## Problem Description

Given a set of non-negative integers and a target sum, the task is to check if there is a subset of the given set whose sum is equal to the given target sum.

## Solution Approach

We can solve this problem using dynamic programming. Create a boolean dynamic programming matrix `dp` with dimensions `(n + 1) x (target_sum + 1)`, where `n` is the number of elements in the given set. The entry `dp[i][j]` is set to `true` if there exists a subset of the first `i` elements that sums up to `j`, and `false` otherwise.

As a subproblem, we consider sums less than the target sum, ranging from 0 to `target_sum - 1`. To reach the target sum `j` with the `i`-th element, we first need to reach the sum `j - A[i]` with the previous `i-1` elements.

We fill the `dp` matrix using the following recurrence relation:
```rust
dp[i][j] = dp[i-1][j] || dp[i-1][j - A[i]]

We initialize the first row of dp (i.e., dp[0][j]) to 'false' because without any elements, we cannot achieve any target sum other than 0. We also initialize the first column of dp (i.e., dp[i][0]) to 'true' because we can always achieve a sum of 0 by not including any elements. Then, we 'continue' filling the dp matrix using the recurrence relation.
At the end, the value 'in' the bottom-right corner, dp[n][target_sum], will indicate whether there is a subset that sums up to the target sum.

The cost of the solution is dominated by the construction of the dynamic programming matrix, thus the time complexity of the algorithm is O(n*sum);

## Usage example

```rust
fn main() {
    let arr = vec![3, 34, 4, 12, 5, 2];
    let target_sum = 9;

    if subset_sum_exists(&arr, target_sum) {
        println!("Subset with sum {} exists.", target_sum);
    } else {
        println!("No subset with sum {} exists.", target_sum);
    }
}
