mod ast;

pub fn eval_formula(formula: &str) -> bool {
    let mut tree: ast::AST = ast::AST::new();

    tree.build(formula);
    tree.eval()
    // dbg!(tree);
    // true
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn subject_tests() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn logical_and_tests() {
        assert!(!eval_formula("11&1&0&1&"));
    }
}
