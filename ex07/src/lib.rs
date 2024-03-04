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
}