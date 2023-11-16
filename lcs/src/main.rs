use lcs::lcs;

fn main() {
    // Test 1
    let s1_1 = "ABCDGH";
    let s2_1 = "AEDFHR";
    let out1 = 3;

    let res1 = lcs(s1_1, s2_1);
    println!("Test 1: Length of Longest Common Subsequence: {}", res1);
    if res1 == out1 {
        println!("Test 1 passed!");
    }

    // Test 2
    let s1_2 = "ABC";
    let s2_2 = "AC";
    let out2 = 2;

    let res2 = lcs(s1_2, s2_2);
    println!("Test 2: Length of Longest Common Subsequence: {}", res2);
    if res2 == out2 {
        println!("Test 2 passed!");
    }
}
