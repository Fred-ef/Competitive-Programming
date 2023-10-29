mod segtree;
use segtree::SegTree;

fn main() {
    let arr = vec![5, 1, 4, 3, 2];
    let mut tree = SegTree::from_array(arr);
    let out1 = 4;
    let out2 = 2;

    tree.update(0, 1, 2);

    // Test 1
    let mut res = tree.max(1, 3);
    println!("First result: {}", res);
    if res == out1 {
        println!("Test1 passed!");
    }

    // Test 2
    res = tree.max(0, 1);
    println!("Second result: {}", res);
    if res == out2 {
        println!("Test2 passed!");
    }
}
