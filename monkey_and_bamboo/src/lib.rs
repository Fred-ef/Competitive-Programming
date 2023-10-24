

pub fn compute_strength(
    rungs: Vec<i32>
) -> i32 {
    let mut last_height = 0;
    let mut distances = Vec::new();

    for curr_height in rungs {
        distances.push(curr_height-last_height);
        last_height = curr_height;
    }

    let mut max_dist = distances[0];
    let mut double_max = 0;

    for (i, dist) in distances.iter().enumerate() {
        if i == 0 {
            continue;
        }

        println!("curr dist: {}", dist);    // DEBUG
        println!("max dist: {}", max_dist); // DEBUG
        println!("carry: {}", double_max);  // DEBUG
        if max_dist == *dist {
            println!("Double!");    // DEBUG
            double_max = 1;
        }
        else {
            if *dist > max_dist {
                println!("Update"); // DEBUG
                max_dist = *dist;
                double_max = 0;
            }
        }
        println!("##########");             // DEBUG
    }

    let res = max_dist + double_max;
    res
}