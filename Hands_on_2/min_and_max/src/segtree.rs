#[derive(Debug)]
struct Node {
    value: i32,
    pending: Option<i32>,
    interval: (usize, usize),
}

// enum for overlap kinds
enum Overlap {
    TOTAL,
    PARTIAL,
    NONE,
}

#[derive(Debug)]
pub struct SegTree {
    tree: Vec<Node>,
}

impl SegTree {
    // constructs a segment tree given an input array
    pub fn from_array(arr: Vec<i32>) -> Self {
        let original_len = arr.len();

        // finding the closest power of 2 to pad the array/tree length ( O(lg(n)) time)
        let mut power = 0;
        while usize::pow(2, power) < original_len {
            power += 1;
        }
        let padded_len = usize::pow(2, power);

        // creating vectors for the ordering procedure
        let mut temp = vec![i32::MAX; padded_len];
        let mut clone = vec![i32::MAX; padded_len];
        for i in 0..original_len {
            clone[i] = arr[i];
        }

        // creating the array-based segment tree
        let mut tree = Vec::with_capacity(2 * padded_len - 1);

        for _ in 0..(2 * padded_len - 1) {
            let new = Node {
                value: 0,
                pending: None,
                interval: (0, 0),
            };
            tree.push(new);
        }

        // filling the leaves of the segment tree
        for i in 0..original_len {
            tree[padded_len + i - 1].value = arr[i];
            tree[padded_len + i - 1].interval = (i, i);
        }

        // call the ordering procedure on the clone (no side effects) to fill the inner nodes of the tree
        Self::divide(
            &mut tree,
            &mut clone,
            &mut temp,
            0,
            padded_len - 1,
            padded_len,
            original_len,
        );

        Self { tree: tree }
    }

    fn divide(
        tree: &mut Vec<Node>,
        arr: &mut [i32],
        temp: &mut Vec<i32>,
        left: usize,
        right: usize,
        leaf_size: usize,
        original_len: usize,
    ) {
        if right > left {
            let mid = (right + left) / 2;

            // split sub-arrays
            Self::divide(tree, arr, temp, left, mid, leaf_size, original_len);
            Self::divide(tree, arr, temp, mid + 1, right, leaf_size, original_len);

            // merge and order sub-arrays
            Self::merge(
                tree,
                arr,
                temp,
                left,
                mid + 1,
                right,
                leaf_size,
                original_len,
            );
        }
    }

    fn merge(
        tree: &mut Vec<Node>,
        arr: &mut [i32],
        temp: &mut Vec<i32>,
        left: usize,
        mid: usize,
        right: usize,
        leaf_size: usize,
        original_len: usize,
    ) {
        let mut i = left;
        let mut j = mid;
        let mut k = left;

        while i <= mid - 1 && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                j += 1;
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

        // getting the number of values in the current sub-array, rounded up to a multiple of 2
        let num_values = right - left + 1;

        // finding the right node in which to push the values; since we have to climb bottom-up
        // towards the root in a binary tree, we need to divide by the logarithm base 2 of the
        // current elements to reach the correct level
        let level_height = num_values.ilog2() as usize;
        let curr_node_index = (leaf_size + left - 1) / usize::pow(2, level_height as u32);

        // we also want to push the interval covered by the node (in the original array), for easy retrieval
        // NOTE: the since the array has been is padded on the right, when examining the right-most slice of
        // the array, we only want to take elements up to the original length of the array (leaving the padding out)
        let curr_seq_end = if right > (original_len - 1) {
            original_len - 1
        } else {
            right
        };
        tree[curr_node_index].interval = (
            left,
            if curr_seq_end > left {
                curr_seq_end
            } else {
                right
            },
        );

        // setting the max element as the node's value
        tree[curr_node_index].value = arr[curr_seq_end];
    }

    pub fn max(&mut self, query: (usize, usize)) -> i32 {
        // dividing input variables
        let left = query.0;
        let right = query.1;
        // if the interval exceeds the length of the array, normalize it
        let query_interval = (
            left,
            if right > self.tree[0].interval.1 {
                self.tree[0].interval.1
            } else {
                right
            },
        );
        if right < left {
            return i32::MIN;
        }

        self.max_rec(query_interval, 0)
    }

    fn max_rec(&mut self, query_interval: (usize, usize), curr_node_index: usize) -> i32 {
        let curr_interval = self.tree[curr_node_index].interval;
        if self.tree[curr_node_index].pending.is_some() {
            // registering any pending update
            let updated_val = self.tree[curr_node_index].pending.unwrap();
            self.tree[curr_node_index].value = if self.tree[curr_node_index].value < updated_val {
                self.tree[curr_node_index].value
            } else {
                updated_val
            };
            if !(curr_interval.0 == curr_interval.1) {
                // if we're not at a leaf, propagate the update to the 2 children
                self.tree[curr_node_index * 2 + 1].pending = self.tree[curr_node_index].pending;
                self.tree[curr_node_index * 2 + 2].pending = self.tree[curr_node_index].pending;
            }
            self.tree[curr_node_index].pending = None; // the node has now been updated
        }

        let overlap_type = Self::get_overlap(curr_interval, query_interval);
        match overlap_type {
            Overlap::NONE => {
                // we are not searching for a value in this interval, so we return 0
                // println!("No overlap");
                return i32::MIN;
            }
            Overlap::TOTAL => {
                // we return the node value, which has already been updated with the min between its value
                // and the updated value
                return self.tree[curr_node_index].value;
            }
            Overlap::PARTIAL => {
                // we return the max between the values returned by the 2 children
                // println!("Partial overlap");
                let left_res = self.max_rec(query_interval, 2 * curr_node_index + 1);
                let right_res = self.max_rec(query_interval, 2 * curr_node_index + 2);
                return if left_res > right_res {
                    left_res
                } else {
                    right_res
                };
            }
        }
    }

    pub fn update(&mut self, query: (usize, usize, i32)) {
        // divinding input variables
        let left = query.0;
        let right = query.1;
        let val = query.2;
        // if the interval exceeds the length of the array, normalize it
        let query_interval = (
            left,
            if right > self.tree[0].interval.1 {
                self.tree[0].interval.1
            } else {
                right
            },
        );
        if right < left {
            return;
        }

        self.update_rec(query_interval, val, 0);
    }

    fn update_rec(
        &mut self,
        query_interval: (usize, usize),
        val: i32,
        curr_node_index: usize,
    ) -> i32 {
        let curr_interval = self.tree[curr_node_index].interval;
        let overlap_type = Self::get_overlap(curr_interval, query_interval);

        match overlap_type {
            Overlap::NONE => {
                // we have nothing to update here, so we just return the node's value
                // println!("No overlap");
                return self.tree[curr_node_index].value;
            }
            Overlap::TOTAL => {
                // we return the min between the node's value and lazily-propagated update
                // println!("Total overlap");
                self.tree[curr_node_index].pending = Some(val);
                // returning the minimum between the value and the updated value to update the max in the ancestors
                if self.tree[curr_node_index].value < val {
                    self.tree[curr_node_index].value
                } else {
                    val
                }
            }
            Overlap::PARTIAL => {
                // we return the max between the values returned by the 2 children
                // println!("Partial overlap");
                let res_left = self.update_rec(query_interval, val, 2 * curr_node_index + 1);
                let res_right = self.update_rec(query_interval, val, 2 * curr_node_index + 2);
                let new_max = if res_left > res_right {
                    res_left
                } else {
                    res_right
                };
                self.tree[curr_node_index].value = new_max;
                new_max
            }
        }
    }

    fn get_overlap(first: (usize, usize), second: (usize, usize)) -> Overlap {
        if first.0 >= second.0 && first.1 <= second.1 {
            return Overlap::TOTAL;
        } else if (first.0 <= second.0 && first.1 >= second.1)
            || (first.1 >= second.0 && first.1 <= second.1)
            || (first.0 >= second.0 && first.0 <= second.1)
        {
            return Overlap::PARTIAL;
        } else {
            return Overlap::NONE;
        }
    }
}
