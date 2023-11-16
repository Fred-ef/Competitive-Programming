use lis::lis;

fn main() {
    // Test 1
    let arr1 = vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    let out1 = 6;
    let res1 = lis(&arr1);
    println!("Test 1: LIS is {:#?}", res1);
    if res1.len() == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    let arr2 = vec![5, 8, 3, 7, 9, 1];
    let out2 = 3;
    let res2 = lis(&arr2);
    println!("Test 2: LIS is {:#?}", res2);
    if res2.len() == out2 {
        println!("Test2 passed!");
    }
}
