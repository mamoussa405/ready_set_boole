mod truth_table;
mod ast;

use ast::AST;
use truth_table::TruthTable;

pub fn conjunctive_normal_form(formula: &str) -> String {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: AST = AST::new();

    tree.build(formula, true);
    tree.simplify_material_properties();
    let nnf: String = tree.get_rpn_formula();
    if tree.is_valid_cnf() {
        return nnf;
    }
    let mut truth_table: TruthTable = TruthTable::new(&nnf);

    truth_table.fill();
    truth_table.eval();
    truth_table.get_cnf_formula()
}