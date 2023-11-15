use subset_sum::subset_sum;

fn main() {

    // Test 1
    let arr1 = vec![3, 2, 5, 1];
    let target_sum1 = 6;

    if subset_sum(&arr1, target_sum1) {
        println!("Test 1: Subset with sum {} exists.", target_sum1);
        println!("Test 1 passed!");
    } else {
        println!("No subset with sum {} exists.", target_sum1);
        println!("Test 1 failed");
    }

    // Test 2
    let arr2 = vec![3, 34, 4, 12, 5, 2];
    let target_sum2 = 9;

    if subset_sum(&arr2, target_sum2) {
        println!("Test 2: Subset with sum {} exists.", target_sum2);
        println!("Test 2 passed!");
    } else {
        println!("No subset with sum {} exists.", target_sum2);
        println!("Test 2 failed");
    }
}