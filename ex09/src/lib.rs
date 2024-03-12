mod ast;

use ast::AST;

/// Evaluate sets depending on the formula
/// # Example
/// ```
/// use ex09::eval_set;
/// 
/// eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]);
/// ```
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
    fn sets_equivalence_tests() {
        let mut res: Vec<i32> = eval_set(
            "AB=",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 5], res);

        res = eval_set(
            "AB=!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![3, 13, 20, 60], res);

        res = eval_set(
            "AA!=",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);
    }

    #[test]
    fn sets_implication_tests() {
        let mut res: Vec<i32> = eval_set(
            "AB>",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 5, 13, 60], res);

        res = eval_set(
            "AB>!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![3, 20], res);

        res = eval_set(
            "AA!>",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![] as Vec<i32>, res);
    }

    #[test]
    fn sets_xor_tests() {
        let mut res: Vec<i32> = eval_set(
            "AB^",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![3, 13, 20, 60], res);

        res = eval_set(
            "AB^!",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 5], res);

        res = eval_set(
            "AA!^",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 2, 3, 5, 20], res);
    }

    #[test]
    fn sets_mixed_tests() {
        let mut res: Vec<i32> = eval_set(
            "AB&C|D^",
            vec![
                vec![-10, 3, 20, 2, 0, 5],
                vec![2, 0, -10, 13, 60, 5],
                vec![1, 3],
                vec![],
            ],
        );
        res.sort();
        assert_eq!(vec![-10, 0, 1, 2, 3, 5], res);

        res = eval_set(
            "K!L|F&",
            vec![
                vec![1,-1],
                vec![0, 2],
                vec![100, -100],
            ],
        );
        res.sort();
        assert_eq!(vec![-1, 1], res);

        res = eval_set(
            "XY=U>RT&|",
            vec![
                vec![42, 1337],
                vec![80],
                vec![-55, 0, 1],
                vec![1, 2, -1],
                vec![],
            ],
        );
        res.sort();
        assert_eq!(vec![-55, -1, 0, 1, 2], res);
    }

    #[test]
    #[should_panic(expected = "The number of sets should be equal to the number of vars in the formula")]
    fn number_of_vars_greater_than_number_of_sets() {
        eval_set("AB&C|", vec![
            vec![1, 2, 3],
            vec![1, 2, 3]
        ]);
    }

    #[test]
    #[should_panic(expected = "The number of sets should be equal to the number of vars in the formula")]
    fn number_of_vars_lesser_than_number_of_sets() {
        eval_set("AB&", vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        ]);
    }

    #[test]
    #[should_panic(expected = "Set with index 1 has duplicate elements")]
    fn set_with_duplicate_element() {
        eval_set("AB&", vec![
            vec![1, 2, 3],
            vec![1, 2, 3, 3],
        ]);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        eval_set("", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        eval_set("      ", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test1() {
        eval_set("AAA!", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        eval_set("A", vec![] as Vec<Vec<i32>> );
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        eval_set("AB", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        eval_set("AAC|", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        eval_set("ABCD&=>AB=|CE=F!G=>^^", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        eval_set("!", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        eval_set("!!!!!", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        eval_set("aY=!)>K^", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        eval_set("&&!!Abcd", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        eval_set("1111&=>11=|11=1!0=>^", vec![] as Vec<Vec<i32>>);
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test4() {
        eval_set("111|", vec![] as Vec<Vec<i32>>);
    }
}
