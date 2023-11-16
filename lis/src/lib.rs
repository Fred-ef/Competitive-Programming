pub fn lis(arr: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();

    // Scanning the input array
    for &num in arr {
        // If the current element is greater than the last element of our sequence, add it to the sequence
        if res.is_empty() || num >= *res.last().unwrap() {
            res.push(num);
        } else {
            // Else, binary search for the index of the smallest element >= num and substitute it with the current element
            let left = bin_search(&res, num);
            res[left] = num;
        }
    }

    res
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
