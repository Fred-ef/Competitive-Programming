## Problem
Given an array of integers, you need to find the number of inversions in the array. An inversion is a pair of indices (i, j) such that i < j and arr[i] > arr[j]. In simpler terms, it represents the number of times a smaller element appears after a larger element in the array.
For example:

Input: [1, 20, 6, 4, 5]
Output: 5

Explanation: In this example, there are five inversions: (20, 6), (20, 4), (20, 5), (6, 4), and (6, 5).

## Solution
The solution to this problem is to use the Merge Sort algorithm, which inherently counts the number of inversions during the sorting process. Merge Sort is a divide-and-conquer sorting algorithm that divides the input array into two halves, recursively sorts them, and then merges the sorted halves while counting the inversions.
In the merge step, when an element from the right subarray is smaller than an element from the left subarray, it indicates an inversion. The count of inversions also has to be incremented by the number of elements remaining in the left subarray.