## Problem
Find the Next Greater Number in a Circular Array. The next greater number of an element `x` is the first greater number in its traversing-order next in the array. If it does not exist, the value is -1.

## Solution

The proposed solution is based on two scans of the `nums` array. We use a `res` vector to store the next greater numbers and initialize all elements to -1. A `stack` vector is used to keep track of the indices of elements in the array.

The resolution process works as follows:

1. We scan the `nums` array twice to ensure we handle circular numbers correctly.

2. We calculate the index in the circular array using `i % n`, where `n` is the length of the `nums` array.

3. We iterate through the `stack` vector. If the current element `nums[index]` is greater than the element at the top of the stack, we pop the element at the top of the stack and assign the current value `nums[index]` to `res`. Otherwise, we continue popping until we satisfy this condition.

4. If the scan is in the first iteration (`i < n`), we push the current index `index` into the stack.

5. In the end, the `res` vector will contain the next greater numbers for each element in `nums`.

## Usage Example

```rust
use project_name::next_greater_elements;

fn main() {
    let nums = vec![1, 2, 1];
    let result = next_greater_elements(nums);
    println!("{:?}", result); // Output: [2, -1, 2]
}
