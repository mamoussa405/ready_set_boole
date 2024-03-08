mod ast;

use ast::AST;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: AST = AST::new();

    tree.build(formula, true);
    tree.simplify_material_properties();
    tree.eval(sets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        let mut res: Vec<i32> = eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]);
        res.sort();
        assert_eq!(vec![0], res);

        res = eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]);
        res.sort();
        assert_eq!(vec![0, 1, 2, 3, 4, 5], res);

        res = eval_set("A!", vec![vec![0, 1, 2]]);
        assert_eq!(vec![] as Vec<i32>, res);
    }

    #[test]
    fn sets_intersection_tests() {
        let mut res: Vec<i32> = eval_set(
            "EF&G&",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -11, 13, 60, 5],
                vec![-11, 3, 100, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);

        res = eval_set(
            "EF&G&",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 5], res);

        res = eval_set(
            "FG!&E&",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![0, 2], res);

        res = eval_set(
            "FG!&E&!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 3, 5, 13, 20, 60, 100, 200, 300], res);

        res = eval_set(
            "FG&",
            vec![
                vec![],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);

        res = eval_set(
            "F!G&",
            vec![
                vec![],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 3, 5, 60, 100, 200, 300], res);

        res = eval_set(
            "FF!&",
            vec![
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);
        
        res = eval_set(
            "F!F!&",
            vec![
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);

        res = eval_set(
            "KM&LX&&P&",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![2, 100, 60, 5, -10, 13],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 5], res);

        res = eval_set(
            "KM&LX&&P&!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![2, 100, 60, 5, -10, 13],
            ],
        );
        res.sort();
        assert_eq!(vec![0, 2, 3, 13, 20, 60, 100, 200, 300], res);
    }

   #[test]
    fn sets_union_tests() {
        let mut res: Vec<i32> = eval_set(
            "EF|G|",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -11, 13, 60, 5],
                vec![-11, 3, 100, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-11, -10, 0, 2, 3, 5, 13, 20, 60, 100, 200, 300], res);

        res = eval_set(
            "FG!|E|",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 3, 5, 13, 20, 60], res);

        res = eval_set(
            "FG!|E|!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![100, 200, 300], res);

        res = eval_set(
            "FG|",
            vec![
                vec![],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 3, 5, 60, 100, 200, 300], res);

        res = eval_set(
            "F!G|",
            vec![
                vec![],
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 3, 5, 60, 100, 200, 300], res);

        res = eval_set(
            "FF!|",
            vec![
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 3, 5, 60, 100, 200, 300], res);
        
        res = eval_set(
            "F!F!|",
            vec![
                vec![-10, 3, 100, 5, 200, 300, 60],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);

        res = eval_set(
            "KM|LX||P|",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![2, 100, 60, 5, -10, 13],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 3, 5, 13, 20, 60, 100, 200, 300], res);

        res = eval_set(
            "KM|LX||P|!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![-10, 3, 100, 5, 200, 300, 60],
                vec![2, 100, 60, 5, -10, 13],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);
    }

    #[test]
    fn sets_implication_tests() {
        let mut res: Vec<i32> = eval_set(
            "AB=",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 5], res);
    }
}
