use next_greater_element::next_greater_elements;

fn main() {
    let arr1 = vec![1, 2, 1];
    let arr2 = vec![1, 2, 3, 4, 3];

    let out1 = vec![2, -1, 2];
    let out2 = vec![2, 3, 4, -1, 4];

    // test 1
    let res1 = next_greater_elements(arr1);
    println!("Result: {:#?}", res1);
    if res1 == out1 {
        println!("test1 passed");
    }

    // test 2
    let res2 = next_greater_elements(arr2);
    println!("Result: {:#?}", res2);
    if res2 == out2 {
        println!("test2 passed");
    }
}
