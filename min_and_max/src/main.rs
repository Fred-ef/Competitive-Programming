mod segtree;
use segtree::SegTree;

fn main() {
    let arr = vec![10, 6, 3, 1, 4];
    let mut tree = SegTree::from_array(arr);

    let mut res = tree.max(0, 1);

    // println!("Seg tree: {:#?}", tree);
    println!("Result: {}", res);

    tree.update(0, 1, 2);
    
    println!("Seg tree: {:#?}", tree);

    res = tree.max(0, 4);
    println!("Result: {}", res);
}
