pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![-1; n];
    let mut stack = Vec::new();

    for i in 0..(2 * n) {
        let index = i % n; // compute the right index in the circular array

        while let Some(&top) = stack.last() {
            if nums[index] > nums[top] {
                res[top] = nums[index];
                stack.pop();
            } else {
                break;
            }
        }

        if i < n {
            stack.push(index);
        }
    }

    res
}