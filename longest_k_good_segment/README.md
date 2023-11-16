# Longest k-good segment

## Problem description
You are given an array a with n integers. A sequence of one or more consecutive elements in the array is called a segment. A segment is considered k-good if it contains no more than k different values. Your task is to find the longest k-good segment in the given array.

Implement the find_longest_k_good_segment function to find the longest k-good segment in the array.

## Solution
The problem can be solved by using a sliding window approach. We maintain a queue to keep track of the current segment, a variable curr_k to count the number of different elements in the current segment, and two pointers start and end to represent the current segment.
We also use an array occ to store the occurrences of elements in the current segment. The maximum value in the input array a is determined to allocate the size of the occ array.
We start by scanning the array from left to right. For each element a[i], we update the sliding window and check if it forms a k-good segment. If the segment is k-good, we record it.
Finally, we iterate through the recorded segments to find the longest one and return its start and end indices.

NOTE: the algorithm finds the FIRST longest k-good segments, as there can be more than one.

## Time and Space complexity

This algorithm has a time complexity of O(n), where n is the length of the input array a, as we scan the array once.
The 'segment' vector is the auxiliary structure that requires the most space, as it can contain more than n elements; it cannot however contain more than O(n^2) elements, so the space complexity is O(n^2) amortized.

## Input

k and the array we look for the longest k-good segment in (already provided in the main method).

## Output

The first longest k-good segment found, along with some test results.