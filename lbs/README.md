# Longest Bitonic Sequence

## Problem Description

Given an array of positive integers, you need to find the maximum length of a Bitonic subsequence. A subsequence of an array is called Bitonic if it is first strictly increasing and then strictly decreasing.

## Solution

To find the maximum length of the Longest Bitonic Subsequence, we will use dynamic programming. We will calculate two subsequences for each element in the array: the Longest Increasing Subsequence (LIS) from the left and the Longest Decreasing Subsequence (LDS) from the right.

1. Initialize two arrays, `lis` and `lds`, with each element set to 1. These arrays will store the length of the LIS and LDS for each element.

2. Calculate the LIS from left to right:
   - For each element at index `i`, compare it with elements at indices `j` from `0` to `i-1`.
   - If `arr[i]` is greater than `arr[j]`, and if `lis[i]` is less than `lis[j] + 1`, update `lis[i]` to `lis[j] + 1`.

3. Calculate the LDS from right to left:
   - For each element at index `i` (starting from the end), compare it with elements at indices `j` from `i+1` to the end.
   - If `arr[i]` is greater than `arr[j`, and if `lds[i]` is less than `lds[j] + 1`, update `lds[i]` to `lds[j] + 1`.

4. Find the maximum bitonic subsequence length:
   - For each element at index `i`, calculate the bitonic subsequence length as `lis[i] + lds[i] - 1`. Subtract `1` to avoid double counting.
   - Update the `max_length` with the maximum value obtained.

5. The `max_length` will represent the length of the Longest Bitonic Subsequence.

## Time and Space complexity
Since we only scan the LIS and LDS arrays from left to right at the same time (computing a sum at each step) in the lbs procedure, the time complexity is dominated by the lis() and lbs() procedures, which compute the LIS and LSD respectively, both in O(n*lg(n)) time; therefore, the overall time complexity is O(n*lg(n)).
We only use two support arrays to store the LIS and the LDS, but each procedure actually requires O(n^2) space to run; this gives us an overall space complexity of O(n^2).

## Input

The vector on which to compute the LBS (already provided in the main method).

## Output

The length of the longest bitonic sequence, along with a few test results.