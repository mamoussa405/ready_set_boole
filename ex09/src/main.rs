use ex09;

fn main() {
    // println!("{:#?}", ex09::eval_set("A!", vec![vec![0, 1, 2]]));
    let res: Vec<i32> = ex09::eval_set(
        "AB&C|D^",
        vec![
            vec![-10, 3, 20, 2, 0, 5],
            vec![2, 0, -10, 13, 60, 5],
            vec![1, 3],
            vec![],
        ],
    );
    println!("{:?}", res);
}
