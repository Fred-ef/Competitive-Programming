use min_num_of_jumps::min_jumps;

fn main() {

    // Test 1
    let arr1 = vec![1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9];
    let out1 = 3;
    let res1 = min_jumps(&arr1);

    println!("Test 1: The minimum number of jumps to reach the end is {}", res1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    let arr2 = vec![1, 4, 3, 2, 6, 7];
    let out2 = 2;
    let res2 = min_jumps(&arr2);

    println!("Test 2: The minimum number of jumps to reach the end is {}", res2);
    if res2 == out2 {
        println!("Test2 passed!");
    }
}