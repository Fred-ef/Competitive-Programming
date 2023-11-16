use std::vec;

pub fn max_sub_array(nums: Vec<i32>) -> Vec<usize> {
    let mut max = 0;
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;

    for (i, &num) in nums.iter().enumerate() {
        if sum > 0 {
            sum += num;
            end = i;
        } else {
            sum = num;
            start = i;
        }

        if sum > max {
            max = sum;
        }
    }

    vec![start, end]
}
