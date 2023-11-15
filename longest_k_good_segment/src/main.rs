use longest_k_good_segment::find_longest_segment;

fn main() {
    // Test 1
    let arr1 = vec![1, 2, 3, 4, 5];
    let k1 = 5;
    let out1 = (0, 4);
    let res1 = find_longest_segment(arr1, k1);
    println!("Test 1: longest k-good segment: [{}, {}]", res1.0, res1.1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    let arr2 = vec![6, 5, 1, 2, 3, 2, 1, 4, 5];
    let k2 = 3;
    let out2 = (0, 2);
    let res2 = find_longest_segment(arr2, k2);
    println!("Test 2: longest k-good segment: [{}, {}]", res2.0, res2.1);
    if res2 == out2 {
        println!("Test2 passed!");
    }

    // Test 3
    let arr3 = vec![1, 2, 3];
    let k3 = 1;
    let out3 = (0, 0);
    let res3 = find_longest_segment(arr3, k3);
    println!("Test 3: longest k-good segment: [{}, {}]", res3.0, res3.1);
    if res3 == out3 {
        println!("Test3 passed!");
    }
}