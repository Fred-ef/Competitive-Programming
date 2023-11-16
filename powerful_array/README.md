# Powerful Array

## Problem Statement

An array A of positive integers a1, a2, ..., an is given. Let us consider its arbitrary subarray al, al + 1..., ar, where 1 ≤ l ≤ r ≤ n. For every positive integer s denote by Ks the number of occurrences of s into the subarray. We call the power of the subarray the sum of products Ks·Ks·s for every positive integer s. The sum contains only finite number of nonzero summands as the number of different values in the array is indeed finite.

You should calculate the power of t given subarrays.

Every element of the array ai is such that (1 ≤ ai ≤ 10^6)

## Solution

To solve this problem, we can use an approach based on the Mo's algorithm, which allows us to efficiently calculate the power of subarrays.

The key idea is to divide array A into blocks of size sqrt(n), where n is the length of A. This enables us to handle queries efficiently and reduce the overall number of operations (specifically, the overall number of movements over the array A).

Next, we group queries based on the block in which they start. Within each block, we sort the queries based on their ending position, so that we can process the queries in order.

To answer the queries, we use two indices, left and right, representing the left and right endpoints of the current subarray. We start with left and right at the beginning of the first block.

For each query, we move the left and right indices to cover the subarray specified by the query. During this process, we use a support array called `occ` to keep track of element occurrences within the current subarray. Additionally, we calculate the current subarray's power based on `occ` and on the value of the current element (as per the formula illustrated above).

The efficiency lies in updating the current power when moving the left and right indices. When adding an element to the subarray, we first subtract the previous power (which is no longer valid) and then add the current power based on the new occurrences; we only have to move a maximum of sqrt(n) steps in any direction when passing from one query to the next in the same block.

Once the power is calculated for each query, we store the results in a results vector, `ans`.

## Time and Space complexity

Since we perform a maximum of sqrt(n) steps when passing from one query to the next, and we have sqrt(n) blocks of sqrt(n) elements each, the overall time complexity of the procedure is O(srqt(n) * n).
We use a support vector in which we store the subarray-queries in an ordered-fashion, which takes O(t) space. We also utilize a support vector of size k, where k is the maximum value we have in the input array; since the input-values are bound (<= 10^6), this technically takes O(1) space, as it is constant, though it can be much larger than the actual size of the array.

## Input

The array and all queries (already provided in the main method).

## Output

The results for each of the queries, along with some test results.