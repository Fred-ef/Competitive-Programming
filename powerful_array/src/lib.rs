pub fn compute_power(arr: &Vec<i32>, queries: &Vec<(usize, usize)>) -> Vec<i32> {
    let n = arr.len();
    let block_size: usize = (n as f64).sqrt() as usize;

    // getting the maximum element in arr to set the occurrences array dimension
    let mut max_val = arr[0];
    for i in 1..n {
        if arr[i] > max_val {
            max_val = arr[i];
        }
    }

    // Create a vector to store the results of the queries
    let mut res = vec![0; queries.len()];

    // Sorting queries of different by block and queries in the same block by end-index
    let mut queries = queries
        .iter()
        .enumerate()
        .map(|(i, q)| (i, q))
        .collect::<Vec<_>>();
    queries.sort_by(|(_, q1), (_, q2)| {
        let block1 = q1.0 / block_size;
        let block2 = q2.0 / block_size;
        if block1 != block2 {
            block1.cmp(&block2)
        } else {
            q1.1.cmp(&q2.1)
        }
    });

    // Main procedure
    let mut prev_left = 0;
    let mut prev_right = 0;
    let mut occ = vec![0; max_val as usize + 1];
    let mut curr_power = 0;

    // Since both arrays (numbers and queries) are 0-indexed, we need to manually add the power of the first
    // subarray (0, 0)
    occ[arr[0] as usize] += 1;
    curr_power += occ[arr[0] as usize] * occ[arr[0] as usize] * arr[0];

    // Scanning each query, ordered by our criteria
    for (i, query) in queries {
        let curr_left = query.0;
        let curr_right = query.1;

        // last query started after curr query start; we need to add the elements in between
        while prev_left > curr_left {
            prev_left -= 1;
            // Adding the element to the left of the current subarray
            let added_element = arr[prev_left];
            // Removing the old power from the sum (we will put back the updated version)
            curr_power -= occ[added_element as usize] * occ[added_element as usize] * added_element;
            occ[added_element as usize] += 1; // adding the new occurrence
                                              // Adding the updated power to the sum
            curr_power += occ[added_element as usize] * occ[added_element as usize] * added_element;
        }

        // last query started before curr query start; we need to remove the elements in between
        while prev_left < curr_left {
            // Removing the element from the left of the current subarray
            let removed_element = arr[prev_left];
            // Removing the old power from the sum (we will put back the updated version)
            curr_power -=
                occ[removed_element as usize] * occ[removed_element as usize] * removed_element;
            occ[removed_element as usize] -= 1;
            // Adding the updated power to the sum
            curr_power +=
                occ[removed_element as usize] * occ[removed_element as usize] * removed_element;

            prev_left += 1;
        }

        // last query ended before curr query end; we need to add the elements in between
        while prev_right < curr_right {
            prev_right += 1;
            // Adding the element to the right of the current subarray
            let added_element = arr[prev_right];
            // Removing the old power from the sum (we will put back the updated version)
            curr_power -= occ[added_element as usize] * occ[added_element as usize] * added_element;
            occ[added_element as usize] += 1;
            // Adding the updated power to the sum
            curr_power += occ[added_element as usize] * occ[added_element as usize] * added_element;
        }

        // last query ended after curr query end; we need to remove the elements in between
        while prev_right > curr_right {
            // Remove the element from the right of the current subarray
            let removed_element = arr[prev_right];
            // Removing the old power from the sum (we will put back the updated version)
            curr_power -=
                occ[removed_element as usize] * occ[removed_element as usize] * removed_element;
            occ[removed_element as usize] -= 1;
            // Adding the updated power to the sum
            curr_power +=
                occ[removed_element as usize] * occ[removed_element as usize] * removed_element;

            prev_right -= 1;
        }

        res[i] = curr_power;
    }

    res
}
