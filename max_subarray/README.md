## Problem

Given a sequence of integers, find the subsequence with the maximum sum and return it.

## Solution

The proposed solution is implemented in Rust. The `max_sub_array` function takes a vector of integers `nums` as input and returns the maximum sum of a subsequence within this vector.

The code uses an approach based on Kadane's algorithm to find the subsequence with the maximum sum. Here's how it works:

1. We initialize two variables, `max` and `sum`, both set to `nums[0]`, which represents the first element of the vector.

2. We iterate through the vector starting from the second element onwards.

3. For each element `num` in the vector, we check if the current sum `sum` is greater than zero. If it is, we add `num` to `sum`. Otherwise, we assign `num` to `sum`. This step keeps track of the sum of the current subsequence.

4. In each iteration, we check if `sum` is greater than `max`. If it is, we update `max` with the value of `sum`. This allows us to find the maximum sum encountered so far.

5. At the end of the loop, `max` will contain the maximum sum of the subsequence.

## Time and Space complexity
The algorithm scans the input vector linearly making constant-time operations; therefore, the time complexity is O(n).
Only 2 supporting variables are utilized, making the space complexity O(1).

## Input

The vector on which to compute the maximum subarray (already provided in the main method).

## Output

The interval representing the maximum subarray, along with some test results.