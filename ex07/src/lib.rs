mod truth_table;

use truth_table::TruthTable;

pub fn sat(formula: &str) -> bool {
    let mut truth_table: TruthTable = TruthTable::new(formula);

    truth_table.fill();
    truth_table.eval();

    truth_table.is_sat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }
    
    #[test]
    fn satisfiable_tests() {
        assert!(sat("A!"));
        assert!(sat("A!A^"));
        assert!(sat("AB="));
        assert!(sat("AB=!"));
        assert!(sat("AB>"));
        assert!(sat("AB>!"));
        assert!(sat("AB=A="));
    }

    #[test]
    fn not_satisfiable_tests() {
        assert!(!sat("PQ|P!Q!&&"));
        assert!(!sat("BB!="));
        assert!(!sat("CC!&CC!|="));
        assert!(!sat("DD|D!D!&&"));
        assert!(!sat("EE!="));
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        sat("");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        sat("      ");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test1() {
        sat("AAA!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        sat("A");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        sat("AB");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        sat("AAC|");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        sat("ABCD&=>AB=|CE=F!G=>^^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        sat("!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        sat("!!!!!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        sat("aY=!)>K^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        sat("&&!!Abcd");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        sat("1111&=>11=|11=1!0=>^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test4() {
        sat("111|");
    }
}