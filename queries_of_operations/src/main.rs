mod lib;
use lib::queries_of_operations;

fn main() {

    // Test 1
    let mut nums = [1, 2, 3].to_vec();
    let res = [9, 18, 17];
    // println!("Array before queries: {:#?}", nums);
    let ops = [(0, 1, 1), (0, 2, 2), (1, 2, 4)].to_vec();
    let queries = [(0, 1), (0, 2), (1, 2)].to_vec();

    queries_of_operations(&mut nums, ops, queries);

    // println!("Array after queries: {:#?}", nums);
    if nums == res {
        println!("Test1 passed!");
    }
}
