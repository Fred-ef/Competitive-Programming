# Longest Increasing Subsequence (LIS)

## Problem Description

Given an array of integers, the task is to find the length of the longest strictly increasing subsequence from the given array. In other words, find the maximum length of a subsequence where the elements are in increasing order.

## Approach

The problem can be efficiently solved using an optimized approach stemming from a key consideration on the classical dynamic programming solution for LIS: given current index i, there's no need to consider every possible LIS(j), for all j < i, to take the longest one; we can in fact exploit "dominant positions", that is positions i that "dominate" other positions j for the following reason:
- LIS(i) >= LIS(j) && S[i] <= S[j]  (with S being the original sequence)
All those positions j can in fact be discarded since i will always be better.

Here's a step-by-step explanation of the approach we take to implement the discussed solution:

1. Initialize an empty stack to keep track of the current increasing subsequence.

2. Iterate through the elements of the input array one by one.

3. For each element `num` in the array:
   - If the stack is empty or if `num` is greater than or equal to the top element of the stack, push `num` onto the stack. This means we have found a valid element to extend the current subsequence.

   - If `num` is less than the top element of the stack, perform a binary search on the stack to find the index `k` of the smallest element in the stack that is greater than or equal to `num`. Replace `stack[k]` with `num`. This ensures that the stack continues to contain a valid increasing subsequence.

4. Continue this process for all elements in the array, and the elements left in the stack represent the longest increasing subsequence.

5. The result is the content of the stack, which represents one of the possible LIS. The length of this subsequence can be obtained using the `len()` method.

## Time and Space complexity
In this solution, we simply iterate on the given array S and, for each element S[i], we have at most O(lgn) steps to perform due to the binary search when the current element is smaller than the top of the stack, which gives us a time complexity of O(n*lg(n)).
The solution only uses a vector of O(n) elements to store the result.

## Input

The vector on which to compute the longest increasing subsequence (already provided in the main method).

## Output

The length of the longest increasing subsequence, along with a few test results.