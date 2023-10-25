# Problem

You are given an array `a` consisting of `n` integers. Your goal is to determine in how many ways you can split the elements of the array into three contiguous parts (of non-zero length) such that the sum of elements in each part is the same.

## Approach

To solve this problem, we can use the following approach:

1. Calculate the total sum of all elements in the array `a`. If the total sum is not divisible by 3, return 0 since it's impossible to split the array into three parts with equal sums.

2. Initialize variables `curr_sum` to keep track of the current sum of elements encountered, `first_third` to count the number of times the current sum is one-third of the total sum, and `result` to store the final result.

3. Iterate through the array `a`, summing up the elements and checking two conditions:
   - If `curr_sum` becomes two-thirds of the total sum, increment the `result` by the current value of `first_third`, since there is one new way to split the array for each way there was to reach one-third of the sum.
   - If `curr_sum` becomes one-third of the total sum, increment `first_third`.

4. Return the final value of `result`, which represents the number of ways to split the array into three parts with equal sums.

## Example

Here's an example of how to use the `number_of_ways` function in Rust:

```rust
let arr = vec![1, 2, 3, 0, 3];
let out = 2;
let res = number_of_ways(arr);
println!("Number of ways: {}", res);
if res == out {
    println!("Test passed!");
}