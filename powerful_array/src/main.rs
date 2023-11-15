use powerful_array::compute_power;


fn main() {

    // Test 1
    let arr1 = vec![1, 2, 1];
    let queries1 = vec![(0,1), (0,2)];
    let res1 = compute_power(&arr1, &queries1);
    let out1 = vec![3, 6];

    println!("Results for Test 1: {:#?}", res1);
    if res1 == out1 {
        println!("Test 1 passed!");
    }

    // Test 2
    let arr2 = vec![1, 1, 2, 2, 1, 3, 1, 1];
    let queries2 = vec![(2,7), (1,6), (2, 7)];
    let res2 = compute_power(&arr2, &queries2);
    let out2 = vec![20, 20, 20];

    println!("Results for Test 2: {:#?}", res2);
    if res2 == out2 {
        println!("Test 2 passed!");
    }

    // let arr = vec![2, 6, 10, 4, 7, 28, 9, 11, 6, 33];
    // let queries = vec![
    //     (1, 5),
    //     (2, 8),
    // ];

    // let results = compute_power(&arr, &queries);

    // for (i, result) in results.iter().enumerate() {
    //     println!("Query {}: {}", i, result);
    // }
}
