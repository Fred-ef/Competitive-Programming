# Problem: Minimum Number of Jumps

## Problem description

Given an array of integers where each element represents the maximum number of steps that can be taken forward from that element, your task is to find the minimum number of jumps needed to reach the end of the array starting from the first element. If an element in the array is 0, it means that you cannot move through that element. If it's not possible to reach the end, return -1.

## Solution

The problem can be solved using a greedy approach, where you continuously track the maximum reachable position and update the number of steps taken.
At each step we check if the goal is reachable/reached, in which case we're done; else, we check if it's time to jump (if we have no remaining steps but have not arrived yet at the maximum reachable position). If we're out of steps AND we're at the maximum reachable position, we've landed on a 0 (game-over).

## Time and Space complexity

We scan the input array only once, performing constant-time operations at each step, for a time complexity of O(n).
We only use a few variables in the algoritmh, thus the space complexity is O(1).

## Input

The vector that gives us the jump values (already provided in the main method).

## Output

The minimum number of jumps to reach the end of the array, along with some test results.