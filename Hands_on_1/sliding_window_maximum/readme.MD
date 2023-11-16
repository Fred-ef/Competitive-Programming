# Sliding window maximum

# Problem description

You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.

## HEAP IMPLEMENTATION

In the HEAP implementation, we cycle on each of the input array elements (except for those
already included in the last possible window, at the end), inserting the current element in the
heap at each iteration (shifting the window by one position); at that point, we iteratively get the
maximum heap element, removing it is outside the current window.
It is now sufficient to return the maximum heap element.
Since each of the n input elements is inserted/removed at most one time, and since each
insertion/deletion takes O(lgn) complexity, the final complexity of this approach amounts to
O(nlgn).

## BST IMPLEMENTATION

In the BST implementation, the procedure is substantially the same as the HEAP one, though
using an inherently more efficient data structure.
Here too, each of the n input elements is inserted/removed at most once, but, working with
windows of k elements, the tree’s height is limited by the k parameter for a balanced BST,
dropping each operation’s cost down to O(lgk) (which is a substantial difference when
O(k) << O(n)).
For those reasons, the final complexity amounts to O(nlgk).

## DEQUE IMPLEMENTATION

In the DEQUE implementation, at each turn we first remove all the elements that are now
outside the window (starting from the head), then all those elements whose value is smaller
than that of the current elements (starting from the tail); at that point, we can insert the current
element in the tail. We now just have to return the deque’s head.
This approach requires the insertion/deletion of each input element exactly once, but with a
fundamental improvement: even though the cost of each insertion amounts to O(n) (worst case)
because of the removal of the head/tail elements, each of those removals can be operated in
constant time O(1).
For these reasons, the approach results in a linear final complexity (O(n)).

## TEST RESULTS COMPARISON

Looking at the various tests’ results, we can make a few observations:
- With few input elements (1024/2048), we see that the BST has the worst
performance: its execution time slowly decreases as the window’s dimension k
increases, showing that the bad performances are probably due to allocation/pointer
management/unbalance-ness.
Another poor performance is that of the brute-force methods, with the “idiomatic”
version performing a little better as k increases, showing an advantage for its better
resources-management.
The best performances are obtained by the deque and heap versions, with the former
doing a bit better than the latter, and both not showing any downfall as k increases.
- Increasing the input elements to 4096/8192, we start to notice that, while remaining a
lot slower for small values of k, the BST versions gets better as k increases, becoming
much more efficient than the brute-force methods, showing the advantage of the
BST-data-structure over an unstructured method.
The situation is basically unchanged for the deque and the heap, which remain at a
very small distance from one another (with deque always leading).
- Some changes going from 16384 input elements upwards: BST still is the absolute
worst for small values of k, while still being more efficient that brute-force for higher k
values.
Heap and deque still hava very similar performances, with deque always leading, for
almost all values of k, except for the smallest values: deque performs a lot better in
that situation, thanks to the stability of its linear execution time, while the heap version,
which in addition has a logarithmic factor, follows a logarithmic evolution, growing lot
initially, but quickly stabilizing.
Those results get a little better in absolute value enabling the native optimazion, while the
relation between them remains basically unchanged.