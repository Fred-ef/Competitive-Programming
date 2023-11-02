# Problem: Minimum Number of Jumps

Given an array of integers where each element represents the maximum number of steps that can be taken forward from that element, your task is to find the minimum number of jumps needed to reach the end of the array starting from the first element. If an element in the array is 0, it means that you cannot move through that element. If it's not possible to reach the end, return -1.

## Problem Statement

You are given an array `arr` of positive integers, where `arr[i]` represents the maximum number of steps that can be taken forward from the element at index `i`. Your goal is to find the minimum number of jumps required to reach the end of the array, starting from the first element.

## Solution

The problem can be solved using a greedy approach, where you continuously track the maximum reachable position and update the number of steps taken.

## Complexity Analysis
In this solution, we simply iterate on the given array S and, for each element S[i], we have at most O(lgn) steps to perform due to the binary search when the current element is smaller than the top of the stack, which gives us a time complexity of O(n*lg(n)).
As for the space, we only use the input array, giving us a O(n) space complexity.

## Usage example

```rust
fn main() {
    let arr = vec![1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9];
    let out = 3;
    let res = min_jumps(&arr);

    println!("The minimum number of jumps to reach the end is {}", res);
}