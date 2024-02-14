mod ast;

pub fn eval_formula(formula: &str) -> bool {
    let mut tree: ast::AST = ast::AST::new();

    tree.build(formula);
    dbg!(tree);
    true
}
