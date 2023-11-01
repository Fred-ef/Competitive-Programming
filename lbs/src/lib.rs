use std::vec;

pub fn lbs(arr: &Vec<i32>) -> usize {
    let n = arr.len();

    // Trivial case: no elements in the array, every sequence is 0
    if n == 0 {
        return 0;
    }

    // Arrays to store the Longest Increasing Subsequence (LIS) and
    // Longest Decreasing Subsequence (LDS) for each element.
    let lis_vec = lis(arr);
    let lds_vec = lds(arr);

    // Find the maximum bitonic subsequence length by combining LIS and LDS.
    let mut max_length = 0;
    for i in 0..n {
        let length = lis_vec[i] + lds_vec[i] - 1; // Subtract 1 to avoid double counting.
        if length > max_length {
            max_length = length;
        }
    }

    max_length
}

fn lis(arr: &Vec<i32>) -> Vec<usize> {
    let mut res = Vec::new();
    let mut lis = vec![0; arr.len()];
    let mut curr_lis_len = 0;

    // Scanning the input array
    for (i, &num) in arr.iter().enumerate() {
        // If the current element is greater than the last element of our sequence, add it to the sequence
        if res.is_empty() || num >= *res.last().unwrap() {
            res.push(num);
            curr_lis_len += 1;
            lis[i] = curr_lis_len;
        } else {    // Else, binary search for the index of the smallest element >= num and substitute it with the current element
            let left = bin_search(&res, num);
            res[left] = num;
            lis[i] = left + 1;
        }
    }
    
    lis
}

fn lds(arr: &[i32]) -> Vec<usize> {
    // Reversing the input array to find the LIS from right to left
    let rev_arr: Vec<i32> = arr.iter().cloned().rev().collect();
    let mut res = Vec::new();
    let mut lis = vec![0; arr.len()];
    let mut curr_lis_len = 0;

    for (i, &num) in rev_arr.iter().enumerate() {
        // If the current element is greater than the last element of our sequence, add it to the sequence
        if res.is_empty() || num >= *res.last().unwrap() {
            res.push(num);
            curr_lis_len += 1;
            lis[i] = curr_lis_len;
        } else {    // Else, binary search for the index of the smallest element >= num and substitute it with the current element
            let left = bin_search(&res, num);
            res[left] = num;
            lis[i] = left + 1;
        }
    }

    lis
}

fn bin_search(arr: &Vec<i32>, key: i32) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < key {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}