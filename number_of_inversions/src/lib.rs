pub fn merge_sort(arr: &mut [i32]) -> u64 {
    let len = arr.len();
    let mut temp = vec![0; len];
    let inv_count = merge_sort_rec(arr, &mut temp, 0, len - 1);
    inv_count
}

pub fn merge_sort_rec(arr: &mut [i32], temp: &mut Vec<i32>, left: usize, right: usize) -> u64 {
    let mut inv_count: u64 = 0;
    if right > left {
        let mid = (right + left) / 2;

        // Recursively sort and count inversions in the left and right subarrays
        inv_count += merge_sort_rec(arr, temp, left, mid);
        inv_count += merge_sort_rec(arr, temp, mid + 1, right);

        // Merge the two sorted subarrays and count inversions
        inv_count += merge(arr, temp, left, mid + 1, right);
    }
    inv_count
}

fn merge(arr: &mut [i32], temp: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> u64 {
    let mut i = left;
    let mut j = mid;
    let mut k = left;
    let mut inv_count: u64 = 0;

    while i <= mid - 1 && j <= right {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
            inv_count += (mid - i) as u64;
        }
        k += 1;
    }

    while i <= mid - 1 {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j <= right {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    for i in left..=right {
        arr[i] = temp[i];
    }

    inv_count
}
