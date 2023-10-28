#[derive(Debug)]
struct Node {
    value: i32,
    interval: (usize, usize),
    seq: Vec<(i32, Option<usize>, Option<usize>)>,
    seq_len: usize
}

// impl Node {
//     pub fn new(isLeaf: bool, value: i32, map: HashMap<i32, usize>) -> Self {
//         Self {isLeaf: isLeaf, value: value, map: map}
//     }
// }

// enum for overlap kinds
enum Overlap {
    TOTAL,
    PARTIAL,
    NONE
}

#[derive(Debug)]
pub struct SegTree {
    tree: Vec<Node>
}

impl SegTree {

    // constructs a segment tree given an input array
    pub fn from_array(arr: Vec<i32>) -> Self {
        let original_len = arr.len();

        // finding the closest power of 2 to pad the array/tree length ( O(lg(n)) time)
        let mut power = 0;
        while usize::pow(2, power) < original_len {
            power +=1;
        }
        let padded_len = usize::pow(2, power);

        // creating vectors for the ordering procedure
        let mut temp = vec![i32::MAX; padded_len];
        let mut clone = vec![i32::MAX; padded_len];
        for i in 0..original_len {
            clone[i] = arr[i];
        }

        // creating the array-based segment tree
        let mut tree = Vec::with_capacity(2*padded_len-1);

        for _ in 0..(2*padded_len-1) {
            let new = Node {
                value: 0,
                interval: (0, 0),
                seq: Vec::new(),
                seq_len: 0,
            };
            tree.push(new);
        }

        // filling the leaves of the segment tree
        for i in 0..original_len {
            tree[padded_len+i-1].value = arr[i];
            tree[padded_len+i-1].interval = (i, i);
            tree[padded_len+i-1].seq.push((arr[i], None, None));
            tree[padded_len+i-1].seq_len = 1;
        }
    
        // call the ordering procedure on the clone (no side effects) to fill the inner nodes of the tree
        Self::divide(&mut tree, &mut clone, &mut temp, 0, padded_len - 1, padded_len, original_len);
    
        Self {tree: tree}
    }
    
    fn divide(tree: &mut Vec<Node>, arr: &mut [i32], temp: &mut Vec<i32>, left: usize, right: usize, leaf_size: usize, original_len: usize) {
        if right > left {
            let mid = (right + left) / 2;
    
            // split sub-arrays
            Self::divide(tree, arr, temp, left, mid, leaf_size, original_len);
            Self::divide(tree, arr, temp, mid + 1, right, leaf_size, original_len);

            // merge and order sub-arrays
            Self::merge(tree, arr, temp, left, mid + 1, right, leaf_size, original_len);
        }
    }
    
    fn merge(tree: &mut Vec<Node>, arr: &mut [i32], temp: &mut Vec<i32>, left: usize, mid: usize, right: usize, leaf_size: usize, original_len: usize) {
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
        let curr_node = (leaf_size+left-1)/usize::pow(2, level_height as u32);

        // we also want to push the interval covered by the node (in the original array), for easy retrieval
        // NOTE: the since the array has been is padded on the right, when examining the right-most slice of
        // the array, we only want to take elements up to the original length of the array (leaving the padding out)
        let right_end = if right > (original_len-1) {original_len-1} else {right};
        tree[curr_node].interval = (left, right_end);

        // ########## DEBUG PRINTS ##########

        // println!("Level height (from the leaves) is: {}", level_height);
        // println!("The parent of nodes from {} to {} is {}", (leaf_size+left-1), (leaf_size+right-1), curr_node);
        // println!("Values between {} and {}:", left, right);
        // print!("[");
        // for i in left..=right {
        //     print!("{} ", arr[i]);
        // }
        // print!("]");
        // println!("");

        // ##################################

        // retrieving the 2 childs:
        let left_child = &tree[curr_node*2+1];
        let right_child = &tree[curr_node*2+2];

        // retrieving the 2 sub arrays:
        let left_seq = &(tree[curr_node*2+1].seq);
        let right_seq = &(tree[curr_node*2+2].seq);
        // println!("Sequences: [{:#?}],\t[{:#?}]", left_seq, right_seq);
        // getting the sequences lengths
        let left_len = left_child.seq_len;
        let right_len = right_child.seq_len;
        // creating 2 indexes to scan the sub-arrays based in each of the child nodes
        let mut j = 0;
        let mut k = 0;

        // allocating an array to fill and set as the curr_node array
        let mut curr_seq: Vec<(i32, Option<usize>, Option<usize>)> = Vec::with_capacity(original_len);
        let mut curr_seq_len: usize = 0;

        // inserting the elements in the node's array and connecting them to the childs arrays
        for i in left..=right_end {
            let mut curr_elem = (arr[i], None, None); // allocating the current element to push into the node's array

            // ##### LINKG TO LEFT CHILD ARRAY #####
            // skipping the values in the child array that are smaller than the current element
            while j < left_len && left_seq[j].0 < arr[i] {
                j += 1;
            }
            // println!("Iter {}, j = {}", i, j);
            // NOTE: this step also ensures that left_seq[j].0 is NEVER bigger than arr[i] (we would reach end of left_seq)
            if j >= left_len {  // the current element is bigger than all values (if any) in the child array
                // point to the end of the child array/-1 if empty
                if left_len == 0 {
                    curr_elem.1 = None;
                }
                else {
                    curr_elem.1 = Some(left_len-1);
                }
            }
            else if left_seq[j].0 == arr[i] {   // values in parent and child perfectly match
                curr_elem.1 = Some(j);    // so we link them
            }
            else  { // not a perfect match, this means the value before was smaller than the curr_elem value
                if j == 0 {
                    curr_elem.1 = None;
                }
                else {
                    curr_elem.1 = Some(j-1);
                }
            }


            // ##### LINKG TO RIGHT CHILD ARRAY #####
            // skipping the values in the child array that are smaller than the current element
            while k < right_len && right_seq[k].0 < arr[i] {
                k += 1;
            }
            // println!("Iter {}, k = {}", i, k);
            // NOTE: this step also ensures that right_seq[k].0 is NEVER bigger than arr[i] (we would reach end of right_seq)
            if k >= right_len {  // the current element is bigger than all values (if any) in the child array
                // point to the end of the child array/-1 if empty
                if right_len == 0 {
                    curr_elem.2 = None;
                }
                else {
                    curr_elem.2 = Some(right_len-1);
                }
            }
            else if right_seq[k].0 == arr[i] {   // values in parent and child perfectly match
                curr_elem.2 = Some(k);    // so we link them
            }
            else  { // not a perfect match, this means the value before was smaller than the curr_elem value
                if k == 0 {
                    curr_elem.2 = None;
                }
                else {
                    curr_elem.2 = Some(k-1);
                }
            }

            curr_seq.push(curr_elem);   // pushing the current element into the current node sequence
            curr_seq_len += 1;  // increasing the length of the current node sequence by 1 (to account for the current element)
        }

        // we can now set the current node array and update the sequence length in the node
        tree[curr_node].seq = curr_seq;
        tree[curr_node].seq_len = curr_seq_len;
        // println!("");
    }

    pub fn smaller_values(&self, left: usize, right: usize, val: i32) -> usize {
        // if the interval exceeds the length of the array, normalize it
        let query_interval = (left, if right > self.tree[0].interval.1 {self.tree[0].interval.1} else {right});
        if right < left {
            return 0;
        }

        // converting the query element "val" to the biggest element which is <= val (this will yield the same result)
        if let Some(index) = Self::tuple_bin_search(&self.tree[0].seq, self.tree[0].seq_len, val) {
            Self::smaller_values_rec(&self, query_interval, Some(index), 0)
        }
        else {
            0
        }
    }

    fn smaller_values_rec(&self, query_interval: (usize, usize), val_index: Option<usize>, curr_node: usize) -> usize {
        let curr_interval = self.tree[curr_node].interval;
        println!("Current node: {}; current interval: {:#?}", curr_node, curr_interval);
        let overlap_type = Self::get_overlap(curr_interval, query_interval);

        match overlap_type {
            Overlap::NONE => {
                // we are not searching for a value in this interval, so we return 0
                println!("No overlap");
                return 0;
            },
            Overlap::TOTAL => {
                // we can stop here as we are looking for occurrences in this exact interval
                // since the sequences are sorted and 0-indexed, we can infer that the item
                // at position i has exactly i elements on the left +1 for the '=' part (<=)
                println!("Total overlap");
                return val_index.unwrap() + 1;
            },
            Overlap::PARTIAL => {
                // we return the sum between the result of the left child and that of the right
                // child, which we can obtain by following the "index-link" in the triple
                println!("Partial overlap");
                let left_res = if self.tree[curr_node].seq[val_index.unwrap()].1 == None {0}
                                        else {self.smaller_values_rec(query_interval, self.tree[curr_node].seq[val_index.unwrap()].1, curr_node*2+1)};
                let right_res = if self.tree[curr_node].seq[val_index.unwrap()].2 == None {0}
                                        else {self.smaller_values_rec(query_interval, self.tree[curr_node].seq[val_index.unwrap()].2, curr_node*2+2)};
                return left_res + right_res;
            },
        }
    }

    fn get_overlap(first: (usize, usize), second: (usize, usize)) -> Overlap {
        println!("First: ({}, {}),\t Second: ({}, {})", first.0, first.1, second.0, second.1);
        if first.0 >= second.0 && first.1 <= second.1 {
            return Overlap::TOTAL;
        }
        else if (first.0 <= second.0 && first.1 >= second.1) ||
                (first.1 >= second.0 && first.1 <= second.1) ||
                (first.0 >= second.0 && first.0 <= second.1) {
            return Overlap::PARTIAL;
        }
        else {
            return Overlap::NONE;
        }
    }

    fn tuple_bin_search(arr: &Vec<(i32, Option<usize>, Option<usize>)>, len: usize, key: i32) -> Option<usize> {
        let mut low = 0;
        let mut high = len-1;

        if high < low {
            return None;
        }
    
        while low <= high {
            let mid = (low + high) / 2;

            if arr[mid].0 == key {
                return Some(mid);
            }
            else if arr[mid].0 < key {
                low = mid + 1;
            }
            else {
                if mid == 0 {
                    break;
                }
                else {
                    high = mid - 1;
                }
            }
        }

        if low == 0 {
            return None;
        }
        else {
            return Some(low-1);
        }
    }
}