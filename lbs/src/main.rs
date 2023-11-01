mod lib;
use lib::lbs;

fn main() {

    // Test 1
    let arr1 = vec![1, 2, 5, 3, 2];
    let out1 = 5;
    let res1 = lbs(&arr1);

    println!("Test 1: Longest Bitonic Subsequence Length: {}", res1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    let arr2 = vec![1, 11, 2, 10, 4, 5, 2, 1];
    let out2 = 6;
    let res2 = lbs(&arr2);

    println!("Test 2: Longest Bitonic Subsequence Length: {}", res2);
    if res2 == out2 {
        println!("Test2 passed!");
    }
}