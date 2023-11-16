pub fn queries_of_operations(
    nums: &mut Vec<i32>,
    ops: Vec<(usize, usize, i32)>,
    queries: Vec<(usize, usize)>,
) {
    let mut ops_occ = vec![0; ops.len() + 1];
    let mut num_sum = vec![0; nums.len() + 1];

    // First of all, we use a new array to memorize, in order, the number of times the i-th certain operation
    // will be executed; this will come in handy later
    for i in 0..queries.len() {
        ops_occ[queries[i].0] += 1;
        ops_occ[queries[i].1 + 1] -= 1;
    }

    // Since we now know how many time each operation will be executed, we can utilize this information
    // to fill an additional array so that the i-th position tells us by how much the i-th element
    // of the "num" array has to be incremented
    // NOTE: since each operation 'op' increases elements by a factor of 'd', we have to add 'd' to the i-th
    // element as many times as 'op' is called on it
    let mut occ_counter = 0;
    for i in 0..(ops_occ.len() - 1) {
        // println!("Current operation: {:#?}", ops[i]);
        occ_counter += ops_occ[i]; // tells us how many times we have to perform this operation
                                   // println!("Adding {} to {}-th element", ops[i].2*occ_counter, ops[i].0);
                                   // println!("Removing {} from {}-th element", ops[i].2*occ_counter, ops[i].1+1);
        num_sum[ops[i].0] += ops[i].2 * occ_counter; // we want to sum "occ_counter" times the 'd' factor from this index on
        num_sum[ops[i].1 + 1] -= ops[i].2 * occ_counter; // we don't want to propagate this operation after the interval end
    }

    // We now know how much has to be added to each position (relative to the previous one), so we
    // can scan the "nums" array and add the correct value to each element
    occ_counter = 0;
    for i in 0..nums.len() {
        occ_counter += num_sum[i];
        nums[i] += occ_counter;
    }
}
