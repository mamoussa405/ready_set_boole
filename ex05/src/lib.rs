mod ast;

use ast::AST;

pub fn negation_normal_form(formula: &str) -> String {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: AST = AST::new();

    tree.build(formula);
    tree.simplify_material_properties();
    dbg!(tree);
    "".to_string()
}

#[cfg(test)]
mod tests {
}
