# Min and Max

## Problem description

You are given an array A[1, n] of n positive integers, each integer is at most
n. You have to build a data structure to answer two different types of
queries:
- Update(i, j, T) that replaces every value A[k] with min(A[k], T), where
i ≤ k ≤ j;
- Max(i, j) that returns the largest value in A[i . . . j].
Our target solution runs in O((n+m) log n) time, where m is the number
of queries.

## Solution

We use a segment tree (specifically a "merge-tree") to easily compute max queries on an interval in O(lgn) time (we just need to pre-compute the maximum value for each interval).
To support the update operation without weighing on the time complexity, we use "lazy propagation": when we want to update values in a range, instead of updating all those values one by one, we register the update only in segments that are completely contained in that range; then, when we perform our queries, we push that update down (as low as the query goes), effectively registering it when we touch individual elements in a query. The update itself is very simple: we just register the smaller value between the node's value and the value of the update.
For that reason, the code that actually registers the update on the nodes is part of the "max" query code, and, being part of the query itself, that update is performed in constant time.

## Time and Space complexity
The use of lazy propagation lets us perform both queries and updates in O(lgn) time. The construction of the segment tree takes O(nlgn) time, as we build it with the help of merge-sort so as to contextually compute the max of each range in-construction. We perform a total of m queries, each in O(lgn) time. All this gives us an overall time complexity of O((n+m)lgn).
As for the space, we only need to store the segment tree (specifically an in-array, heap-like, segment tree), which amounts for a space complexity of O(n).

## Input

Multiple lines:
- The first line contains n and m
- The next line contains the n integers in A
- Each of the subsequent m lines contains the query
- - The first value of each line is either 0 (query Update) or 1 (query Max)
- - For a query Update the values of i, j, and T follow
- - For a query Max the values of i and j follows

NOTE: in the sample test-cases, the queries are 1-indexed; the main converts them to 0-indexed queries.

## Output

The results of max queries.