mod truth_table;

use truth_table::TruthTable;

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut truth_table: TruthTable = TruthTable::new(formula);

    truth_table.fill();
    truth_table.eval(formula);
    truth_table.get_cnf_formula()
}