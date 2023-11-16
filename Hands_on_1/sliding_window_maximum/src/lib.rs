use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k: usize = k as usize;
    let n: usize = nums.len();
    let mut result: Vec<i32> = Vec::with_capacity(n);
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(n);
    for i in 0..(n) {
        heap.push((nums[i], i));
        if i < (k - 1) {
            continue;
        }

        while heap.peek().unwrap().1 < i - (k - 1) {
            heap.pop();
        }

        result.push(heap.peek().unwrap().0);
    }

    result
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k: usize = k as usize;
    let n: usize = nums.len();
    let mut result: Vec<i32> = Vec::with_capacity(n);
    let mut tree: BinarySearchTree<(i32, usize)> = BinarySearchTree::new();
    for i in 0..(n) {
        tree.insert((nums[i], i));
        if i < (k - 1) {
            continue;
        }

        while tree.max().unwrap().1 < i - (k - 1) {
            tree.extract_max();
        }

        result.push(tree.max().unwrap().0);
    }

    result
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k: usize = k as usize;
    let n: usize = nums.len();
    let mut result: Vec<i32> = Vec::with_capacity(n);
    let mut deque: VecDeque<(i32, usize)> = VecDeque::with_capacity(n);
    for i in 0..(n) {
        while !deque.is_empty() && deque.back().unwrap().0 <= nums[i] {
            deque.pop_back();
        }

        deque.push_back((nums[i], i));

        if i >= k - 1 {
            while !deque.is_empty() && deque.front().unwrap().1 < i - (k - 1) {
                deque.pop_front();
            }

            result.push(deque.front().unwrap().0);
        }
    }

    result
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
