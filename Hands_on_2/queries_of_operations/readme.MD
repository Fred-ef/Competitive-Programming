# Queries of operations

## Problem description

You are given an array A[1, n] of n integers and an array O[1, m] of m operations. Each operation is a triple hl, r, di, with 1 ≤ l ≤ r ≤ n, and it increases the entries A[l], A[l + 1], . . . , A[r] by value d, i.e., A[l] = A[l] + d.
You are also given k queries. Each query is a pair ha, bi, with 1 ≤ a ≤ b ≤ m and it asks to execute all the operations O[a], O[a + 1], . . . , O[b].
Your goal is to perform all the queries and report the array A after all these operations.
Note that the problem is offline, i.e., you can execute the queries in any order.
We assume that k, m ≤ n, just for convenience in expressing the time complexities below. The solution should run in Θ((k + m) log n + n) time.

## Solution

This solution is fundamentally based on the dynamic-prefix-sum technique.
We use an "operation occurrences" (ops_occ) array to memorize the number of times each query needs to be executed, incrementing by one the position relative to the query start and decrementing by one the position relative to the query end. Each position in this array corresponds to an operation, and the value in position i tells us how many times we have to execute the i-th query.

At this point we create another array, the "numbers sum" (num_sum) array, whose length is the same as the input array A, and use the previous array to fill it: for each query <start, end, val> of the ops_occ array, we increase the element at position 'start' of the num_sum array by 'val', and decrease the element at position 'end' of the num_sum array by 'val.

Once we're done iterating on the ops_occ array, we just have to iterate on num_sum array keeping a counter; we add every value we find to the counter (a positive value means that one or more queries are starting, a negative value means that one or more queries are finishing, and they all add up in the counter), and then we increase the corresponding position in the input/result array (the procedure is in-place) by the value of the counter.

## Time and Space complexity

We first scan the queries array (O(m)), then the ops_occ array (O(m)) and then the num_sum array (O(n)), for a total time complexity of O(n+m).
We use 2 supporting arrays, one of size n and one of size m, for an overall space complexity of O(n+m).

## Input

Multiple lines:

- The first line contains n, m, and k
- The next line contains the n integers in A
- Next m lines contain the operations
- - an operation consists of three integers: l, r, and d
- Next k lines contain the queries
- - a query number consists of two integers: a and b

NOTE: in the sample test-cases, the queries are 1-indexed; the main converts them to 0-indexed queries.

## Output

The n integers of the final array A.