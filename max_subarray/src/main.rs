use std::vec;

use max_subarray::max_sub_array;

fn main() {
    let arr1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let arr2 = vec![1];
    let arr3 = vec![5, 4, -1, 7, 8];

    let out1 = [3, 8];
    let out2 = [0, 0];
    let out3 = [0, 4];

    // test 1
    let res1 = max_sub_array(arr1);
    println!("The maximum subarray (interval) in the first array is {:#?}", out1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // test 2
    let res2 = max_sub_array(arr2);
    println!("The maximum subarray (interval) in the second array is {:#?}\n", out2);
    if res2 == out2 {
        println!("Test2 passed!");
    }

    // test 3
    let res3 = max_sub_array(arr3);
    println!("The maximum subarray (interval) in the third array is {:#?}", out3);
    if res3 == out3 {
        println!("test3 passed!");
    }
}
