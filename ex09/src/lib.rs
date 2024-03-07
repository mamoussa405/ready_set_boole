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
    }
}
