use number_of_inversions::merge_sort;

fn main() {

    // test 1
    let mut arr = vec![1, 20, 6, 4, 5];
    let out = 5;
    let res = merge_sort(&mut arr);
    println!("Number of inversions is {}", res);
    if res == out {
        println!("Test1 passed!");
    }
}
