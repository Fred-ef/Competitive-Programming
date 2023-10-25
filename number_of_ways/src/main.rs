mod lib;
use lib::number_of_ways;

fn main() {

    // Test 1
    let arr1 = vec![1, 2, 3, 0, 3];
    let out1 = 2;
    let res1 = number_of_ways(arr1);
    println!("Number of ways for first vector: {}", res1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    let arr2 = vec![0, 1, -1, 0];
    let out2 = 1;
    let res2 = number_of_ways(arr2);
    println!("Number of ways for second vector: {}", res2);
    if res2 == out2 {
        println!("Test2 passed!");
    }

    // Test 3
    let arr3 = vec![4, 1];
    let out3 = 0;
    let res3 = number_of_ways(arr3);
    println!("Number of ways for third vector: {}", res3);
    if res3 == out3 {
        println!("Test3 passed!");
    }

    // Test 4
    let arr4 = vec![0, 1, -1, 0, 1, -1, 0];
    let out4 = 6;
    let res4 = number_of_ways(arr4);
    println!("Number of ways for fourth vector: {}", res4);
    if res4 == out4 {
        println!("Test4 passed!");
    }
}