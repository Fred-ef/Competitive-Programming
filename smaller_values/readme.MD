# Smaller values

## Problem description

You are given an array A[0..n − 1] of positive integers smaller than n. Your goal is to solve m given count queries. A query count(i, j, X) has to report the number of values in A[i..j] which are smaller than or equal to X (notice that both positions i and j are included).

## Solution

The approach chosen for this solution uses segment tree, so as to have an appropriate structure to execute each query in O(lgn) time.
The tree is a "merge tree", built in a heap-like fashion using the merge-sort algorithm: this way, in in each node of the tree we can easily store an ordered (by construction) array of triples <val, sx, dx>:
- the array is ordered by 'val'; this way, if the element with val=X is stored in A[2] (0-indexed), it means that there are 2 elements smaller than itself
- the 'sx' field will connect the value to the array of the node's left-child
- the 'dx' field will connect the value to the array of the node's right child

What we mean by "connecting a value to a child-node's array" is that we use a variant of the fractional cascading technique: we link each element in a node's array to the same element in the arrays of the left and right childs of that node, so that we can propagate the query down in the right direction in just one step. If the left and/or right child's array doesn't contain the item, then we just set its "linking index" to None.

For the query, it's pretty simple:
- if we have no overlap, we don't care about the element and return 0
- if we have a full overlap, we return the position of the element +1 in the node's array (since it's ordered, the position of an element tells us exactly how many elements in the array are smaller than itself, and then we increase by one because we actually want "smaller or equal")
- if we have a partial overlap, we propagate the query to the children by using the "index link" (or return 0 if we have no link, i.e. the element is not contained in the child array).

There only remains one detail to consider: what do we do if the key of a query is not in the array? How can we link it to the children sequences?
To handle this situation, we just perform a binary search on the tree root and "approximate" the input key with the smallest item in the tree that is >= than the key itself.

## Time and Space complexity

This solution allows us to perform each query in O(lgn) time, so solving the m queries requires O(mlgn) time. The construction of the merge-tree is dominated by the merge-sort procedure, and thus takes O(nlgn) time. The resulting overall time complexity is of O((n+m)lgn) time.
As for the space, we only store the segment tree, but in each node there's an array instead of a single value. Nonetheless, each item of the tree is only stored in the sequences that can be found on the root-leaf path to that item, therefore any item is stored O(lgn) times, which gives us an overall space complexity of O(nlgn).

## Input

Multiple lines:
- The first line contains n and m
- The next line consists in n integers, separated by a space
- Finally, there will be m lines, one for each query
- - Each of these lines contains i, j and X, separated by a space

## Output

The result of each query in input order.