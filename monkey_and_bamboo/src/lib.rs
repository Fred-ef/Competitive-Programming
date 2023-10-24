

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

        if max_dist == *dist {
            double_max = 1;
        }
        else {
            if *dist > max_dist {
                max_dist = *dist;
                double_max = 0;
            }
        }
    }

    let res = max_dist + double_max;
    res
}