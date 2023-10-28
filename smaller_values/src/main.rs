mod lib;
mod segtree;

use segtree::SegTree;

fn main() {
    let arr = vec![10, 6, 3, 1, 4];
    let tree = SegTree::from_array(arr);
    let val = 4;

    let res = tree.smaller_values(2, 4, val);

    // println!("Seg tree: {:#?}", tree);
    println!("Values smaller or equal than {}: {}", val, res);
}
