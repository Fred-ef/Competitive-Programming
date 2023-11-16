use std::collections::VecDeque;

pub fn find_longest_segment(a: Vec<i32>, k: i32) -> (usize, usize) {
    let max_val = *a.iter().max().unwrap(); // need the maximum to construct occurrences array
    let mut segments: Vec<(usize, usize)> = Vec::new(); // will contain all k-good segments
    let mut result: (usize, usize) = (0, 0); // will contain (one of) the longest k-good segment
    let mut q: VecDeque<i32> = VecDeque::new(); // used to track elements in current sequence
    let mut occ: Vec<usize> = vec![0; (max_val + 1) as usize]; // used to track duplicates in current sequence
    let mut len = 0; // tracks the current sequence length
    let mut curr_k = 0; // tracks the number of distinct elements in the current sequence
    let mut start = 0; // tracks the starting index of the current sequence
    let mut end; // tracks the ending index of the current sequence
    let mut curr_el; // holds the element currently being removed from the sequence

    for i in 0..a.len() {
        while len >= k || curr_k >= k {
            curr_el = q.pop_front().unwrap();
            start += 1;
            len -= 1;
            occ[curr_el as usize] -= 1;
            if occ[curr_el as usize] == 0 {
                curr_k -= 1;
            }
        }

        q.push_back(a[i]);
        len += 1;
        end = i;
        curr_el = a[i];
        occ[curr_el as usize] += 1;
        if occ[curr_el as usize] == 1 {
            curr_k += 1;
        }
        if curr_k == k {
            segments.push((start, end));
        }
    }

    let mut max_length = 0;
    for (s, e) in segments {
        if e - s > max_length {
            result = (s, e);
            max_length = e - s;
        }
    }

    result
}
