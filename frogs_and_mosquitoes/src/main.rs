mod lib;
use lib::eat_mosquitoes;

fn main() {

    // test 1
    let frog1 = vec![(10, 2), (15, 0), (6, 1), (0,1)];
    let mosq1 = vec![(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];
    let out1 = vec![(3, 114), (1, 10), (1, 1), (1, 2)];

    let res1 = eat_mosquitoes(frog1, mosq1);
    println!("Result: {:?}", res1);
    if res1 == out1 {
        println!("Test1 passed!");
    }

    // test 2
    let frog2 = vec![(10, 2)];
    let mosq2= vec![(20, 2), (12, 1)];
    let out2 = vec![(1, 3)];

    let res2 = eat_mosquitoes(frog2, mosq2);
    println!("Result: {:?}", res2);
    if res2 == out2 {
        println!("Test2 passed!");
    }
}
