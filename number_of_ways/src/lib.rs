pub fn number_of_ways(a: Vec<i64>) -> i64 {
    let mut total = 0;

    // compute the sum of all elements
    for &num in &a {
        total += num;
    }

    // if the total is not divisible by 3, the array cannot be partitioned in 3 portions of equal sum
    if total % 3 != 0 {
        return 0;
    }

    let third_total = total / 3;
    let mut curr_sum = 0;
    let mut first_third = 0; // number of ways to make 1/3 of the total
    let mut result = 0;

    for i in 0..(a.len() - 1) {
        curr_sum += a[i];
        if curr_sum == 2 * third_total {
            // if we're at exactly 2/3 of the total, we have as many new ways of partitioning
            // as there are ways to make 1/3 of the total until now
            result += first_third;
        }
        if curr_sum == third_total {
            // if we're at 1/3 of the total, it means there's a new way to get as much
            first_third += 1;
        }
    }

    result
}
