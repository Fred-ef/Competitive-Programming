# The monkey and the oiled bamboo

## Problem description
I have a ladder and I have to climb it from the ground up.
In In each jump I can jump from the current rung (or the ground) to the next rung only (can’t skip rungs). Initially I set my strength factor k. The meaning of k is, in any jump I can’t jump more than k feet. If I jump exactly k feet in a jump, k is decremented by 1, but if I jump less than k feet, k remains the same.
Given the heights of the rungs of the ladder from the ground, we want to find the
minimum strength factor k, such that we can reach the top rung.


### Solution

The solution is simple:

1. Calculate the relative distances between rungs from the given heights.
2. Find the maximum relative distance, which represents the longest gap you need to jump.
3. If the maximum distance occurs only once, the minimum strength factor 'k' is equal to this maximum distance.
4. If the maximum distance occurs multiple times, the minimum strength factor 'k' is the maximum distance plus 1 (to account for the fact that jumping the maximum gap for the first time would reduce 'k' and we wouldn't be able to jump the same gap a second time).

## Time and Space complexity

We have a step of pre-processing in which we compute the vector of relative distances scanning the input array in O(n); then we scan the relative-distances vector only once in the main computation, for an overall time complexity of O(n).
We use the vector of relative-distances as the auxiliary DS, for an overall space complexity of O(n).

## Input

The rung heights vector (already provided in the main method).

## Output

The minimum strength needed to reach the end of the ladder, along with some test results.