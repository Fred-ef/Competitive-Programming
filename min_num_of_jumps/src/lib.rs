use std::cmp::max;

pub fn min_jumps(arr: &Vec<i32>) -> i32 {
    let n = arr.len();

    // If the array is empty, no jumps to perform
    if n <= 1 {
        return 0;
    }

    // If we can jump to the end from the first element, return immediately with 1 jump
    if arr[0] >= (n - 1) as i32 {
        return 1;
    }

    // If the first cell is 0, we can't even start jumping; return error value
    if arr[0] == 0 {
        return -1;
    }

    // Initializing support variables
    let mut reachable = arr[0] as usize; // Initialize with position reachable from first cell
    let mut rem_steps = arr[0] as usize; // Initialize with the remaining number of steps from the first cell
    let mut num_jumps = 1; // We already "started" the first jump, and have to see where we can land

    for i in 1..n {
        // If we're at the goal, return jumps done until now
        if i == n - 1 {
            return num_jumps;
        }

        // If the goal is reachable, return jumps done until now plus 1 jump to reach the end
        if arr[i] >= (n - 1 - i) as i32 {
            return num_jumps + 1;
        }

        // We may already be able to go farther (thanks to a previous jump) than if we jumped from the current position
        reachable = max(reachable, i + arr[i] as usize); // So we take the best value
        rem_steps -= 1; // We have done 1 iteration, corresponding to one step

        if rem_steps == 0 {
            // If we cannot do any more step and are in the reachable position, it means we had to land on a 0; return error
            if i >= reachable {
                return -1;
            }

            num_jumps += 1; // If we cannot do any more step but haven't gotten to our reachable distance, it means that we have to jump again...
            rem_steps = reachable - i; // ... updating the number of steps we can make to those needed to reach current goal distance
        }
    }

    // We haven't been able to reach the end, return error
    -1
}
