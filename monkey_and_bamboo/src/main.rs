use monkey_and_bamboo::compute_strength;

fn main() {

    // test 1
    let rungs1 = vec![1, 6, 7, 11, 13];
    let out1 = 5;
    let res1 = compute_strength(rungs1);
    println!("Test 1: computed strength: {}", res1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // test 2
    let rungs2 = vec![3, 9, 10, 14];
    let out2 = 6;
    let res2 = compute_strength(rungs2);
    println!("Test 2: computed strength: {}", res2);
    if res2 == out2 {
        println!("Test2 passed!");
    }

    // test 3
    let rungs3 = vec![1, 5, 10, 15, 20, 23, 25, 26];
    let out3 = 6;
    let res3 = compute_strength(rungs3);
    println!("Test 3: computed strength: {}", res3);
    if res3 == out3 {
        println!("Test3 passed!");
    }
}
